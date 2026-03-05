# N-Queens — Comparison

## Core Insight
N-Queens is the classic backtracking problem. Three arrays track column and diagonal conflicts for O(1) safety checks. The bitmask approach (Rust only) compresses these into integers for maximum performance.

## OCaml Approach
- Boolean arrays for column/diagonal tracking
- `List.mapi` + `List.for_all` for functional safety check
- `List.concat_map` for generating all valid continuations
- Accumulation via `ref` list or pure list return

## Rust Approach
- `Vec<bool>` for column/diagonal tracking
- Inner `fn` with many `&mut` parameters (borrow checker requires explicit passing)
- Bitmask variant using bit manipulation (`wrapping_neg`, `& (bits-1)`)
- `clone()` for saving board state to solutions

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Safety check | `List.mapi` + `List.for_all` | `.iter().enumerate().all()` |
| Solution storage | `ref` list, prepend + `List.rev` | `Vec::push` + `clone()` |
| Functional style | `List.concat_map` for branching | Recursive with `push`/`pop` |
| Bitmask variant | Not shown (less idiomatic) | `u32` bit manipulation — fastest |
| Parameter passing | Closures capture mutable state | Explicit `&mut` params (borrow checker) |
