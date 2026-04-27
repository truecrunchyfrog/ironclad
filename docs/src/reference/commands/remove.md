# `ic remove`

Remove a fact file and its index entry.

## Syntax

```bash
ic remove <selector>
```

## Arguments

- `<selector>`
  A fact selector: either a label or a fact ID.

## Notes

- The fact file is deleted.
- If the fact was indexed, the matching label is removed from `index.toml`.
