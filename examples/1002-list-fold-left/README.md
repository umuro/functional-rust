[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1002 — List Fold Left
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement `fold_left` — the left-to-right accumulator fold — both as a thin wrapper around `Iterator::fold` and as an explicit tail-recursive function. Compute sum, product, and maximum from a list using the same fold abstraction. Compare with OCaml's `List.fold_left` and a custom tail-recursive implementation.

## Learning Outcomes

- Use `items.iter().fold(init, f)` as the idiomatic Rust fold
- Implement `fold_left_recursive` using slice pattern `[head, tail @ ..]` for educational comparison
- Understand that `fold_left` is left-associative: `((init ⊕ x₁) ⊕ x₂) ⊕ …`
- Derive `sum`, `product`, and `max` as specialisations of `fold_left`
- Map Rust's `Iterator::fold` to OCaml's `List.fold_left f init lst`
- Recognise that Rust's iterative `fold` is always tail-recursive; OCaml's recursive version needs manual TCO

## Rust Application

`fold_left_iter` wraps `items.iter().fold(init, f)` with a generic signature `fn(U, &[T], F) -> U`. The closure `f` receives `(acc: U, item: &T)` — the accumulator and a reference to the current element. `fold_left_recursive` uses a `while !items.is_empty()` loop (or slice destructuring) to process elements sequentially. Both produce the same results: `fold_left_iter(0, &[1,2,3,4,5], |acc, x| acc + x)` returns `15`.

## OCaml Approach

`List.fold_left (+) 0 numbers` is the standard library call. The custom `fold_left_recursive f acc = function | [] -> acc | head :: tail -> fold_left_recursive f (f acc head) tail` is tail-recursive and gets TCO by OCaml's compiler. Convenience functions `sum` and `product` are partial applications: `let sum = List.fold_left (+) 0`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Idiomatic | `iter.fold(init, f)` | `List.fold_left f init lst` |
| Argument order | `f(accumulator, item)` | `f acc item` |
| Item type | `&T` (reference from `iter()`) | `T` (value from list) |
| Tail recursion | Always (iterative) | TCO by compiler |
| Partial application | Not idiomatic (closure) | `let sum = List.fold_left (+) 0` |
| Max with empty | Requires `Option<U>` or sentinel | Same pattern |

`fold_left` is the universal list combinator: `map`, `filter`, `sum`, `product`, `reverse`, `length` — all are expressible as `fold_left`. Understanding it deeply is the foundation of functional list processing.

## Exercises

1. Implement `map` using `fold_left_iter` by collecting transformed elements into a `Vec`.
2. Implement `filter` using `fold_left_iter` by conditionally appending to the accumulator.
3. Implement `reverse` using `fold_left_iter` with a `Vec` accumulator using `insert(0, ...)`.
4. Write `fold_right` (right-associative fold) and show that `fold_right cons [] lst` reconstructs the list.
5. In OCaml, implement `List.fold_left` from scratch without using the standard library version, then verify it matches `List.fold_left` on 10 example inputs.
