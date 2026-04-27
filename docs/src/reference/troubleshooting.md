# Troubleshooting

## `catalog not found`

Ironclad could not discover `.ironclad/` from the current working directory.

Fixes:
- `cd` into the right container directory
- pass `--catalog-dir /path/to/.ironclad`

## `label not found`

You asked for a fact or snapshot entry that does not exist.

Fixes:
- run `ic list`
- check spelling
- if you meant a raw fact file, use the fact ID instead

## `fact selector not found`

The selector is neither:
- an indexed label
- nor an existing fact ID file

## Import/export failures

Typical causes:
- an imported key was never exported
- two facts exported the same key
- the export trace match no longer identifies any sample

## `op eval` feels odd with stdin

Remember that `--input -` consumes stdin for the batch itself.
Avoid also trying to source `--options` from stdin in the same invocation.

## Subprocess failures

Operations like `seed.run` and `run` surface:
- non-zero exits
- stderr
- signal termination when possible

That usually gives you enough information to debug the underlying command.
