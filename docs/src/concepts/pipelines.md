# Pipelines

A fact pipeline is just the ordered list of steps in a fact.

Each step:
- chooses an operation with `use`
- optionally passes `options`
- receives the output samples of the previous step

## A tiny example

```toml
[[steps]]
use = "seed.file.text"
options.files = ["observatory.log"]

[[steps]]
use = "text.lines"

[[steps]]
use = "text.find"
options.regex = "comet-[0-9]+"
```

This pipeline:
1. reads a file
2. splits it into lines
3. extracts comet IDs from each line

## Per-sample and batch-style behavior

Most operations conceptually run per sample.

For example:
- `text.trim`
- `text.replace`
- `html.find`
- `json.find`

Some operations act more like batch filters or producers:
- `seed.*` operations usually create samples from nothing
- `slice` reorders or narrows the whole input batch
- `compact` removes empty samples from the whole input batch

You do not usually need to think about the distinction while authoring facts, but it helps explain the shape of the output.
