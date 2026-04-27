# Configuration

Ironclad configuration comes from:

1. CLI flags
2. environment variables prefixed with `IC_`
3. a config file

## CLI flags

Global flags:
- `-v`, `-vv`, `-vvv`
  increase log verbosity
- `--config-file PATH`
  use a specific config file
- `--catalog-dir PATH`
  point at the exact catalog directory

## Config file

By default Ironclad looks for:

```text
~/.config/ironclad/config.toml
```

## Environment variables

Environment variables are read with the `IC_` prefix.

Examples:
- `IC_CATALOG_DIR`
- `IC_VERBOSE`

## Precedence

Practical rule of thumb:
- explicit CLI flags win
- environment can override file defaults
- config file provides the baseline
