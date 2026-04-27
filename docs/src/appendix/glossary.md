# Glossary

- catalog directory
  the `.ironclad/` directory
- container directory
  the directory above the catalog directory
- fact
  a TOML pipeline describing one tracked assumption
- fact selector
  a fact label or fact ID accepted by commands such as `show`, `edit`, and `remove`
- fact ID
  the file-based identifier of a fact
- label
  the human-friendly indexed name of a fact
- sample
  one unit of tracked content
- trace
  provenance metadata attached to a sample
- batch of samples
  the set of samples produced by one fact
- resolved snapshot
  the latest captured snapshot, usually stored in `actual.json`
- approved snapshot
  the reviewed snapshot, usually stored in `canon.json`
