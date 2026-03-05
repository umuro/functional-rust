# unwrap_or Patterns

| Pattern | Rust | OCaml |
|---------|------|-------|
| Eager | `unwrap_or(x)` | `Option.value ~default:x` |
| Lazy | `unwrap_or_else(f)` | `Option.value_or_thunk` |
| Default | `unwrap_or_default()` | N/A |
