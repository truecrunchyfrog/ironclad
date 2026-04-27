# Selectors

Several commands let you choose facts by fact selector.

A fact selector can be:
- a label
- a fact ID

This applies to commands such as:
- `ic show`
- `ic edit`
- `ic rename`
- `ic remove`

## Include and exclude sets

`ic resolve` works slightly differently.

It supports:
- positional include labels
- `--exclude` labels

That lets you answer questions such as:
- “resolve only the homepage fact”
- “resolve everything except the chatty HTTP fact”

## Missing selectors

If a fact selector does not resolve to a known indexed label or an existing fact file, Ironclad fails explicitly.

That is intentional. Ignoring a misspelled selector would make command results ambiguous and harder to trust.
