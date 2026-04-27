# Building Pipelines

Pipelines work best when each step does one small thing clearly.

The most reliable style is usually:

1. seed a source
2. split it into useful pieces
3. normalize noisy whitespace
4. narrow it to the parts you truly care about

## Example: from messy bulletin board to clean samples

Imagine a hand-maintained text file:

```text
  Neptune Parade

Moonlight Chess Club
    
Lantern Repair Night
```

A neat fact might look like:

```toml
[[steps]]
use = "seed.file.text"
options.files = ["bulletin.txt"]

[[steps]]
use = "text.lines"

[[steps]]
use = "text.trim"

[[steps]]
use = "compact"
```

This is more stable than diffing the whole file. You keep the meaningful lines and discard the empty dramatic pauses.

## Prototype with `op eval`

When a step is tricky, prototype it before baking it into a fact:

```bash
ic op eval text.trim --input -
ic op eval text.find --input - --options '{ text = "Neptune" }'
```

That is often faster than repeatedly editing the fact file.
