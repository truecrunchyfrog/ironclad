# `text.split`

Split each sample into multiple samples.

## Options

The operation supports several modes:

```toml
at_index = 0
```

```toml
on_text.text = ""
on_text.max = 0
```

```toml
on_text_inclusive.text = ""
```

## Behavior

- `at_index`
  split at a byte index when valid
- `on_text`
  split on a delimiter, optionally with a limit
- `on_text_inclusive`
  keep the delimiter attached to each piece
