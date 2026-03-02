# Comparison: List Length

## OCaml — Naive

```ocaml
let rec length_naive = function
  | [] -> 0
  | _ :: t -> 1 + length_naive t
```

## OCaml — Tail-recursive

```ocaml
let length list =
  let rec aux n = function
    | [] -> n
    | _ :: t -> aux (n + 1) t
  in aux 0 list
```

## Rust — Idiomatic

```rust
pub fn length<T>(slice: &[T]) -> usize {
    slice.len()  // O(1) — stored metadata
}
```

## Rust — Fold

```rust
pub fn length_fold<T>(slice: &[T]) -> usize {
    slice.iter().fold(0, |acc, _| acc + 1)
}
```

## Rust — Recursive (naive)

```rust
pub fn length_recursive<T>(slice: &[T]) -> usize {
    match slice {
        [] => 0,
        [_, rest @ ..] => 1 + length_recursive(rest),
    }
}
```

## Rust — Tail-recursive style

```rust
pub fn length_tail<T>(slice: &[T]) -> usize {
    fn aux<T>(n: usize, slice: &[T]) -> usize {
        match slice {
            [] => n,
            [_, rest @ ..] => aux(n + 1, rest),
        }
    }
    aux(0, slice)
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Best approach | `List.length` or tail-recursive | `.len()` (O(1)) |
| Data structure | Linked list (no stored length) | Slice (fat pointer with length) |
| TCO | Guaranteed | Not guaranteed |
| Naive recursion | Stack overflow on large lists | Same risk |
| Fold | `List.fold_left` (tail-recursive) | `.fold()` (iterative) |

## Type Signatures

- **OCaml:** `val length : 'a list -> int`
- **Rust:** `fn length<T>(slice: &[T]) -> usize`

## Takeaways

1. Data structure choice dominates: slices carry their length; linked lists don't — this single difference makes the entire problem trivial in Rust
2. OCaml's TCO guarantee makes tail recursion a real optimization tool; in Rust, prefer iterators/fold instead
3. `fold` is the universal functional accumulator — same pattern in both languages, same safety
4. Rust's `usize` vs OCaml's `int`: unsigned vs signed return types for inherently non-negative values
5. The educational value is in seeing *why* the problem exists in OCaml (linked lists) and *why* it doesn't in Rust (slices)
