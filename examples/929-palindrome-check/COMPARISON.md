# Comparison: Palindrome Check

## OCaml — Using List.rev

```ocaml
let is_palindrome lst =
  lst = List.rev lst
```

## Rust — Idiomatic (index-based, zero allocation)

```rust
pub fn is_palindrome<T: PartialEq>(list: &[T]) -> bool {
    let n = list.len();
    (0..n / 2).all(|i| list[i] == list[n - 1 - i])
}
```

## Rust — Functional (iterator-based)

```rust
pub fn is_palindrome_iter<T: PartialEq>(list: &[T]) -> bool {
    list.iter().eq(list.iter().rev())
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Input type | `'a list` (linked list) | `&[T]` (slice reference) |
| Equality | Structural `=` (polymorphic) | `PartialEq` trait bound |
| Reversal | `List.rev` → new list (O(n) alloc) | `iter().rev()` → zero alloc |
| Access pattern | Sequential only | Random access O(1) |
| Memory | GC-managed | Borrowed reference |

## Type Signatures

- OCaml: `val is_palindrome : 'a list -> bool`
- Rust: `fn is_palindrome<T: PartialEq>(list: &[T]) -> bool`

## Takeaways

1. Rust's `DoubleEndedIterator` eliminates the need for explicit list reversal
2. Slice-based random access makes palindrome checking more efficient than linked-list traversal
3. OCaml's structural equality is convenient but less flexible than Rust's trait-based `PartialEq`
4. The iterator approach (`iter().eq(iter().rev())`) is the closest Rust analog to OCaml's style
5. Rust's zero-cost abstractions mean the iterator version compiles to the same code as manual indexing
