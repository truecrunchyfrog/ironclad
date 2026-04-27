# `compact`

Remove samples whose content is empty.

## Options

None.

## Behavior

- Operates over the whole batch
- Keeps sample order
- Removes only samples whose content is exactly `""`

This pairs naturally with `text.trim`.
