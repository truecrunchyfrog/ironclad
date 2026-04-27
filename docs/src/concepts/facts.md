# Facts

A fact is a named assumption expressed as a small pipeline.

Examples:
- “this config file contains exactly three upstream hosts”
- “the lunch page still says Wednesday is dumpling day”
- “the build tool prints the same version string as yesterday”

Each fact lives in a TOML file under `.ironclad/facts/`.

## Labels and IDs

Facts have two identities:

- a fact ID
  usually a generated ULID-like string used as the filename
- a label
  a human-friendly name stored in `index.toml`

You can operate on many fact commands using either the label or the raw fact ID.

## Fact shape

A fact may include:
- `description`
- `imports`
- `exports`
- `steps`
- `secret`

The central part is the `steps` array. Each step uses an operation and optional options.

```toml
description = "Track the names of dragons currently listed in the registry."

[[steps]]
use = "seed.file.text"
options.files = ["dragons.txt"]

[[steps]]
use = "text.lines"

[[steps]]
use = "text.trim"

[[steps]]
use = "compact"
```

## Indexed and unindexed facts

Normally you create a fact with a label, which adds it to `index.toml`.

You can also create one with `--no-index`. That leaves the fact file on disk but without a label entry in the index.

That is occasionally useful for experimentation, but indexed facts are the normal case.
