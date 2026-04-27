# First Fact

Create a fact with a friendly label:

```bash
ic add comet-board
```

Open it:

```bash
ic edit comet-board
```

Give it a description and a pipeline:

```toml
description = "Track the currently advertised comet names."

[[steps]]
use = "seed.file.text"
options.files = ["comets.txt"]

[[steps]]
use = "text.lines"

[[steps]]
use = "text.trim"

[[steps]]
use = "compact"
```

Resolve it:

```bash
ic resolve comet-board --output -
```

Show the fact later:

```bash
ic show comet-board
ic show comet-board --path
```

That is enough to establish the rhythm: author, resolve, inspect.
