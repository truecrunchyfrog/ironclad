# Quickstart

This chapter shows the shortest path from an empty directory to a working fact.

## 1. Initialize a catalog

```bash
ic init
```

This creates `.ironclad/` in the current directory.

## 2. Add a fact

```bash
ic add tea-menu
```

Ironclad prints either the label you chose or a fact ID if you created an unindexed fact.

## 3. Open the fact

```bash
ic edit tea-menu
```

Put a small pipeline in it:

```toml
description = "Track the teas currently advertised in the cafe window."

[[steps]]
use = "seed.file.text"
options.files = ["menu.txt"]

[[steps]]
use = "text.lines"

[[steps]]
use = "text.trim"

[[steps]]
use = "compact"
```

And add a file:

```text
Jasmine Green

Smoked Earl Grey
Ube Oolong
```

## 4. Resolve the current state

```bash
ic resolve tea-menu --output -
```

The snapshot should contain three samples.

## 5. Accept it as the approved snapshot

```bash
ic apply tea-menu
```

If this is the first run, you can also approve everything:

```bash
ic apply --all
```

## 6. Review later changes

```bash
ic resolve
ic diff
ic inspect tea-menu
ic check
```

At that point you have the basic workflow: capture, compare, inspect, and approve.
