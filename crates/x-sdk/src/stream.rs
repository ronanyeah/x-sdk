//! Streaming endpoints: newline-delimited JSON that stays open.
//!
//! [`connect`] opens an endpoint from its params and yields the endpoint's own response type,
//! reconnecting when the connection drops.
//!
//! The generated stream functions return the raw `reqwest::Response`. To drive one yourself,
//! [`ndjson`] decodes it and [`reconnecting`] keeps a stream going across disconnects.

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use futures_util::stream;
pub use futures_util::{Stream, StreamExt};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use x_sdk_gen::apis::{self, configuration::Configuration};

#[derive(Debug)]
pub enum Error {
    /// The connection dropped or the body could not be read.
    Http(reqwest::Error),
    /// A line was not the expected JSON. The stream carries on with the next line.
    Json {
        source: serde_json::Error,
        line: String,
    },
    /// Connecting failed with an error that retrying will not fix (bad credentials, bad
    /// parameters, ...). The stream ends after yielding it.
    Connect(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(e) => write!(f, "stream read failed: {e}"),
            Error::Json { source, line } => write!(f, "invalid stream line ({source}): {line}"),
            Error::Connect(e) => write!(f, "could not connect: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Http(e) => Some(e),
            Error::Json { source, .. } => Some(source),
            Error::Connect(_) => None,
        }
    }
}

/// Decode a streaming response, one `T` per line. X sends blank keepalive lines, which are
/// skipped. A read error is yielded once and then the stream ends.
pub fn ndjson<T: DeserializeOwned>(
    response: reqwest::Response,
) -> impl Stream<Item = Result<T, Error>> {
    let body = Box::pin(response.bytes_stream());
    stream::unfold((body, Vec::<u8>::new(), false), |(mut body, mut buf, mut done)| async move {
        loop {
            if let Some(end) = buf.iter().position(|&b| b == b'\n') {
                let line: Vec<u8> = buf.drain(..=end).collect();
                if let Some(item) = parse(&line) {
                    return Some((item, (body, buf, done)));
                }
                continue;
            }
            if done {
                // A last line the server never terminated.
                let line = std::mem::take(&mut buf);
                return parse(&line).map(|item| (item, (body, buf, done)));
            }
            match body.next().await {
                Some(Ok(chunk)) => buf.extend_from_slice(&chunk),
                Some(Err(e)) => {
                    buf.clear();
                    done = true;
                    return Some((Err(Error::Http(e)), (body, buf, done)));
                }
                None => done = true,
            }
        }
    })
}

fn parse<T: DeserializeOwned>(line: &[u8]) -> Option<Result<T, Error>> {
    let line = line.trim_ascii();
    if line.is_empty() {
        return None;
    }
    Some(serde_json::from_slice(line).map_err(|source| Error::Json {
        source,
        line: String::from_utf8_lossy(line).into_owned(),
    }))
}

/// The params of a streaming endpoint, which know what the endpoint yields. Implemented for
/// every generated streaming `*Params` struct; use [`connect`].
#[allow(async_fn_in_trait)]
pub trait StreamEndpoint: Clone {
    /// One decoded line.
    type Item: DeserializeOwned + Send + 'static;
    type Error;

    /// Call the generated stream function.
    async fn open(
        self,
        cfg: &Configuration,
    ) -> Result<reqwest::Response, apis::Error<Self::Error>>;
}

/// Open a streaming endpoint and keep it open: decoded as the endpoint's own response type and
/// reconnected as described on [`reconnecting`].
///
/// ```no_run
/// use x_sdk::apis::stream_api::StreamPostsSampleParams;
/// use x_sdk::stream::{self, StreamExt};
///
/// # async fn run(cfg: x_sdk::Config) {
/// let mut posts = std::pin::pin!(stream::connect(&cfg, StreamPostsSampleParams::default()));
/// while let Some(post) = posts.next().await {
///     println!("{:?}", post.unwrap().data);
/// }
/// # }
/// ```
pub fn connect<'a, P: StreamEndpoint + 'a>(
    cfg: &'a Configuration,
    params: P,
) -> impl Stream<Item = Result<P::Item, Error>> + 'a {
    reconnecting(move || params.clone().open(cfg))
}

type Inner<T> = Pin<Box<dyn Stream<Item = Result<T, Error>> + Send>>;

/// Connect, decode, and reconnect whenever the stream drops, following X's backoff guidance:
/// network errors back off linearly (250 ms steps, up to 16 s), HTTP errors exponentially
/// (from 5 s, up to 320 s), and rate limiting (429) exponentially from 1 minute. Backoff resets
/// once an item arrives.
///
/// `connect` is called for every attempt and calls the generated stream function. Errors that
/// retrying will not fix end the stream after yielding [`Error::Connect`]. Bad lines are
/// yielded as [`Error::Json`] and the stream continues.
///
/// ```no_run
/// use x_sdk::apis::stream_api::{self, StreamPostsSampleParams};
/// use x_sdk::models::StreamPostsSampleResponse;
/// use x_sdk::stream::{self, StreamExt};
///
/// # async fn run(cfg: x_sdk::apis::configuration::Configuration) {
/// let cfg = &cfg;
/// let mut posts = std::pin::pin!(stream::reconnecting::<StreamPostsSampleResponse, _, _, _>(
///     || stream_api::stream_posts_sample(cfg, StreamPostsSampleParams::default()),
/// ));
/// while let Some(post) = posts.next().await { /* ... */ }
/// # }
/// ```
pub fn reconnecting<T, E, F, Fut>(connect: F) -> impl Stream<Item = Result<T, Error>>
where
    T: DeserializeOwned + Send + 'static,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<reqwest::Response, apis::Error<E>>>,
{
    let state = (connect, None::<Inner<T>>, Backoff::default(), false);
    stream::unfold(state, |(mut connect, mut inner, mut backoff, finished)| async move {
        if finished {
            return None;
        }
        loop {
            if let Some(current) = inner.as_mut() {
                match current.next().await {
                    Some(Ok(item)) => {
                        backoff = Backoff::default();
                        return Some((Ok(item), (connect, inner, backoff, false)));
                    }
                    Some(Err(e @ Error::Json { .. })) => {
                        return Some((Err(e), (connect, inner, backoff, false)));
                    }
                    // Dropped, or ended cleanly: either way, reconnect.
                    Some(Err(_)) | None => {
                        inner = None;
                        tokio::time::sleep(backoff.network()).await;
                        continue;
                    }
                }
            }
            match connect().await {
                Ok(response) => inner = Some(Box::pin(ndjson(response))),
                Err(apis::Error::Reqwest(_)) => tokio::time::sleep(backoff.network()).await,
                Err(apis::Error::ResponseError(r)) if r.status == StatusCode::TOO_MANY_REQUESTS => {
                    tokio::time::sleep(backoff.rate_limited()).await
                }
                Err(apis::Error::ResponseError(r)) if r.status.is_server_error() => {
                    tokio::time::sleep(backoff.http()).await
                }
                Err(e) => {
                    return Some((Err(Error::Connect(e.to_string())), (connect, inner, backoff, true)));
                }
            }
        }
    })
}

#[derive(Default)]
struct Backoff {
    network: u32,
    http: u32,
    rate_limited: u32,
}

impl Backoff {
    fn network(&mut self) -> Duration {
        self.network += 1;
        Duration::from_millis(250 * u64::from(self.network)).min(Duration::from_secs(16))
    }

    fn http(&mut self) -> Duration {
        let wait = exponential(Duration::from_secs(5), self.http);
        self.http += 1;
        wait
    }

    fn rate_limited(&mut self) -> Duration {
        let wait = exponential(Duration::from_secs(60), self.rate_limited);
        self.rate_limited += 1;
        wait
    }
}

fn exponential(start: Duration, attempt: u32) -> Duration {
    start
        .saturating_mul(2u32.saturating_pow(attempt))
        .min(Duration::from_secs(320))
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct N {
        n: u32,
    }

    fn response(chunks: &[&'static [u8]]) -> reqwest::Response {
        let chunks = chunks
            .iter()
            .map(|c| Ok::<_, std::io::Error>(c.to_vec()))
            .collect::<Vec<_>>();
        let body = reqwest::Body::wrap_stream(stream::iter(chunks));
        reqwest::Response::from(http::Response::new(body))
    }

    #[tokio::test]
    async fn splits_lines_across_chunks_and_skips_keepalives() {
        let r = response(&[b"{\"n\":1}\r\n\r\n{\"n\"", b":2}\n", b"oops\n", b"{\"n\":3}"]);
        let items: Vec<_> = ndjson::<N>(r).collect().await;

        assert_eq!(items.len(), 4);
        assert_eq!(items[0].as_ref().unwrap(), &N { n: 1 });
        assert_eq!(items[1].as_ref().unwrap(), &N { n: 2 });
        assert!(matches!(&items[2], Err(Error::Json { line, .. }) if line == "oops"));
        assert_eq!(items[3].as_ref().unwrap(), &N { n: 3 });
    }

    #[test]
    fn backoff_follows_the_guidance() {
        let mut b = Backoff::default();
        assert_eq!(b.network(), Duration::from_millis(250));
        assert_eq!(b.network(), Duration::from_millis(500));
        assert_eq!(b.http(), Duration::from_secs(5));
        assert_eq!(b.http(), Duration::from_secs(10));
        assert_eq!(b.rate_limited(), Duration::from_secs(60));
        assert_eq!(b.rate_limited(), Duration::from_secs(120));

        let mut b = Backoff::default();
        for _ in 0..100 {
            b.network();
            b.http();
            b.rate_limited();
        }
        assert_eq!(b.network(), Duration::from_secs(16));
        assert_eq!(b.http(), Duration::from_secs(320));
        assert_eq!(b.rate_limited(), Duration::from_secs(320));
    }
}
