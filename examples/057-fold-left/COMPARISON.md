# Comparison: fold_left — OCaml vs Rust

## OCaml — Tail-Recursive

```ocaml
let rec fold_left f acc = function
  | []     -> acc
  | h :: t -> fold_left f (f acc h) t

let sum     lst = fold_left ( + ) 0 lst
let product lst = fold_left ( * ) 1 lst
let maximum lst = fold_left max (List.hd lst) (List.tl lst)
let reverse lst = fold_left (fun acc x -> x :: acc) [] lst
```

## Rust — Idiomatic

```rust
pub fn sum_idiomatic(xs: &[i64]) -> i64 { xs.iter().sum() }
pub fn product_idiomatic(xs: &[i64]) -> i64 { xs.iter().product() }
pub fn maximum_idiomatic(xs: &[i64]) -> Option<i64> { xs.iter().copied().max() }
pub fn reverse_idiomatic(xs: &[i64]) -> Vec<i64> {
    let mut v = xs.to_vec();
    v.reverse();
    v
}
```

## Rust — Functional (Custom fold_left)

```rust
pub fn fold_left<T, A>(f: impl Fn(A, &T) -> A, mut acc: A, xs: &[T]) -> A {
    for x in xs { acc = f(acc, x); }
    acc
}

pub fn maximum_functional(xs: &[i64]) -> Option<i64> {
    let (&first, rest) = xs.split_first()?;
    Some(fold_left(|acc, &x| if x > acc { x } else { acc }, first, rest))
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type signature | `('b -> 'a -> 'b) -> 'b -> 'a list -> 'b` | `fn fold_left<T, A>(f: Fn(A, &T) -> A, acc: A, xs: &[T]) -> A` |
| Tail-call optimization | Guaranteed by compiler | Not guaranteed (we use a loop instead) |
| Empty list max | `List.hd` raises exception | Returns `None` (safe) |
| Reverse mechanism | Cons to front: `x :: acc` | `v.reverse()` in-place or `insert(0, x)` |
| Built-in | `List.fold_left` | `Iterator::fold` |
| Accumulator | Passed by value (GC'd) | Moved by ownership |

## Type Signatures Explained

**OCaml:** `val fold_left : ('b -> 'a -> 'b) -> 'b -> 'a list -> 'b`
- Accumulator type `'b` comes first in the function parameter
- The accumulator is threaded through each recursive call

**Rust:** `fn fold_left<T, A>(f: impl Fn(A, &T) -> A, acc: A, xs: &[T]) -> A`
- `A` is the accumulator type, `T` is the element type
- `&T`: elements are borrowed from the slice
- `mut acc`: the accumulator is mutably rebound on each iteration

## Takeaways

1. **fold_left is Rust's natural fold** — `Iterator::fold` processes left to right, matching fold_left's semantics exactly
2. **Safety improvement:** Rust's `maximum` returns `Option` instead of panicking on empty input
3. **No TCO needed:** Rust's `for` loop is already iterative — no tail-call optimization concern
4. **Ownership makes fold natural** — the accumulator is moved into each step, not shared
5. **OCaml's `function` keyword** combines `fun` + `match`; Rust uses `match` explicitly or `for` loops
