# Setup

An Ironclad catalog is where you keep your facts, and the snapshots derived by those facts.

In your workspace root, initialize a catalog.
```bash
$ ic init
```

This scaffolds the catalog (`.ironclad`) inside the working directory.
```
.ironclad
├── facts
└── index.toml
```

- `facts` is a directory containing all of your facts.
- `index.toml` is a file that links the files in `facts` with friendlier names.
