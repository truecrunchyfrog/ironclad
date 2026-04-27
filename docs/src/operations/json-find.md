# `json.find`

Find values in JSON using a JSONPath expression.

## Options

```toml
path = "$"
```

- `path`
  a JSONPath expression

## Behavior

- Produces one sample per matched value
- Adds a `json_node_path` trace entry
- Strings stay strings; other JSON values are serialized back to text
