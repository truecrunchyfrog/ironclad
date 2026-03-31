# ironclad

Ironclad is (ideally) a deterministic source tracker that ensures mutations to moving parts of a system are observed, enabling reaction.
It is essentially a testing framework for dev ops.

The moving parts, "cells", may regularly be compared against the "baseline", the accepted state.
When the current state differs from the baseline and
the change is wanted, you may acknowledge the change, by promoting it to the baseline, making it the new accepted state.
If the change is unwanted, well, then you may deal with that.

A cell is any source point obtained anywhere, internal or external to your system, that you wish to react upon when changed.

The state of cells are derived from pipelines.
A pipeline starts off with a seed operation, that retrieves some source, such as a local file or a web resource.
Stage by stage the pipeline refines the seed to eventually point at the exact data relevant to the cell.

## Usage

Create an instance (`.ironclad/`) in the current directory:
```bash
ic ledger init
```
It may be tracked by a VCS.

Create a cell:
```bash
ic cell add my-tracker
```

Add a stage to the pipeline of the cell `my-tracker`:
```bash
ic pipeline push my-tracker head.file.text --options '{"files": ["do-not-touch.txt"]}'
```

- `head.*` operations take no input (apart from options) and produce an output.
- non-`head.*` operations take input and produce an output.
A pipeline should always start with a `head.*` operation to seed the pipeline,
and the rest of the stages should consist of non-`head.*` operations to transform the batch.

Split the file's lines into separate samples:
```bash
ic pipeline push my-tracker text.lines
```

You can try it out right away:
```bash
echo 'this file may\nNOT\nbe touched' > do-not-touch.txt
ic pipeline eval my-tracker
# [
#   [
#     {
#       "traces": [
#         {
#           "path": "do-not-touch.txt"
#         },
#         {}
#       ],
#       "content": "this file may"
#     },
#     {
#       "traces": [
#         {
#           "path": "do-not-touch.txt"
#         },
#         {}
#       ],
#       "content": "NOT"
#     },
#     {
#       "traces": [
#         {
#           "path": "do-not-touch.txt"
#         },
#         {}
#       ],
#       "content": "be touched"
#     }
#   ]
# ]
```
Evaluating pipelines directly does not modify any Ironclad state.

Currently, no state is on the record.

Run an audit to evaluate all cells and push their state into the pending snapshot:
```bash
ic audit
# my-tracker: dirty (-0 +3)
# 1 not ack'd
```
The cell is marked as dirty as the baseline is empty, but the file has evaluated to three samples:

Acknowledge the changes to promote them to the baseline:
```bash
ic ack
# my-tracker is dirty
# +
#   this file may
# y/n/N/q/s/t/? = y
# +
#   NOT
# y/n/N/q/s/t/? = y
# +
#   be touched
# y/n/N/q/s/t/? = y
```

Once again run an audit:
```bash
ic audit
# ok!
```
If the cell was recently audited, the samples may be cached. Pass `--new` to enforce fresh samples.
