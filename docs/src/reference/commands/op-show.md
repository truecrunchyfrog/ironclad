# `ic op show`

Show operation metadata.

## Syntax

```bash
ic op show <operation-id>
```

## Behavior

The command prints:
- the operation ID
- the operation description
- a TOML template of its default options, if any

## Example

```bash
ic op show seed.run
ic op show text.find
```
