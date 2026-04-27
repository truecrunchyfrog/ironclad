# Recipes

## Clean a line-oriented text file

```toml
[[steps]]
use = "seed.file.text"
options.files = ["guest-list.txt"]

[[steps]]
use = "text.lines"

[[steps]]
use = "text.trim"

[[steps]]
use = "compact"
```

## Scrape one HTML fragment

```toml
[[steps]]
use = "seed.net.http"
options.url = "https://example.com"

[[steps]]
use = "html.find"
options.selector = "main .headline"
options.document = true

[[steps]]
use = "html.inner.text"
```

## Run a classic Unix filter per sample

```toml
[[steps]]
use = "text.lines"

[[steps]]
use = "run"
options.program = "rev"
```

## Keep only interesting JSON values

```toml
[[steps]]
use = "seed.file.text"
options.files = ["status.json"]

[[steps]]
use = "json.find"
options.path = "$.checks[*].name"
```

## Trim then remove empties

This is a common cleanup pattern:

```toml
[[steps]]
use = "text.trim"

[[steps]]
use = "compact"
```
