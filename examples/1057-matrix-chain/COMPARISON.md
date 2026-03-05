# Matrix Chain Multiplication — Comparison

## Core Insight
Matrix chain multiplication is the canonical interval DP problem. The key is trying every split point k in range [i, j) and taking the minimum total cost. A separate `split` table enables reconstructing the optimal parenthesization.

## OCaml Approach
- `Buffer` for building parenthesization string recursively
- `Printf.sprintf` for formatting matrix names
- `max_int` as initial sentinel
- `ref` cells for tracking best in inner loop

## Rust Approach
- `format!` macro for string building in recursive parenthesization
- `usize::MAX` as sentinel
- Nested function for recursive string building
- `HashMap` with tuple keys for memoization

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| String building | `Buffer` + `Printf.sprintf` | `format!()` macro |
| Infinity sentinel | `max_int` | `usize::MAX` |
| 2D table init | `Array.init n (fun _ -> Array.make n 0)` | `vec![vec![0; n]; n]` |
| Split tracking | Parallel `split` array | Parallel `split` vec |
| Recursion | Natural OCaml recursion | Inner `fn` with explicit params |
