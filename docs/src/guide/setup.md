# Setup

Start in the directory you want Ironclad to observe and initialize a catalog:

```bash
ic init
```

That creates `.ironclad/` in the current directory.

```text
.ironclad/
├── .gitignore
├── facts/
├── index.toml
└── snapshots/
```

The usual habit is:
- keep `.ironclad/` in the project root
- keep the files you observe next to it in the same container directory

That keeps relative paths in fact files simple and predictable.

## Using an explicit catalog

You can point commands at a specific catalog directory with `--catalog-dir`:

```bash
ic --catalog-dir /path/to/workspace/.ironclad inspect
```

Pass the actual `.ironclad/` path, not its parent.
