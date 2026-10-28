//! Checks that each patch in `patches/` is still needed and still applied.
//!
//! Each test compares three things:
//! - the wire: raw JSON from X, read with reqwest and untouched by any SDK type
//! - a strict mirror: a local type that follows the unmodified spec, which must fail
//!   (or come up empty) on that JSON
//! - the SDK: the generated, patched type, which must decode it
//!
//! If the wire or the strict mirror no longer shows the problem, the patch can be
//! dropped. If the SDK check fails, a regenerate lost the patch. Failure messages name
//! the patch.
//!
//! The `#[ignore]` tests are live and need `X_USER_TOKEN` and `X_USER_ID`.

use serde::Deserialize;
use serde_json::Value;
use x_sdk_gen::apis::{configuration::Configuration, users_api};
use x_sdk_gen::models::GetUsersFollowingResponse;

type Following =
    Result<GetUsersFollowingResponse, x_sdk_gen::apis::Error<users_api::GetUsersFollowingError>>;

fn env(k: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| panic!("{k} not set"))
}

fn cfg() -> Configuration {
    let mut c = Configuration::new();
    c.oauth_access_token = Some(env("X_USER_TOKEN"));
    c
}

/// Builds an optional query param as whatever type the generated call takes
/// (`Vec<String>` or a field enum); empty means not set.
fn opt<T: serde::de::DeserializeOwned>(v: &[&str]) -> Option<Vec<T>> {
    (!v.is_empty()).then(|| serde_json::from_value(serde_json::json!(v)).unwrap())
}

/// GET /2/users/{id}/following as raw JSON.
async fn raw_following(query: &str) -> Value {
    let url = format!(
        "https://api.x.com/2/users/{}/following?max_results=100&{query}",
        env("X_USER_ID")
    );
    reqwest::Client::new()
        .get(url)
        .bearer_auth(env("X_USER_TOKEN"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap()
}

/// GET /2/users/{id}/following through the SDK.
async fn sdk_following(
    user_fields: &[&str],
    expansions: &[&str],
    post_fields: &[&str],
) -> Following {
    users_api::get_users_following(
        &cfg(),
        &env("X_USER_ID"),
        Some(100),
        None,
        opt(user_fields),
        opt(expansions),
        opt(post_fields),
    )
    .await
}

// patch 02: public_metrics.tweet_count (API) vs post_count (spec, required)

/// `UserPublicMetrics` as the unmodified spec defines it.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct StrictUserMetrics {
    followers_count: i64,
    following_count: i64,
    listed_count: i64,
    post_count: i64,
}

#[tokio::test]
#[ignore]
async fn tweet_count_patch_still_required() {
    let raw = raw_following("user.fields=public_metrics").await;
    let metrics: Vec<&Value> = raw["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|u| &u["public_metrics"])
        .collect();
    assert!(
        metrics.iter().any(|m| m.get("tweet_count").is_some()),
        "API stopped sending tweet_count: drop patch 02"
    );
    assert!(
        metrics.iter().all(|m| m.get("post_count").is_none()),
        "API now sends post_count: drop patch 02"
    );

    let strict = serde_json::from_value::<StrictUserMetrics>(metrics[0].clone());
    assert!(
        strict.is_err(),
        "spec-faithful type decodes the live shape: drop patch 02 ({strict:?})"
    );

    let r = sdk_following(&["public_metrics"], &[], &[])
        .await
        .expect("patch 02 was lost");
    let read = r
        .data
        .unwrap()
        .into_iter()
        .filter_map(|u| u.public_metrics)
        .any(|m| m.tweet_count.is_some());
    assert!(read, "SDK did not read tweet_count: patch 02 was lost");
}

// patch 03: Problem is an internally tagged enum whose variants also require `type`

/// A real partial error from /following.
const PROBLEM: &str = r#"{"detail":"Sorry, you are not authorized to see the post with most_recent_tweet_id: [2113062452774515013].","parameter":"most_recent_tweet_id","resource_id":"2113062452774515013","resource_type":"tweet","section":"includes","title":"Authorization Error","type":"https://api.x.com/2/problems/not-authorized-for-resource","value":"2113062452774515013"}"#;

/// `Problem` as the unmodified spec generates it: a tagged enum whose variant also
/// requires `type`.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
enum StrictProblem {
    #[serde(rename = "https://api.x.com/2/problems/not-authorized-for-resource")]
    NotAuthorized(StrictProblemVariant),
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct StrictProblemVariant {
    detail: String,
    title: String,
    r#type: String,
}

#[test]
fn problem_patch_still_required() {
    let strict = serde_json::from_str::<StrictProblem>(PROBLEM);
    assert!(
        strict.is_err(),
        "spec-faithful Problem decodes: drop patch 03 ({strict:?})"
    );

    let p: x_sdk_gen::models::Problem = serde_json::from_str(PROBLEM).expect("patch 03 was lost");
    let out = serde_json::to_value(&p).unwrap();
    assert_eq!(
        out["type"],
        "https://api.x.com/2/problems/not-authorized-for-resource"
    );
}

// patch 04: most_recent_tweet_id and includes.tweets (API) vs most_recent_post_id and
// includes.posts (spec). Serde ignores unknown keys, so a strict type is silently empty
// rather than an error.

#[derive(Debug, Deserialize)]
struct StrictUser {
    most_recent_post_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StrictIncludes {
    posts: Option<Vec<Value>>,
}

#[tokio::test]
#[ignore]
async fn most_recent_tweet_id_patch_still_required() {
    let raw = raw_following("expansions=most_recent_post_id").await;
    let users = raw["data"].as_array().unwrap();
    assert!(
        users
            .iter()
            .any(|u| u.get("most_recent_tweet_id").is_some()),
        "API stopped sending most_recent_tweet_id: drop patch 04"
    );
    assert!(
        users.iter().all(|u| u.get("most_recent_post_id").is_none()),
        "API now sends most_recent_post_id: drop patch 04"
    );

    let filled = users
        .iter()
        .filter(|u| {
            serde_json::from_value::<StrictUser>((*u).clone())
                .unwrap()
                .most_recent_post_id
                .is_some()
        })
        .count();
    assert_eq!(
        filled, 0,
        "spec-faithful type fills most_recent_post_id: drop patch 04"
    );

    let r = sdk_following(&[], &["most_recent_post_id"], &[])
        .await
        .unwrap();
    assert!(
        r.data
            .unwrap()
            .iter()
            .any(|u| u.most_recent_post_id.is_some()),
        "SDK did not read most_recent_tweet_id: patch 04 was lost"
    );
}

#[tokio::test]
#[ignore]
async fn includes_tweets_patch_still_required() {
    let raw = raw_following("expansions=most_recent_post_id").await;
    assert!(
        raw["includes"].get("tweets").is_some(),
        "API stopped sending includes.tweets: drop patch 04"
    );
    assert!(
        raw["includes"].get("posts").is_none(),
        "API now sends includes.posts: drop patch 04"
    );

    let strict: StrictIncludes = serde_json::from_value(raw["includes"].clone()).unwrap();
    assert!(
        strict.posts.is_none(),
        "spec-faithful type fills includes.posts: drop patch 04"
    );

    let r = sdk_following(&[], &["most_recent_post_id"], &[])
        .await
        .unwrap();
    assert!(
        r.includes.and_then(|i| i.posts).is_some(),
        "SDK did not read includes.tweets: patch 04 was lost"
    );
}

// patch 05: PostPublicMetrics.retweet_count (API) vs repost_count (spec, required)

/// `PostPublicMetrics` as the unmodified spec defines it.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct StrictPostMetrics {
    bookmark_count: i64,
    impression_count: i64,
    like_count: i64,
    quote_count: i64,
    reply_count: i64,
    repost_count: i64,
}

#[tokio::test]
#[ignore]
async fn retweet_count_patch_still_required() {
    let raw = raw_following("expansions=most_recent_post_id&tweet.fields=public_metrics").await;
    let metrics: Vec<&Value> = raw["includes"]["tweets"]
        .as_array()
        .expect("no includes.tweets")
        .iter()
        .map(|t| &t["public_metrics"])
        .collect();
    assert!(
        metrics.iter().any(|m| m.get("retweet_count").is_some()),
        "API stopped sending retweet_count: drop patch 05"
    );
    assert!(
        metrics.iter().all(|m| m.get("repost_count").is_none()),
        "API now sends repost_count: drop patch 05"
    );

    let strict = serde_json::from_value::<StrictPostMetrics>(metrics[0].clone());
    assert!(
        strict.is_err(),
        "spec-faithful type decodes the live shape: drop patch 05 ({strict:?})"
    );

    let r = sdk_following(&[], &["most_recent_post_id"], &["public_metrics"])
        .await
        .expect("patch 05 was lost");
    let posts = r.includes.and_then(|i| i.posts).expect("no includes.posts");
    // Serialized rather than field-accessed so this doesn't depend on the field's type.
    let kept = posts
        .iter()
        .filter_map(|p| serde_json::to_value(p.public_metrics.as_ref()?).ok())
        .any(|m| m["repost_count"].is_number());
    assert!(kept, "SDK dropped retweet_count: patch 05 was lost");
}

// templates/reqwest/api.mustache: chrono's Display (`2026-10-01 00:00:00 +00:00`) is not
// RFC3339, and the generator uses it for date-time query params. X rejects it.

fn instant() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::DateTime::parse_from_rfc3339("2026-10-01T00:00:00+00:00").unwrap()
}

/// Every `chrono::DateTime` parameter of a function in src/apis, as (file, name).
fn datetime_params() -> Vec<(String, String)> {
    let mut out = vec![];
    for entry in std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/apis")).unwrap() {
        let path = entry.unwrap().path();
        let src = std::fs::read_to_string(&path).unwrap();
        for sig in src.lines().filter(|l| l.starts_with("pub async fn")) {
            for arg in sig.split(", ").filter(|a| a.contains("chrono::DateTime")) {
                let name = arg
                    .split(':')
                    .next()
                    .unwrap()
                    .trim()
                    .trim_start_matches('(');
                out.push((path.display().to_string(), name.to_string()));
            }
        }
    }
    out
}

#[test]
fn datetime_patch_still_required_and_applied() {
    let display = instant().to_string();
    assert!(
        chrono::DateTime::parse_from_rfc3339(&display).is_err(),
        "chrono Display is RFC3339 now ({display}): drop the datetime template patch"
    );

    let params = datetime_params();
    assert!(
        !params.is_empty(),
        "no date-time params found: test is stale"
    );
    for (file, name) in params {
        let src = std::fs::read_to_string(&file).unwrap();
        let query = format!("(\"{name}\", &");
        let lines: Vec<_> = src.lines().filter(|l| l.contains(&query)).collect();
        assert!(!lines.is_empty(), "no query line for {name} in {file}");
        for l in lines {
            assert!(
                l.contains("to_rfc3339()"),
                "{file}: `{name}` is not sent as RFC3339: {}",
                l.trim()
            );
        }
    }
}

#[tokio::test]
#[ignore]
async fn datetime_patch_still_required_live() {
    let status = |t: String| async move {
        let url = format!(
            "https://api.x.com/2/users/{}/tweets?max_results=5&start_time={}",
            env("X_USER_ID"),
            t.replace('+', "%2B").replace(' ', "%20")
        );
        reqwest::Client::new()
            .get(url)
            .bearer_auth(env("X_USER_TOKEN"))
            .send()
            .await
            .unwrap()
            .status()
    };

    let bad = status(instant().to_string()).await;
    assert_eq!(
        bad.as_u16(),
        400,
        "X now accepts the non-RFC3339 form ({bad}): drop the datetime template patch"
    );
    let good = status(instant().to_rfc3339()).await;
    assert!(good.is_success(), "X rejected RFC3339 ({good})");

    let r = users_api::get_users_posts(
        &cfg(),
        &env("X_USER_ID"),
        Some(5),
        None,
        Some(instant()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;
    assert!(r.is_ok(), "SDK call with start_time failed: {r:?}");
}
