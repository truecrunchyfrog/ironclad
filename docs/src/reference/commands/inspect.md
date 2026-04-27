# `ic inspect`

Inspect one snapshot.

## Syntax

```bash
ic inspect [<label>] [--trace] [--snapshot FILE|-] [--raw]
```

## Arguments and options

- `<label>`
  Show detailed samples for one fact.
- `--trace`
  Include trace lines in detailed output.
- `--snapshot FILE|-`
  Read a snapshot from a file or stdin instead of the default `canon.json`.
- `--raw`
  Print the entire snapshot as JSON.

## Behavior

- Without a label, `inspect` prints one overview line per fact:
  label, sample count, created timestamp.
- With a label, it prints structured sample records.

## Example

```bash
ic inspect
ic inspect tea-menu
ic inspect tea-menu --trace
```
