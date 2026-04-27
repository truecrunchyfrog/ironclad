# Snapshot Format Reference

Snapshots are JSON objects keyed by fact label.

## Shape

```json
{
  "fact-label": {
    "samples": [
      {
        "traces": [{ "path": "file.txt" }],
        "content": "hello"
      }
    ],
    "created": "..."
  }
}
```

## Batch fields

- `samples`
  ordered list of samples
- `created`
  timestamp for when the batch was created

## Sample fields

- `traces`
  ordered list of trace objects
- `content`
  string content being tracked

## Stability notes

- Snapshot files are a practical storage format, not a public network protocol.
- They are suitable for `inspect`, `diff`, `check`, `apply`, and `op eval` experiments.
