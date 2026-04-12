# OCaml vs Rust: List.partition — Split by Predicate

## Side-by-Side Code

### OCaml
```ocaml
let numbers = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]
let (small, big) = List.partition (fun x -> x <= 5) numbers
```

### Rust (idiomatic)
```rust
pub fn partition<T: Clone, F>(items: &[T], pred: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    items.iter().cloned().partition(|x| pred(x))
}
```

### Rust (functional fold)
```rust
// Mirrors OCaml's accumulator-based implementation explicitly.
pub fn partition_fold<T: Clone, F>(items: &[T], pred: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    items.iter().fold((vec![], vec![]), |(mut yes, mut no), x| {
        if pred(x) { yes.push(x.clone()); } else { no.push(x.clone()); }
        (yes, no)
    })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| partition | `('a -> bool) -> 'a list -> 'a list * 'a list` | `fn partition<T: Clone, F>(&[T], F) -> (Vec<T>, Vec<T>)` |
| result type | `'a list * 'a list` (tuple) | `(Vec<T>, Vec<T>)` (tuple) |
| predicate | `'a -> bool` | `F: Fn(&T) -> bool` |
| destructuring | `let (small, big) = ...` | `let (small, big) = ...` |

## Key Insights

1. **Single-pass vs. two-filter**: Both OCaml and Rust `partition` traverse the list once and split into two output collections, which is more efficient than calling `filter` twice.
2. **Iterator adapter**: Rust's `Iterator::partition` is a built-in method that collects into a pair of collections — exactly matching OCaml's `List.partition` semantics.
3. **Tuple destructuring**: Both languages support `let (a, b) = partition(...)` for destructuring the result tuple — the syntax is nearly identical.
4. **Clone requirement**: Rust must `clone()` each element since ownership prevents moving items into two separate Vecs simultaneously. OCaml's GC shares list nodes freely.
5. **Predicate signature**: OCaml's predicate receives a value `'a`; Rust's receives a reference `&T` (since the iterator yields `&T` from a `&[T]` slice). The `cloned()` adapter materializes the `T` after filtering.

## When to Use Each Style

**Use `Iterator::partition` when:** you need to split a slice into two groups in a single pass — it's the most idiomatic and readable approach.
**Use the fold-based partition when:** you want to accumulate into more complex structures than a pair of `Vec`s (e.g., a `HashMap` of groups), or when teaching the fold-based derivation of `partition`.
