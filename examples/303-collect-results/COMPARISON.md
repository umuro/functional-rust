# Collecting Results

| Concept | OCaml | Rust |
|---------|-------|------|
| All-or-nothing | Manual fold | `.collect::<Result<Vec<_>, _>>()` |
| Short-circuit | Manual | Automatic |
