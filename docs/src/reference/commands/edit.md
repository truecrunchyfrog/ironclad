# `ic edit`

Open a fact in your editor.

## Syntax

```bash
ic edit <selector>
```

## Arguments

- `<selector>`
  A fact label or fact ID.

## Notes

- Uses `$EDITOR`.
- The command fails if `$EDITOR` is unset, empty, or cannot be launched.
- The editor exit code is propagated when possible.
