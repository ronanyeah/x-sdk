# X serves the `security` scope arrays from an unordered set
fetch-schema:
    curl -s "https://api.x.com/2/openapi.json" \
      | jq 'walk(if type == "object" and has("security") then .security |= map(map_values(sort)) else . end)' \
      > openapi.json

# OAuth2 scopes in openapi.json -> crates/x-sdk/src/scope.rs
gen-scopes:
    jq -r -f scripts/gen-scope.jq openapi.json > crates/x-sdk/src/scope.rs
    rustfmt --edition 2024 crates/x-sdk/src/scope.rs
