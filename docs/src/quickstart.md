# Quickstart

Set up an instance:
```bash
$ ic init
```

Create a fact:
```bash
$ ic add my-config
```

Open your new fact in `$EDITOR`:
```bash
$ ic edit my-config
```

Create a pipeline for the fact:

`.ironclad/facts/01KQ3EM5NBJAQHH5XKBJXDQHGQ.toml`
```toml
[[steps]]
use = "seed.file.text"
options.files = ["config.json"]

[[steps]]
use = "json.find"
options.path = "$.*"
```

`config.json`
```json
{
  "api_key": "password",
  "base_url": "https://192.168.0.1/api"
}
```

Evaluate the pipeline:
```bash
$ ic resolve my-config --output - | jq '.config.samples'
[
  {
    "traces": [
      {
        "path": "config.json"
      },
      {
        "json_node_path": "$['api_key']"
      }
    ],
    "content": "password"
  },
  {
    "traces": [
      {
        "path": "config.json"
      },
      {
        "json_node_path": "$['base_url']"
      }
    ],
    "content": "https://192.168.0.1/api"
  }
]
```
