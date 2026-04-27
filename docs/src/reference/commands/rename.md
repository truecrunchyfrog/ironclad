# `ic rename`

Rename an indexed fact label.

## Syntax

```bash
ic rename <selector> <new-label>
```

## Arguments

- `<selector>`
  A fact label or fact ID.
- `<new-label>`
  The new label.

## Notes

- The underlying fact file does not change; the index mapping does.
- Reusing another label fails unless it already points to the same fact ID.
