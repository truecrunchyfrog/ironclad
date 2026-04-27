# `html.find`

Find HTML elements by CSS selector.

## Options

```toml
selector = ""
document = false
```

- `selector`
  CSS selector
- `document`
  parse as a full HTML document instead of a fragment

## Behavior

- Produces one sample per matching node
- Adds a `node_id` trace entry
