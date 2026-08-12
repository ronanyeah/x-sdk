# X serves the `security` scope arrays from an unordered set
fetch-schema:
    curl -s "https://api.x.com/2/openapi.json" \
      | jq 'walk(if type == "object" and has("security") then .security |= map(map_values(sort)) else . end)' \
      > openapi.json
