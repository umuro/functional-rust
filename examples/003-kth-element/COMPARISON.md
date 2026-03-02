# Comparison: K-th Element

## OCaml

```ocaml
let rec at k = function
  | [] -> None
  | h :: t -> if k = 1 then Some h else at (k - 1) t
```

## Rust — Idiomatic (safe indexing)

```rust
pub fn at<T>(slice: &[T], index: usize) -> Option<&T> {
    slice.get(index)
}
```

## Rust — 1-based (matching OCaml)

```rust
pub fn at_one_based<T>(slice: &[T], k: usize) -> Option<&T> {
    if k == 0 { None } else { slice.get(k - 1) }
}
```

## Rust — Functional (recursive)

```rust
pub fn at_recursive<T>(slice: &[T], k: usize) -> Option<&T> {
    match (slice, k) {
        ([], _) => None,
        ([h, ..], 1) => Some(h),
        ([_, rest @ ..], k) => at_recursive(rest, k - 1),
    }
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Indexing | 1-based | 0-based (idiomatic) |
| Data structure | `'a list` (linked) | `&[T]` (contiguous) |
| Access time | O(k) | O(1) |
| Return | `'a option` (copy) | `Option<&T>` (borrow) |
| Bounds check | Pattern match | `.get()` returns `None` |

## Type Signatures

- **OCaml:** `val at : int -> 'a list -> 'a option`
- **Rust:** `fn at<T>(slice: &[T], index: usize) -> Option<&T>`

## Takeaways

1. Rust's `.get()` is a one-liner that replaces OCaml's entire recursive function — contiguous memory wins
2. The 0-based vs 1-based convention matters: `usize` subtraction can panic on underflow in Rust
3. OCaml's `int` allows negative indices (returns `None`); Rust's `usize` is unsigned — different safety properties
4. Recursive slice matching works but is purely educational — `.get()` is always preferred in production Rust
5. Both languages encode "might not exist" in the type system (`Option`/`option`) rather than throwing exceptions
