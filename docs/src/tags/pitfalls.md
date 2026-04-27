# Pitfalls

## Bytes are not characters

`B` counts bytes, not graphemes. This matters for Unicode text.

## Text boundary misses can be surprising

If a text boundary is absent, the selection logic falls back in a way that may not match your first guess. Test unusual rules with `ic op eval text.tag`.

## Tags are powerful but local

Tags are great when you control the source text. They are usually the wrong tool for arbitrary external HTML or JSON, where dedicated parsers are clearer.
