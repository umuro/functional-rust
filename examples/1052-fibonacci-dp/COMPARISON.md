# Fibonacci Bottom-Up DP — Comparison

## Core Insight
Bottom-up DP eliminates recursion overhead. The O(1) space variant using tuple state `(a, b) → (b, a+b)` is naturally expressed as a fold in both languages, showing how functional patterns align with space-optimal DP.

## OCaml Approach
- Array-based DP with imperative `for` loop and mutable array
- `ref` cells for the two-variable version (imperative in functional clothing)
- `List.fold_left` with tuple destructuring for the pure functional version
- `List.init` creates the iteration range

## Rust Approach
- `Vec`-based DP table — explicit allocation, indexed access
- Tuple destructuring `let (mut a, mut b)` for O(1) space
- `(2..=n).fold((0, 1), |(a, b), _| (b, a+b))` — idiomatic iterator chain
- Range iterators are zero-cost abstractions

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Array DP | `Array.make n 0` | `vec![0u64; n + 1]` |
| Mutable vars | `ref` cells + `:=` | `let mut` + `=` |
| Fold syntax | `List.fold_left (fun (a,b) _ -> ...)` | `(2..=n).fold((0,1), \|(a,b), _\| ...)` |
| Range creation | `List.init (n-1) Fun.id` (allocates list) | `2..=n` (zero-cost iterator) |
| Space complexity | Same O(1) for both | Same O(1) for both |
| Overflow handling | Silent overflow (OCaml ints) | Panic on debug, wrap on release |
