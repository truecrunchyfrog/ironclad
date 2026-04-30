# Fact File Reference

A fact file is TOML with a small, regular shape.

## Fields

### `description`

Optional human-readable text.

```toml
description = "Watch the currently listed moon phase."
```

### `imports`

A list of export keys this fact depends on.

```toml
imports = ["base_url", "api_token"]
```

### `exports`

A map of export keys to trace-match rules.

```toml
[exports.base_url]
trace_key = "json_node_path"
trace_value = "$['base_url']"
```

Ironclad finds the sample whose trace contains that exact key/value pair and exports it.

### `steps`

An ordered array of operations.

You can write `steps` in two different TOML styles.

Array-of-tables style:

```toml
[[steps]]
use = "seed.file.text"
options.files = ["status.txt"]
```

Array-value style:

```toml
steps = [
  { use = "seed.file.text", options = { files = ["status.txt"] } },
  "text.trim",
  "compact",
]
```

In array-value style, a step can be either:
- a string, which becomes the operation ID with empty options
- an inline table, which specifies the full step

The string shorthand is useful for optionless steps such as `"text.trim"` or
`"compact"`.

You cannot mix the two TOML styles for one `steps` field. In particular:
- `[[steps]]` always creates an array of tables
- string shorthand only works inside `steps = [ ... ]`
- TOML does not support array indexing syntax such as `steps.1 = "text.trim"`

### `secret`

Marks the fact as sensitive.

```toml
secret = true
```

When `secret = true`, a normal `resolve` run redacts the sample contents before
writing the resolution snapshot.

The snapshot still records enough information to detect drift, but it does not
store the original secret value in plaintext.

If you need the raw values for one run, `ic resolve --no-redact` disables
redaction for that invocation.

## Full example

```toml
description = "Track all creature names announced by the observatory."
secret = false

steps = [
  { use = "seed.file.text", options = { files = ["observatory-board.txt"] } },
  "text.lines",
  "text.trim",
  "compact",
]
```

## Notes

- Unknown operation options are rejected by most operations through `deny_unknown_fields`.
- Import interpolation only happens for exact strings like `$(key)`.
