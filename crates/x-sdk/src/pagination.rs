//! Walking paginated endpoints.
//!
//! ```no_run
//! use x_sdk::apis::users_api;
//! use x_sdk::pagination::Pager;
//!
//! # async fn run(cfg: x_sdk::apis::configuration::Configuration) -> Result<(), Box<dyn std::error::Error>> {
//! let cfg = &cfg;
//! let mut pages = Pager::new(|token| async move {
//!     users_api::get_users_me_following(
//!         cfg,
//!         users_api::GetUsersMeFollowingParams {
//!             max_results: Some(1000),
//!             pagination_token: token,
//!             ..Default::default()
//!         },
//!     )
//!     .await
//! });
//! while let Some(page) = pages.next_page().await {
//!     let page = page?;
//!     println!("{:?}", page.data);
//! }
//! # Ok(()) }
//! ```

use std::future::Future;

/// A response that may be followed by another page.
pub trait Paginated {
    /// The token to request the next page with, if there is one.
    fn next_token(&self) -> Option<&str>;
}

/// Fetches pages one at a time. `fetch` gets the pagination token to request (`None` for the
/// first page) and calls the endpoint.
///
/// Capture shared state such as the `Configuration` by reference: `fetch` runs once per page.
pub struct Pager<F> {
    fetch: F,
    token: Option<String>,
    done: bool,
    remaining: Option<usize>,
}

impl<F> Pager<F> {
    /// Resume from a token saved from an earlier run instead of starting at the first page.
    pub fn start_at(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Stop after `pages` requests.
    pub fn max_pages(mut self, pages: usize) -> Self {
        self.remaining = Some(pages);
        self
    }

    /// The token the next call to `next_page` will request with, if any. Save it to resume
    /// later with `start_at`.
    pub fn next_token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}

impl<F, Fut, T, E> Pager<F>
where
    F: FnMut(Option<String>) -> Fut,
    Fut: Future<Output = Result<T, E>>,
    T: Paginated,
{
    // The bounds live here so the closure's token type is inferred at the call site.
    pub fn new(fetch: F) -> Self {
        Self {
            fetch,
            token: None,
            done: false,
            remaining: None,
        }
    }

    /// The next page, or `None` once the last one has been returned. An error ends the
    /// iteration; the token to retry from is still available from `next_token`.
    pub async fn next_page(&mut self) -> Option<Result<T, E>> {
        if self.done || self.remaining == Some(0) {
            return None;
        }
        if let Some(n) = self.remaining.as_mut() {
            *n -= 1;
        }
        match (self.fetch)(self.token.clone()).await {
            Ok(page) => {
                self.token = page.next_token().map(str::to_owned);
                self.done = self.token.is_none();
                Some(Ok(page))
            }
            Err(e) => {
                self.done = true;
                Some(Err(e))
            }
        }
    }
}

#[cfg(feature = "stream")]
impl<F, Fut, T, E> Pager<F>
where
    F: FnMut(Option<String>) -> Fut,
    Fut: Future<Output = Result<T, E>>,
    T: Paginated,
{
    /// The pages as a `Stream`. The stream ends after the last page, or after the first error.
    pub fn into_stream(self) -> impl futures_util::Stream<Item = Result<T, E>> {
        futures_util::stream::unfold(self, |mut pager| async move {
            let page = pager.next_page().await?;
            Some((page, pager))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Page(Option<&'static str>);

    impl Paginated for Page {
        fn next_token(&self) -> Option<&str> {
            self.0
        }
    }

    // "a" -> page 1, "b" -> page 2, anything else (the first request) -> page 0.
    async fn fetch(token: Option<String>) -> Result<Page, &'static str> {
        match token.as_deref() {
            None => Ok(Page(Some("a"))),
            Some("a") => Ok(Page(Some("b"))),
            Some("b") => Ok(Page(None)),
            Some("bad") => Err("boom"),
            Some(_) => unreachable!(),
        }
    }

    async fn count<F, Fut>(mut pager: Pager<F>) -> usize
    where
        F: FnMut(Option<String>) -> Fut,
        Fut: Future<Output = Result<Page, &'static str>>,
    {
        let mut n = 0;
        while let Some(Ok(_)) = pager.next_page().await {
            n += 1;
        }
        n
    }

    #[tokio::test]
    async fn walks_until_no_token() {
        assert_eq!(count(Pager::new(fetch)).await, 3);
    }

    #[tokio::test]
    async fn max_pages_stops_early() {
        assert_eq!(count(Pager::new(fetch).max_pages(2)).await, 2);
    }

    #[tokio::test]
    async fn start_at_resumes() {
        assert_eq!(count(Pager::new(fetch).start_at("a")).await, 2);
    }

    #[tokio::test]
    async fn error_ends_iteration_and_keeps_token() {
        let mut pager = Pager::new(fetch).start_at("bad");
        assert_eq!(pager.next_page().await.unwrap().err(), Some("boom"));
        assert!(pager.next_page().await.is_none());
        assert_eq!(pager.next_token(), Some("bad"));
    }
}
