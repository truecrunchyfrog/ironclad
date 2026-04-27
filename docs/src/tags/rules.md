# Left and Right Rules

Rules describe how much text to keep to the left or right of the tag.

Arrows:
- `<-`
  select leftward
- `->`
  select rightward

Pipes change inclusivity:
- `|<-`
  exclude the boundary when selecting leftward
- `->|`
  exclude the boundary when selecting rightward

Without a pipe, the boundary is included.

Each tag can carry up to two rules:
- one left rule
- one right rule

If omitted, defaults are used.
