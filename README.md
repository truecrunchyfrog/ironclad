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

Set up an `.ironclad/` instance in the current directory:
```bash
$ ic ledger init
```

Create a cell:
```bash
$ ic cell add my-fragile-file
```

Add a stage to the pipeline of the cell `my-fragile-file`:
```bash
$ ic pipeline push my-fragile-file head.file.text --options '{"files": ["do-not-touch.txt"]}'
```
`head.file.text` is an operation that reads the content of the `files` specified.

You can try it out right away:
```bash
$ echo -n 'this file may\nNOT\nbe touched' > do-not-touch.txt
$ ic pipeline eval my-fragile-file
[
  {
    "content": "this file may\nNOT\nbe touched"
  }
]
(truncated)
```

- `head.*` operations take no input (apart from options) and produce an output.
- Non-`head.*` operations take input and produce an output.

A pipeline should always start with a `head.*` operation to seed the pipeline,
and the rest of the stages should consist of non-`head.*` operations to transform the batch.

Add another stage:
```bash
$ ic pipeline push my-fragile-file text.lines
```
This will split the file's lines into separate samples.

```bash
$ ic pipeline eval my-fragile-file
[
  {
    "content": "this file may"
  },
  {
    "content": "NOT"
  },
  {
    "content": "be touched"
  }
]
(truncated)
```
Testing pipelines directly does not modify any Ironclad state.

Run an audit to evaluate all cells and push their state into the pending snapshot:
```bash
$ ic audit
my-fragile-file: dirty (-0 +3)
1 not ack'd
```
The `my-fragile-file` cell is marked as dirty because the baseline is empty and the file has evaluated to three samples.

Acknowledge the changes to promote them to the baseline:
```bash
$ ic ack
my-fragile-file is dirty
+
  this file may
y/n/N/q/s/t/? = y
+
  NOT
y/n/N/q/s/t/? = y
+
  be touched
y/n/N/q/s/t/? = y
```

Once again, run an audit:
```bash
$ ic audit
ok!
```
If the cell was recently audited, the samples may be cached. Pass `--new` to enforce fresh samples.
