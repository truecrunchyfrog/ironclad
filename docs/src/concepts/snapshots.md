# Snapshots

A snapshot is a map of fact labels to batches of samples.

Ironclad usually deals with two snapshots:
- the resolved snapshot, usually stored in `actual.json`
- the approved snapshot, usually stored in `canon.json`

## What snapshots contain

Each fact label points to a batch:
- `samples`
- `created`

Each sample contains:
- `content`
- `traces`

The `content` is what you actually compare.
The `traces` explain where that content came from.

## Approved and resolved snapshots

In review-oriented terms:
- the approved snapshot is the baseline
- the resolved snapshot is the proposal

`ic diff` compares the two.
`ic inspect` lets you read one snapshot.
`ic apply` promotes approved entries from the resolved snapshot into the approved snapshot.

## What counts as drift

Drift means the batches differ.

That includes:
- added samples
- removed samples
- changed content
- multiplicity changes

If a batch used to contain one copy of a sample and now contains two, Ironclad treats that as drift.
