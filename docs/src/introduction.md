# Introduction

Ironclad watches the assumptions your software quietly leans on.

Tests catch a lot, but they do not usually notice when:
- a vendor quietly changes an HTTP response
- a CLI tool starts printing one extra warning line
- a config file grows a field you were not expecting
- an internal dashboard moves the one sentence you scrape every Monday morning

Ironclad turns those assumptions into facts, facts into snapshots, and snapshots into something you can review with intent instead of surprise.

## The basic rhythm

Most Ironclad workflows follow the same loop:

1. Define one or more facts.
2. Resolve the current state into a snapshot.
3. Compare that snapshot with the baseline.
4. Review the drift.
5. Apply the approved changes.

That loop is small on purpose. The interesting part is not the number of commands. It is the fact that each fact can describe a tiny pipeline: read a file, fetch a page, split some text, pull out the one line you actually care about, and keep only that.

## What Ironclad stores

An Ironclad catalog lives in `.ironclad/` and usually contains:

```text
.ironclad/
├── facts/
├── index.toml
└── snapshots/
    ├── actual.json
    └── canon.json
```

- `facts/` holds fact definitions.
- `index.toml` maps friendly labels to fact IDs.
- `snapshots/canon.json` is your approved baseline.
- `snapshots/actual.json` is the latest resolved state.

## Why this feels different from plain diffing

Ironclad does not only diff files. It diffs processed observations.

That means you can compare:
- the second JSON field inside a local file
- the text inside a specific HTML node
- the output of a command after trimming noise
- a section of a file selected by tags

The end result is often much calmer than “watch the whole file and hope for the best”.
