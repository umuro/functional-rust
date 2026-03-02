# Comparison: Reverse a List

## OCaml

```ocaml
let rev list =
  let rec aux acc = function
    | [] -> acc
    | h :: t -> aux (h :: acc) t
  in aux [] list
```

## Rust — Idiomatic (iterator)

```rust
pub fn reverse<T: Clone>(slice: &[T]) -> Vec<T> {
    slice.iter().rev().cloned().collect()
}
```

## Rust — Fold (mirrors OCaml accumulator)

```rust
pub fn reverse_fold<T: Clone>(slice: &[T]) -> Vec<T> {
    slice.iter().fold(Vec::new(), |mut acc, item| {
        acc.insert(0, item.clone());
        acc
    })
}
```

## Rust — Recursive

```rust
pub fn reverse_recursive<T: Clone>(slice: &[T]) -> Vec<T> {
    fn aux<T: Clone>(acc: Vec<T>, slice: &[T]) -> Vec<T> {
        match slice {
            [] => acc,
            [head, rest @ ..] => {
                let mut new_acc = vec![head.clone()];
                new_acc.extend(acc);
                aux(new_acc, rest)
            }
        }
    }
    aux(Vec::new(), slice)
}
```

## Rust — In-place (owned data)

```rust
pub fn reverse_in_place<T>(slice: &mut [T]) {
    slice.reverse();
}
```

## Comparison Table

| Aspect | OCaml | Rust (idiomatic) | Rust (in-place) |
|--------|-------|------------------|-----------------|
| Allocation | New list | New Vec | None |
| Complexity | O(n) | O(n) | O(n) |
| Ownership | Immutable | Borrows input | Mutates input |
| Trait needed | None | `Clone` | None |
| Prepend cost | O(1) cons | O(n) insert(0) | N/A |

## Type Signatures

- **OCaml:** `val rev : 'a list -> 'a list`
- **Rust:** `fn reverse<T: Clone>(slice: &[T]) -> Vec<T>`
- **Rust (in-place):** `fn reverse_in_place<T>(slice: &mut [T])`

## Takeaways

1. OCaml's cons (`::`) is O(1) prepend — perfect for accumulator-based reversal; Rust's Vec prepend is O(n) — use `rev()` iterator instead
2. Rust's `iter().rev()` is lazy — it changes direction without traversing, then `collect()` builds the result in one pass
3. In-place reversal (`slice.reverse()`) is uniquely Rust — OCaml's immutable lists can't be mutated
4. The `Clone` trait bound makes copying explicit — OCaml copies values implicitly during pattern matching
5. The accumulator pattern translates directly but isn't idiomatic Rust — iterators express the same intent more clearly
