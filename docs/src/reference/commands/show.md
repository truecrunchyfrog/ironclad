# `ic show`

Show a fact summary or its file path.

## Syntax

```bash
ic show <selector> [--path]
```

## Arguments and options

- `<selector>`
  A fact selector: either a label or a fact ID.
- `--path`
  Print the fact file path instead of its description.

## Notes

- Without `--path`, the command currently prints the fact description.
- If no description is set, it prints an empty line.
