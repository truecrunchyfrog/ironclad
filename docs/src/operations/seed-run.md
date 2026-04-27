# `seed.run`

Run one program and capture its stdout as a sample.

## Options

```toml
program = ""
args = []
```

## Behavior

- Runs once for the whole step
- Uses the operation working directory
- Produces one sample from stdout
- Fails on non-zero exit
