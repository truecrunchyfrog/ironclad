# Catalogs

A catalog is the metadata directory where Ironclad keeps facts, indexes, and snapshots.

The catalog directory is `.ironclad/`.

The directory above it is the container directory.

That distinction matters:
- the catalog directory stores Ironclad data
- the container directory is the workspace your facts usually observe

For example:

```text
project-root/
├── .ironclad/
├── app/
├── config/
└── README.md
```

Here:
- `project-root/.ironclad/` is the catalog directory
- `project-root/` is the container directory

## Discovery

If you do not pass `--catalog-dir`, Ironclad searches upward from the current working directory until it finds `.ironclad/`.

If you do pass `--catalog-dir`, it must point to the actual catalog directory itself. Ironclad does not append `.ironclad` for you.

That is deliberate. It keeps explicit paths explicit.

## Catalog layout

```text
.ironclad/
├── .gitignore
├── facts/
├── index.toml
└── snapshots/
    ├── actual.json
    └── canon.json
```

- `facts/` holds fact files.
- `index.toml` maps labels to fact IDs.
- `snapshots/actual.json` stores the last resolved state.
- `snapshots/canon.json` stores the approved baseline.
