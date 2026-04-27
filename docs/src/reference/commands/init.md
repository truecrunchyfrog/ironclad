# `ic init`

Initialize a catalog.

## Syntax

```bash
ic init [--dir PATH]
```

## Options

- `--dir PATH`
  Create the catalog at the given path. If the path already ends in `.ironclad`, that exact directory is used. Otherwise Ironclad creates `.ironclad/` inside it.

## Notes

- `ic init` creates the catalog directory and its initial files.
- If the target already exists, the command fails.

## Example

```bash
ic init
ic init --dir /srv/aurora/.ironclad
```
