# Introduction

Ironclad records and compares the assumptions your software depends on.

Tests catch many failures, but they do not usually notice when:
- a vendor changes an HTTP response
- a CLI tool starts printing one extra warning line
- a config file gains a field you were not expecting
- an internal dashboard moves the sentence you scrape every Monday morning

Ironclad turns those assumptions into facts, facts into snapshots, and snapshots into material you can review directly.

## The basic rhythm

Most Ironclad workflows follow the same loop:

1. Define one or more facts.
2. Resolve the current state into a snapshot.
3. Compare that snapshot with the baseline.
4. Review the drift.
5. Apply the approved changes.

The command loop is intentionally small. The main flexibility comes from the fact pipeline itself: read a file, fetch a page, split some text, extract the line that matters, and keep only that.

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

## Why this differs from plain diffing

Ironclad does not only diff files. It diffs processed observations.

That means you can compare:
- the second JSON field inside a local file
- the text inside a specific HTML node
- the output of a command after trimming noise
- a section of a file selected by tags

The result is usually more targeted than watching an entire file and diffing every incidental change.
