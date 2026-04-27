# Operating Outside a Catalog

Most commands need a catalog.

Some do not.

## `op eval`

`ic op eval` can run outside a catalog when the operation does not require catalog files.

For example:

```bash
printf '[{"traces":[{}],"content":" hello "}]' \
  | ic op eval text.trim --input -
```

That makes `op eval` useful as a tiny pipeline laboratory.

## When a catalog is still required

Operations that need filesystem context or catalog-backed paths still expect a meaningful working directory, and fact-related commands still require a real catalog directory.
