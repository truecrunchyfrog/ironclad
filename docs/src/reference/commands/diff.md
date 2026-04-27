# `ic diff`

Compare two snapshots.

## Syntax

```bash
ic diff [<label>] [--trace] [--proposal FILE|-] [--baseline FILE|-] [--raw]
```

## Arguments and options

- `<label>`
  Show detailed change records for one fact.
- `--trace`
  Include traces in detailed output.
- `--proposal FILE|-`
  Read the resolved snapshot from somewhere other than `actual.json`.
- `--baseline FILE|-`
  Read the approved snapshot from somewhere other than `canon.json`.
- `--raw`
  Print the whole diff structure as JSON.

## Behavior

- Without a label, `diff` prints one overview line per changed fact.
- With a label, it prints numbered change records with explicit `before` and `after` sections.
- Unchanged facts are omitted from the overview.

## Example

```bash
ic diff
ic diff tea-menu
ic diff tea-menu --trace
```
