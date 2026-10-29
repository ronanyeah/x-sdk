# x-sdk

Rust client for the X API v2 (`api.x.com`), generated from X's OpenAPI spec, with hand-written helpers for OAuth2, pagination and streaming.

## Install

```toml
[dependencies]
x-sdk = { git = "https://github.com/ronanyeah/x-sdk" }
```

| Feature      | Default | Effect                                                              |
| ------------ | ------- | ------------------------------------------------------------------- |
| `rustls`     | yes     | TLS via rustls                                                      |
| `native-tls` | no      | TLS via the system library (use with `default-features = false`)   |
| `local`      | no      | `OAuth2Client::authorize_local`, a localhost listener for CLI login |
| `stream`     | no      | the `stream` module, and `into_stream` on pagers                    |

## Calling the API

Every endpoint is a function in `x_sdk::apis::*_api` that takes a config and one params struct. Request and response types are in `x_sdk::models`.

```rust
use x_sdk::{Config, apis::users_api};

// An app-only bearer token or a user access token.
let cfg = Config::bearer(token);

let me = users_api::get_users_me(&cfg, Default::default()).await?;
```

## Getting a user token

Endpoints that act as a user need an OAuth2 user token. `x_sdk::auth` implements the authorization code flow with PKCE. In the X developer portal, create an app and register your redirect URI as a callback URL. A Web App is a confidential client; a Native App is public (omit `with_secret`).

```rust
use x_sdk::auth::{OAuth2Client, Scope};

let client = OAuth2Client::new(client_id).with_secret(client_secret);

// Send the user to request.url. Keep `request` until they return.
let request = client.authorization_url(redirect_uri, &[Scope::TweetRead, Scope::OfflineAccess])?;

// On the callback: checks `state`, surfaces `error=`, returns the code.
let code = request.code_from_callback(&callback_url)?;
let tokens = client.exchange_code(&code, redirect_uri, &request.code_verifier).await?;

let cfg = tokens.configuration();
```

For a CLI, the `local` feature's `authorize_local` listens on a loopback redirect, hands you the URL to open, and exchanges the code:

```rust
let tokens = client
    .authorize_local("http://localhost:8000/", &[Scope::TweetRead], |url| {
        println!("Open this URL and approve the app:\n\n{url}");
    })
    .await?;
```

Access tokens expire. Request `Scope::OfflineAccess` to get a refresh token, then `client.refresh(&refresh_token).await?`. Refresh tokens are single use: persist the replacement in `tokens.refresh_token` every time.

## Pagination

`Pager` follows `next_token` for you. Wrap the call in a closure, then walk pages or items.

```rust
use x_sdk::pagination::Pager;

let cfg = &cfg;
let mut items = Pager::new(|token| async move {
    users_api::get_users_me_following(
        cfg,
        users_api::GetUsersMeFollowingParams {
            max_results: Some(1000),
            pagination_token: token,
            ..Default::default()
        },
    )
    .await
})
.items();

while let Some(user) = items.next_item().await {
    println!("{:?}", user?.username);
}
```

Use `next_page` for whole pages, `max_pages` to cap requests, and `start_at` with `next_token` to resume.

## Streaming

With the `stream` feature, `stream::connect` opens a streaming endpoint, decodes each line as that endpoint's response type, and reconnects with X's recommended backoff.

```rust
use x_sdk::apis::stream_api::StreamPostsSampleParams;
use x_sdk::stream::{self, StreamExt};

let mut posts = std::pin::pin!(stream::connect(&cfg, StreamPostsSampleParams::default()));
while let Some(post) = posts.next().await {
    println!("{:?}", post?.data);
}
```
