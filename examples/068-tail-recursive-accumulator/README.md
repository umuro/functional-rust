# Example 068: Tail-Recursive Accumulator Pattern

**Difficulty:** ⭐⭐
**Category:** Recursion
**Concept:** Transforming naive recursion into tail recursion using an accumulator parameter. In OCaml, this enables tail-call optimization (TCO) for stack safety. In Rust, the pattern is instructive but iterators/loops are preferred since Rust doesn't guarantee TCO.
**OCaml → Rust insight:** OCaml guarantees TCO for tail-recursive functions; Rust does not — idiomatic Rust uses `iter().sum()`, `iter().rev()`, and loops instead of recursive accumulators.
