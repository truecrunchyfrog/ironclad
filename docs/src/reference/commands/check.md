# `ic check`

Check whether two snapshots are identical.

## Syntax

```bash
ic check [--proposal FILE|-] [--baseline FILE|-]
```

## Options

- `--proposal FILE|-`
  Read the resolved snapshot from somewhere other than `actual.json`.
- `--baseline FILE|-`
  Read the approved snapshot from somewhere other than `canon.json`.

## Behavior

- Prints `ok (0)` when nothing drifted.
- Prints `drift (N)` when `N` facts drifted.
- Exits `0` for no drift and `1` for any drift.

This is the command you want in CI.
