# Rod Cutting — Comparison

## Core Insight
Rod cutting maximizes revenue by trying every possible first cut. It's equivalent to unbounded knapsack where items (cut lengths) can be reused. Cut reconstruction uses a parallel array tracking which cut was optimal at each length.

## OCaml Approach
- Imperative loops with `Array` for DP table
- `ref` cells for tracking best and remaining in reconstruction
- `List.rev` to reverse the accumulated cuts list
- `min` for bounding loop range

## Rust Approach
- `vec!` DP table with method-chained `.max()`
- `HashMap` for memoization
- `Vec` for cut reconstruction with `push`
- `.min()` method on `usize` for range bounding

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Range bound | `min i (Array.length prices)` | `i.min(prices.len())` |
| Cut tracking | Parallel `cuts` array + `ref` reconstruction | Parallel `cuts` vec + `while` loop |
| List building | Prepend `::` + `List.rev` | `Vec::push` (already in order) |
| Inner loop | `for j = 1 to min i len` | `for j in 1..=i.min(len)` |
