# Review Workflow

The review workflow is the heart of Ironclad.

## Resolve

Capture the current state:

```bash
ic resolve
```

That writes `.ironclad/snapshots/actual.json`, the resolved snapshot.

## Inspect

Get an overview:

```bash
ic inspect
```

Inspect one fact in detail:

```bash
ic inspect comet-board
ic inspect comet-board --trace
```

## Diff

See which facts changed:

```bash
ic diff
```

See the detailed per-sample change records for one fact:

```bash
ic diff comet-board
ic diff comet-board --trace
```

## Check

Use `check` when you want a clean success/failure exit status:

```bash
ic check
```

It prints either `ok (0)` or `drift (N)` and exits `0` or `1`.

## Apply

Approve one fact:

```bash
ic apply comet-board
```

Approve everything:

```bash
ic apply --all
```

That promotes entries from the resolved snapshot into the approved snapshot.
