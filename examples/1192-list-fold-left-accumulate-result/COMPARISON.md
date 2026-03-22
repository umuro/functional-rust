# OCaml vs Rust: List.fold_left — Accumulate a Result

## Side-by-Side Code

### OCaml

```ocaml
(* Idiomatic: operator sections as first-class functions *)
let numbers = [1; 2; 3; 4; 5]
let sum     = List.fold_left ( + ) 0 numbers        (* 15 *)
let product = List.fold_left ( * ) 1 numbers        (* 120 *)
let max_val = List.fold_left max min_int numbers    (* 5 *)

(* Recursive: tail-recursive fold showing the accumulator threading *)
let rec fold_left_rec f acc = function
  | [] -> acc
  | x :: rest -> fold_left_rec f (f acc x) rest
```

### Rust (idiomatic)

```rust
pub fn sum(numbers: &[i64]) -> i64 {
    numbers.iter().fold(0, |acc, &x| acc + x)
}

pub fn product(numbers: &[i64]) -> i64 {
    numbers.iter().fold(1, |acc, &x| acc * x)
}

// Returns None for empty slice — no sentinel value needed
pub fn max_val(numbers: &[i64]) -> Option<i64> {
    numbers.iter().copied().reduce(|a, b| a.max(b))
}
```

### Rust (functional/recursive)

```rust
pub fn fold_left_recursive<T, U, F>(items: &[T], acc: U, f: &F) -> U
where
    F: Fn(U, &T) -> U,
{
    match items {
        [] => acc,
        [head, rest @ ..] => {
            let new_acc = f(acc, head);
            fold_left_recursive(rest, new_acc, f)
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| fold signature | `val fold_left : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a` | `fn fold_left<T, U, F: Fn(U, &T) -> U>(items: &[T], init: U, f: F) -> U` |
| operator section | `( + )` as `int -> int -> int` | closure `\|acc, &x\| acc + x` |
| maximum (empty-safe) | `List.fold_left max min_int` → may return `min_int` | `reduce(\|a, b\| a.max(b))` → `Option<i64>` |
| input type | `'a list` | `&[T]` (borrowed slice) |

## Key Insights

1. **Direct semantic equivalent:** `List.fold_left f acc xs` in OCaml and `iter.fold(init, f)` in Rust compute exactly the same thing — a left fold applying `f` to the accumulator and each element in order. The algorithm is identical; only the syntax differs.
2. **Operator sections vs. closures:** OCaml's `( + )` lifts a binary infix operator into a two-argument function. Rust has no operator-section syntax, so you write explicit closures. This is more verbose but makes the binding explicit — there is no ambiguity about whether the operator is curried or takes both arguments at once.
3. **Empty-list semantics:** `List.fold_left max min_int` returns `min_int` for an empty list — a sentinel that silently propagates through further computation. Rust's `reduce` returns `None`, making the empty case a type-level distinction. This catches bugs at the call site rather than at runtime.
4. **Tail recursion:** OCaml's `List.fold_left` is tail-recursive in the standard library implementation, so it can process lists of any length without stack overflow. Rust's recursive `fold_left_recursive` is also tail-recursive because the recursive call is the last expression, but Rust does not currently guarantee tail-call optimization — use `Iterator::fold` in production.
5. **Argument order:** OCaml: `fold_left f acc list` (function, accumulator, list). Rust: `iter.fold(init, f)` (accumulator, function as method argument on the iterator). Both pass the accumulator before the combining function at the conceptual level, but the positions differ syntactically.

## When to Use Each Style

**Use idiomatic Rust when:** Summing, multiplying, or otherwise reducing a slice — `iter.fold(init, f)` is a zero-allocation, inlined operation in release builds.
**Use recursive Rust when:** Teaching the OCaml parallel or processing a custom algebraic type where `Iterator` is not available. Note: Rust does not guarantee TCO, so avoid deep recursion over large inputs.
