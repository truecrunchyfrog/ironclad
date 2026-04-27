# Installation

You can install Ironclad with Cargo.

## From local source

```bash
cargo install --path crates/ironclad-cli
```

## From GitHub

```bash
cargo install --git https://github.com/truecrunchyfrog/ironclad --branch master
```

## Verify the install

```bash
ic --help
ic op list
```

If `ic op list` prints a list of operation IDs, the CLI is installed and the built-in operations are registered.
