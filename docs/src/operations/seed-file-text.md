# `seed.file.text`

Read text from one or more files.

## Options

```toml
files = []
```

- `files`
  list of glob patterns relative to the container directory

## Behavior

- Produces one sample per matched file
- Adds a trace with `path=<relative-path>`
- Fails if a file cannot be read as UTF-8 text

## Example

```toml
[[steps]]
use = "seed.file.text"
options.files = ["config/*.toml"]
```
