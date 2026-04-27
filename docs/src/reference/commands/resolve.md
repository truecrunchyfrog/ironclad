# `ic resolve`

Resolve facts into a snapshot.

## Syntax

```bash
ic resolve [<include> ...] [--exclude <label> ...] [--output FILE|-] [--no-redact]
```

## Arguments and options

- `<include> ...`
  Resolve only these labeled facts.
- `--exclude <label> ...`
  Resolve all indexed facts except these labels.
- `--output FILE|-`
  Write the resolved snapshot somewhere other than `actual.json`.
- `--no-redact`
  Do not redact secret facts.

## Notes

- With no include or exclude arguments, all indexed facts are resolved.
- The command prints progress to stderr while steps run.
- Export/import dependencies are sorted automatically.

## Example

```bash
ic resolve
ic resolve homepage
ic resolve --exclude noisy-banner --output -
```
