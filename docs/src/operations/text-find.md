# `text.find`

Find text matches inside each sample.

## Options

```toml
text = ""
regex = ""
expand = ""
```

The actual options are:
- `text`
  plain substring match
- `regex`
  regular expression match
- `expand`
  optional regex expansion template

## Behavior

- Produces one sample per match
- Adds `start` and `end` trace entries
