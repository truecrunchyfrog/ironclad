# `ic apply`

Promote snapshot entries into the approved snapshot.

## Syntax

```bash
ic apply <label> ...
ic apply --all
```

## Options

- `<label> ...`
  Promote only these facts.
- `--all`
  Replace the approved snapshot with the full resolved snapshot.
- `--promotion FILE|-`
  Use a snapshot other than `actual.json` as the resolved source.
- `--baseline FILE|-`
  Use a snapshot other than `canon.json` as the approved source.
- `--output FILE|-`
  Write the updated approved snapshot somewhere other than `canon.json`.

## Notes

- Applying selected labels can add, replace, or remove those labels in the approved snapshot.
- If a requested label is absent from both the resolved snapshot and the approved snapshot, the command fails.
