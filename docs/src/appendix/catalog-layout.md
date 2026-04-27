# Catalog Layout

Typical catalog tree:

```text
.ironclad/
├── .gitignore
├── facts/
│   ├── 01...
│   └── 01...
├── index.toml
└── snapshots/
    ├── actual.json
    └── canon.json
```

The fact directory stores TOML files named by fact ID.
The index maps labels to those IDs.
