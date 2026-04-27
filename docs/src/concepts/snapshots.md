# Snapshots

A snapshot is a map of fact labels to batches of samples.

Ironclad usually deals with two snapshots:
- `actual`
  the latest resolved state
- `canon`
  the approved baseline

## What snapshots contain

Each fact label points to a batch:
- `samples`
- `created`

Each sample contains:
- `content`
- `traces`

The `content` is what you actually compare.
The `traces` explain where that content came from.

## Baseline and proposal

In review-oriented terms:
- `canon` is the baseline
- `actual` is the proposal

`ic diff` compares the two.
`ic inspect` lets you read one snapshot.
`ic apply` promotes approved entries from proposal into baseline.

## What counts as drift

Drift means the batches differ.

That includes:
- added samples
- removed samples
- changed content
- multiplicity changes

If a batch used to contain one copy of a sample and now contains two, Ironclad treats that as drift.
