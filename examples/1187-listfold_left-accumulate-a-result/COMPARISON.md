# OCaml vs Rust: List.fold_left — Accumulate a Result

## Side-by-Side Code

### OCaml
```ocaml
let numbers = [1; 2; 3; 4; 5]

let sum     = List.fold_left ( + ) 0 numbers        (* 15  *)
let product = List.fold_left ( * ) 1 numbers        (* 120 *)
let max_val = List.fold_left max min_int numbers    (* 5   *)

(* Recursive definition — what fold_left actually does *)
let rec fold_left_rec f acc = function
  | []      -> acc
  | x :: xs -> fold_left_rec f (f acc x) xs
```

### Rust (idiomatic — std specialised folds)
```rust
pub fn sum_idiomatic(numbers: &[i64]) -> i64 {
    numbers.iter().sum()
}

pub fn product_idiomatic(numbers: &[i64]) -> i64 {
    numbers.iter().product()
}

pub fn max_idiomatic(numbers: &[i64]) -> Option<i64> {
    numbers.iter().copied().max()
}
```

### Rust (functional — generic fold_left mirroring OCaml)
```rust
pub fn fold_left<T, Acc, F>(f: F, init: Acc, list: &[T]) -> Acc
where
    F: Fn(Acc, &T) -> Acc,
{
    list.iter().fold(init, |acc, x| f(acc, x))
}
```

### Rust (recursive fold — explicit recursion as in OCaml)
```rust
pub fn fold_left_rec<T, Acc, F>(f: &F, init: Acc, list: &[T]) -> Acc
where
    F: Fn(Acc, &T) -> Acc,
{
    match list {
        [] => init,
        [head, tail @ ..] => fold_left_rec(f, f(init, head), tail),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| fold signature | `val fold_left : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a` | `fn fold_left<T, Acc, F: Fn(Acc, &T) -> Acc>(f: F, init: Acc, list: &[T]) -> Acc` |
| List type | `'b list` | `&[T]` (slice) |
| Accumulator | `'a` (any type) | `Acc` (generic) |
| Max result | `int` (unsafe with `min_int`) | `Option<i64>` (safe) |
| Operator section | `( + )` as a value | `\|a, &b\| a + b` closure |

## Key Insights

1. **`Iterator::fold` is `List.fold_left`** — the argument order differs (`init` first in both, but closure parameters are `(acc, elem)` vs OCaml's `f acc elem`), the semantics are identical: left-associative accumulation.

2. **No sentinel values** — OCaml needs `min_int` as the identity for max (a footgun: fails on empty list with wrong answer). Rust returns `Option<T>` from `.max()`, forcing callers to handle the empty case explicitly.

3. **Operator sections vs closures** — OCaml treats `( + )` as a first-class value of type `int -> int -> int`. Rust lacks operator sections; you write `|a, &b| a + b` or use `i64::wrapping_add` as a function pointer. The closure version is often clearer anyway.

4. **Specialised folds via traits** — Rust's `Sum` and `Product` traits let `.sum()` and `.product()` work for any numeric type without specifying a function. This is more ergonomic than `fold_left ( + ) 0` for the common cases.

5. **Slice pattern recursion** — `[head, tail @ ..]` in Rust mirrors OCaml's `x :: xs` destructuring. Both produce tail-recursive-style decomposition; Rust's version is not tail-call-optimised by the compiler, but for typical list sizes this is fine.

## When to Use Each Style

**Use idiomatic Rust (`.sum()` / `.product()` / `.max()`)** when reducing with a standard numeric operation — it communicates intent directly and the compiler can optimise it well.

**Use `Iterator::fold` with a closure** when the reduction logic is custom (e.g. building a `HashMap`, accumulating a string, computing a running statistic).

**Use the recursive `fold_left_rec`** when teaching the OCaml→Rust translation or when the recursive structure itself matters for reasoning about correctness.
