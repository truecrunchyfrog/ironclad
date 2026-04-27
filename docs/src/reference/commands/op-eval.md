# `ic op eval`

Evaluate a single operation by hand.

## Syntax

```bash
ic op eval <operation-id> [--input FILE|-] [--options TOML|-]
```

## Options

- `--input FILE|-`
  A JSON batch of samples. Defaults to an empty batch.
- `--options TOML|-`
  A TOML value passed as operation options.

## Notes

- This command is excellent for prototyping pipelines.
- It can run outside a catalog for operations that do not require catalog-backed files.
- Avoid reading both `--input` and `--options` from stdin in the same invocation.
