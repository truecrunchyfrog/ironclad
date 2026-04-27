# `run`

Run a program once per sample, piping the sample content to stdin.

## Options

```toml
program = ""
args = []
```

## Behavior

- Runs once per input sample
- Writes sample content to child stdin
- Replaces content with child stdout
- Adds a new empty trace step to preserve lineage

## Example

```toml
[[steps]]
use = "run"
options.program = "rev"
```

`hello` becomes `olleh`.
