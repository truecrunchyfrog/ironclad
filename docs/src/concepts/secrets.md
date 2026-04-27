# Secrets

Facts can be marked as secret:

```toml
secret = true
```

When a secret fact is resolved normally, Ironclad redacts the sample contents before writing them to snapshots.

Instead of storing the original content, it stores a digest marker.

That means:
- you can still detect drift
- you do not leak the secret value into the snapshot file

If you really need the unredacted values during a run, `ic resolve --no-redact` disables redaction for that invocation.

Use that flag carefully.
