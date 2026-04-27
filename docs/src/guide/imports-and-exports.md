# Imports and Exports

Facts can depend on values produced by other facts.

This happens through exports and imports.

## Export a sample

Exports name one sample from a fact output batch.

```toml
exports.base_url = { trace_key = "json_node_path", trace_value = "$['base_url']" }
```

That means:
- find the sample whose trace contains that key/value pair
- export it under the key `base_url`

## Import a value

Another fact can import that key:

```toml
imports = ["base_url"]
```

And use it in a step option:

```toml
[[steps]]
use = "seed.net.http"
options.url = "$(base_url)"
```

## Exact placeholder behavior

Imports are resolved only when the string value is exactly `$(key)`.

That is important:
- `url = "$(base_url)"` resolves
- `url = "prefix $(base_url)"` does not

This keeps interpolation narrow and predictable.

## Export key rules

Export keys must be globally unique across the resolved fact set for a single snapshot.

If two facts export the same key, resolution fails. Ironclad would rather stop loudly than guess.
