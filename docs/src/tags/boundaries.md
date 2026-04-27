# Selection Boundaries

Tag rules use three boundary types:

- line boundaries: `1L`, `3L`
- byte boundaries: `4B`, `20B`
- text boundaries: `'boundary text'`

## Lines

`1L` means one line boundary away.

## Bytes

`4B` means four bytes away.

Use this carefully with Unicode text.

## Text

`'marker'` means search for a text marker.

Escapes supported in text boundaries include:
- `\\`
- `\'`
- `\n`
- `\r`
- `\t`
