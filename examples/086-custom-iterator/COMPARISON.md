## Core Insight

Custom iterators encapsulate state. Each call to `next()` advances the internal state machine. This is more explicit than OCaml's closure-based sequences.

## OCaml Approach
- Closure captures mutable ref: `let r = ref 0 in fun () -> incr r; !r`
- Or pure: state threaded through `Seq` thunks

## Rust Approach
- Struct with state fields
- `impl Iterator` with `next(&mut self)`
- State mutation is explicit and checked by borrow rules

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| State | Closure ref / pure threading | Struct fields |
| Mutation | `ref` / `incr` | `&mut self` |
| Infinite | Lazy Seq | Iterator (never returns None) |
