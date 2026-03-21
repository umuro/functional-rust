📖 **[View on hightechmind.io →](https://hightechmind.io/rust/915-iterator-min-by-max-by)**

---

# 915-iterator-min-by-max-by — Custom Comparison min_by / max_by
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Standard `min()` and `max()` require `Ord` — total ordering — which excludes `f64` (NaN breaks total order) and types requiring custom comparison semantics. `min_by(cmp)` and `max_by(cmp)` accept an explicit `Fn(&A, &A) -> Ordering` comparator, enabling: comparing floats with `partial_cmp().unwrap_or(Equal)`, multi-key comparison with `.then_with(|| a.secondary.cmp(&b.secondary))`, reverse ordering by swapping arguments, and domain-specific orderings like "closest to zero." This is the Rust equivalent of OCaml's `List.min_elt ~compare` and Python's `min(key=...)`.

## Learning Outcomes

- Use `.min_by(cmp)` and `.max_by(cmp)` with a custom `Ordering` comparator
- Handle `f64` comparison using `partial_cmp().unwrap_or(Ordering::Equal)`
- Implement multi-key comparison using `.then_with()`
- Achieve reverse ordering by swapping comparator arguments
- Compare with OCaml's `List.min_elt ~compare` and sort comparators

## Rust Application

The tests demonstrate: `min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))` for floats. `min_by(|a, b| b.cmp(a))` for reverse order (finds max). Multi-key comparison: `.min_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)))` finds the lexicographically smallest shortest string. `max_by` with a custom struct comparator. The comparator is a `Fn(&A, &A) -> Ordering` — any function or closure satisfying this signature works.

## OCaml Approach

`List.min_elt ~compare: ('a -> 'a -> int) -> 'a list -> 'a option` (Base library). Standard OCaml requires manual `fold_left`: `List.fold_left (fun acc x -> if compare x acc <= 0 then x else acc)`. Multi-key: `let cmp a b = let c = String.length a - String.length b in if c <> 0 then c else String.compare a b`. Float comparison: `Float.compare` handles NaN consistently. OCaml's `compare` returns `int`, Rust's `Ordering` is a typed enum — both provide equivalent expressiveness.

## Key Differences

1. **Typed Ordering**: Rust `Ordering` is an enum (`Less`, `Equal`, `Greater`); OCaml's comparator returns `int` (negative, zero, positive).
2. **f64 safety**: Rust explicitly requires handling NaN via `partial_cmp().unwrap_or()`; OCaml's `Float.compare` defines NaN ordering (NaN < everything).
3. **Composition**: Rust `.then_with()` on `Ordering` makes multi-key comparison ergonomic; OCaml chains `if c <> 0 then c else ...`.
4. **Standard library**: Both `min_by` and `max_by` are standard in Rust; OCaml's standard library requires `Base` or manual implementation.

## Exercises

1. Find the string that comes last lexicographically (max by reverse alphabetical order) using `max_by(|a, b| b.cmp(a))`.
2. Implement `closest_to_zero(data: &[f64]) -> Option<f64>` using `min_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap())`.
3. Write a comparator for `struct Event { timestamp: u64, priority: u8 }` that sorts by priority descending, then timestamp ascending, using `.then_with()`.
