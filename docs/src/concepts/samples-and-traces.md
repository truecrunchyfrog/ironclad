# Samples and Traces

A sample is the atomic piece of observed state in Ironclad.

Each sample has:
- `content`
- one or more `traces`

## Content

The content is the string that operations transform and that snapshots ultimately compare.

It might be:
- one line from a file
- one HTML node
- one JSON value
- one command output

## Traces

Traces are small key-value maps that explain provenance.

Examples:

```json
{ "path": "menu.txt" }
```

```json
{ "json_node_path": "$['dessert']" }
```

```json
{ "start": "10", "end": "24" }
```

Each time an operation evolves a sample, it usually appends a new trace.

That means a single sample can tell a small story:

1. it came from `menu.txt`
2. then it was split into lines
3. then one regex extracted a piece of it

When `inspect --trace` or `diff --trace` is useful, it is usually because that story matters as much as the final content.
