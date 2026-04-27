# `seed.net.http`

Fetch a URL with HTTP GET.

## Options

```toml
url = ""
user_agent = "Mozilla/5.0 ..."
```

- `url`
  request target
- `user_agent`
  HTTP user agent string

## Behavior

- Produces one sample containing the response body
- Fails on HTTP error status codes
