//! Live check that `Pager` hands the token from one page to the next.
//!
//! Costs at most three small user reads: one user per page, capped at three pages.
//! Needs `X_USER_TOKEN`, and an `X_USER_ID` that follows at least two accounts.

use x_sdk::apis::users_api;
use x_sdk::config;
use x_sdk::pagination::Pager;

fn env(k: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| panic!("{k} not set"))
}

#[tokio::test]
#[ignore]
async fn following_pages_do_not_repeat() {
    let cfg = config::bearer(env("X_USER_TOKEN"));
    let cfg = &cfg;
    let id = env("X_USER_ID");
    let id = id.as_str();

    let mut pages = Pager::new(|token| async move {
        users_api::get_users_following(cfg, id, Some(1), token.as_deref(), None, None, None).await
    })
    .max_pages(3);

    let mut seen = Vec::new();
    let mut requests = 0;
    while let Some(page) = pages.next_page().await {
        let page = page.expect("request failed");
        requests += 1;
        for user in page.data.unwrap_or_default() {
            assert!(
                !seen.contains(&user.id),
                "user {:?} came back on two pages: the token was not passed on",
                user.id
            );
            seen.push(user.id);
        }
    }

    assert!(
        requests >= 2,
        "only {requests} page(s): X_USER_ID follows too few accounts to test pagination"
    );
    assert_eq!(seen.len(), requests, "max_results=1 should give one user per page");
}
