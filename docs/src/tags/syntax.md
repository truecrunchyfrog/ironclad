# Tag Syntax

The base syntax is:

```text
~ic=<id>
```

Or with rules:

```text
~ic=<id>=(...)
```

The ID is the value used by `text.tag`.

## Minimal example

```text
The moon is calm ~ic=moon-line
```

And the fact step:

```toml
[[steps]]
use = "text.tag"
options.tag = "moon-line"
```
