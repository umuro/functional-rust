# try_fold — Comparison

## Core Insight
Regular fold processes all elements. `try_fold` stops at the first failure — essential for validation pipelines and overflow-safe arithmetic.

## OCaml Approach
- No built-in `try_fold` — must write recursive version
- Pattern match on `Ok`/`Error` at each step
- Can use `Seq` for lazy evaluation with short-circuit

## Rust Approach
- `Iterator::try_fold` is built-in and optimized
- Returns `Result<Acc, E>` — Err short-circuits
- Also works with `Option` (None short-circuits)
- Compiler can optimize the early-exit path

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Built-in | No | Yes (`try_fold`) |
| Short-circuit | Manual recursion | Automatic |
| Works with | `Result` (manual) | `Result`, `Option`, `ControlFlow` |
| Performance | Recursive | Optimized iterator machinery |
| Related | `fold_left` | `fold`, `try_for_each` |
