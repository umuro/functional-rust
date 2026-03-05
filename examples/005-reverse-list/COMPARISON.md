## Core Insight

Reversing a list reveals the importance of accumulator patterns. The naive approach (reverse tail, append head) creates O(n²) work. The tail-recursive approach (prepend to accumulator) achieves O(n).

## OCaml Approach
- `List.rev` — standard library, O(n)
- Naive: `rev xs @ [x]` — O(n²)
- Tail-recursive: accumulator pattern with `x :: acc`

## Rust Approach
- `.reverse()` — in-place mutation, O(n)
- `.iter().rev().collect()` — creates new reversed Vec
- Manual fold: `.fold(vec![], |acc, x| ...)`

## Comparison Table

| Approach | OCaml | Rust |
|----------|-------|------|
| Built-in | `List.rev` | `.reverse()` / `.rev()` |
| Naive recursive | `rev tail @ [head]` | `rec_reverse(&v[1..]).push(v[0])` |
| Tail-recursive | `aux (x::acc) xs` | loop with accumulator |
| Complexity (good) | O(n) | O(n) |
