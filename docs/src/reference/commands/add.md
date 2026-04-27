# `ic add`

Create a fact.

## Syntax

```bash
ic add <label>
ic add --no-index
```

## Arguments and options

- `<label>`
  Assign a friendly label and add the fact to `index.toml`.
- `--no-index`
  Create the fact file without indexing it.

## Notes

- When indexed, the command prints the label.
- When unindexed, the command prints the raw fact ID.
- Duplicate labels are rejected.

## Example

```bash
ic add tea-menu
ic add --no-index
```
