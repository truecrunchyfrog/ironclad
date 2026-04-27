# `ic apply`

Promote snapshot entries into the baseline.

## Syntax

```bash
ic apply <label> ...
ic apply --all
```

## Options

- `<label> ...`
  Promote only these facts.
- `--all`
  Replace the baseline with the full promotion snapshot.
- `--promotion FILE|-`
  Use a snapshot other than `actual.json` as the promotion source.
- `--baseline FILE|-`
  Use a snapshot other than `canon.json` as the baseline source.
- `--output FILE|-`
  Write the promoted result somewhere other than `canon.json`.

## Notes

- Applying selected labels can add, replace, or remove those labels in the baseline.
- If a requested label is absent from both the promotion and the baseline, the command fails.
