# OCaml vs Rust: List.filter — Select Elements by Predicate

## Side-by-Side Code

### OCaml
```ocaml
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds  = List.filter (fun x -> x mod 2 <> 0) numbers
```

### Rust (idiomatic)
```rust
pub fn filter_by<T, F>(items: &[T], predicate: F) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    items.iter().filter(|x| predicate(x)).copied().collect()
}
```

### Rust (functional/recursive)
```rust
pub fn filter_recursive<T, F>(list: &[T], predicate: &F) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    match list {
        [] => vec![],
        [head, tail @ ..] => {
            let mut rest = filter_recursive(tail, predicate);
            if predicate(head) {
                rest.insert(0, *head);
            }
            rest
        }
    }
}
```

### Rust (partition — two groups in one pass)
```rust
pub fn partition_by<T, F>(items: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    items.iter().partition(|x| predicate(x))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Filter function | `val filter : ('a -> bool) -> 'a list -> 'a list` | `fn filter_by<T, F>(items: &[T], predicate: F) -> Vec<T>` |
| Predicate type | `'a -> bool` | `F: Fn(&T) -> bool` |
| List type | `'a list` | `&[T]` (slice) |
| Result type | `'a list` (same linked list) | `Vec<T>` (heap-allocated, owned) |
| Two-way split | two `List.filter` calls | `Iterator::partition` — returns `(Vec<T>, Vec<T>)` |

## Key Insights

1. **Direct structural mapping:** `List.filter pred lst` becomes `.iter().filter(|x| pred(x)).copied().collect()` — same semantics, different syntax
2. **Closure double-reference:** Rust's `.filter()` passes `&&T` to the closure (iterator yields `&T`, filter wraps it in another `&`). Using `predicate(x)` where `x: &&T` works because `F: Fn(&T) -> bool` auto-derefs; alternatively, write `filter(|&&x| pred(x))` to unpack explicitly
3. **Recursive predicate must be `&F`:** Rust monomorphizes generics — a recursive function `filter_recursive<F: Fn…>` would create an infinite chain of distinct types at compile time. Passing `predicate: &F` breaks the cycle by erasing the recursive type instantiation
4. **`partition` avoids two passes:** OCaml's standard library has no single-pass split, so `List.filter` is called twice. Rust's `Iterator::partition` iterates once and routes each element into one of two `Vec`s — more cache-friendly and concise
5. **`Copy` bound vs cloning:** The `T: Copy` bound lets `.copied()` cheaply copy values out of iterator references. For non-`Copy` types, use `.cloned()` (requires `T: Clone`) or collect `Vec<&T>` references instead

## When to Use Each Style

**Use idiomatic Rust (`.filter()`) when:** you need a single filtered list from any iterator — it composes with map, take, skip, and other combinators naturally.

**Use `partition_by` when:** you need both the matching and non-matching elements — avoids traversing the input twice and keeps the code at the call site symmetrical.

**Use recursive Rust when:** teaching OCaml-to-Rust translation, demonstrating pattern matching on slices, or working through the structural recursion explicitly for educational purposes.
