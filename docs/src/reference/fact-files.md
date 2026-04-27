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

```toml
[[steps]]
use = "seed.file.text"
options.files = ["status.txt"]
```

### `secret`

Marks the fact as sensitive.

```toml
secret = true
```

## Full example

```toml
description = "Track all creature names announced by the observatory."
secret = false

[[steps]]
use = "seed.file.text"
options.files = ["observatory-board.txt"]

[[steps]]
use = "text.lines"

[[steps]]
use = "text.trim"

[[steps]]
use = "compact"
```

## Notes

- Unknown operation options are rejected by most operations through `deny_unknown_fields`.
- Import interpolation only happens for exact strings like `$(key)`.
