# Comparison: fold_right — OCaml vs Rust

## OCaml — Direct Recursion

```ocaml
let rec fold_right f lst acc =
  match lst with
  | []     -> acc
  | h :: t -> f h (fold_right f t acc)

let sum  lst = fold_right ( + ) lst 0
let prod lst = fold_right ( * ) lst 1
let cat  lst = fold_right ( ^ ) lst ""
```

## Rust — Idiomatic (Iterator Methods)

```rust
pub fn sum_idiomatic(xs: &[i64]) -> i64 {
    xs.iter().sum()
}

pub fn product_idiomatic(xs: &[i64]) -> i64 {
    xs.iter().product()
}

pub fn concat_idiomatic(xs: &[&str]) -> String {
    xs.iter().copied().collect()
}
```

## Rust — Functional (Recursive fold_right)

```rust
pub fn fold_right<T, A>(f: impl Fn(&T, A) -> A + Copy, xs: &[T], init: A) -> A {
    match xs {
        [] => init,
        [head, tail @ ..] => f(head, fold_right(f, tail, init)),
    }
}

pub fn sum_functional(xs: &[i64]) -> i64 {
    fold_right(|x, acc| x + acc, xs, 0)
}
```

## Rust — rfold (Built-in Right Fold)

```rust
pub fn sum_rfold(xs: &[i64]) -> i64 {
    xs.iter().rfold(0, |acc, &x| x + acc)
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type signature | `('a -> 'b -> 'b) -> 'a list -> 'b -> 'b` | `fn fold_right<T, A>(f: impl Fn(&T, A) -> A, xs: &[T], init: A) -> A` |
| Data structure | Cons-list (recursive) | Slice (contiguous memory) |
| Stack safety | Can overflow on large lists | Recursive version can overflow; `rfold` is safe |
| Element access | Pattern match `h :: t` | Slice pattern `[head, tail @ ..]` |
| Ownership | Values shared/copied implicitly | `&T` references borrowed from slice |
| Partial application | `let sum = fold_right (+)` | Closures: `\|x, acc\| x + acc` |
| Built-in | `List.fold_right` | `Iterator::rfold` |

## Type Signatures Explained

**OCaml:** `val fold_right : ('a -> 'b -> 'b) -> 'a list -> 'b -> 'b`
- `'a` and `'b` are type variables (generics)
- Takes: a function, a list, and an initial accumulator
- The function takes an element and accumulator, returns new accumulator

**Rust:** `fn fold_right<T, A>(f: impl Fn(&T, A) -> A + Copy, xs: &[T], init: A) -> A`
- `T` and `A` are generic type parameters
- `&T`: borrows elements (OCaml copies)
- `impl Fn`: accepts any callable (closure, function pointer)
- `+ Copy`: needed because the closure is called recursively

## Takeaways

1. **fold_right is natural in OCaml** because lists are recursive — the fold mirrors the data structure
2. **Rust prefers iterators** — `rfold` achieves the same semantics without stack risk
3. **Borrowing changes the API** — Rust's fold_right takes `&T` where OCaml takes `'a`
4. **Partial application** is lightweight in OCaml; Rust uses closures for the same effect
5. **The recursive Rust version** is primarily pedagogical — in production, use `rfold` or `fold`
