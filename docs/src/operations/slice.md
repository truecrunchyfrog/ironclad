# `slice`

Take a slice of the current batch.

## Options

```toml
drop = 0
take = 0
```

- `drop`
  number of samples to skip from the start
- `take`
  number of samples to keep after dropping

If `take` is omitted, Ironclad keeps the rest.
