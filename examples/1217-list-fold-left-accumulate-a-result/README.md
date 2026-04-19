# Example 1217: List Fold Left — Accumulate a Result

**Difficulty:** ⭐⭐
**Category:** Lists & HOF
**OCaml Source:** `List.fold_left` — OCaml standard library (`Stdlib.List`)

## Problem Statement

Walk a list left-to-right, threading an accumulator through a combining
function `f : 'acc -> 'elt -> 'acc`, and return the final accumulator.
Classic applications: sum, product, and building a labelled string from a
list of integers.

## Learning Outcomes

- Translating `List.fold_left` to Rust's `Iterator::fold` combinator.
- Seeing why the accumulator is passed **by value** and returned (pure fold), not mutated in place.
- How OCaml's operator-as-function (`(+)`, `( * )`) becomes a Rust closure `|acc, x| acc + x`.
- Left vs right associativity: subtraction and string concatenation both expose the direction.
- Tail-recursive pattern matching as a mirror of `let rec fold_left f acc = function ...`.

## OCaml Approach

`List.fold_left f init lst` starts with `init` and applies `f` successively
to the running accumulator and the next element: `f (f (f init x1) x2) x3`.
Operators curry and can be passed as values (`(+)`), so `List.fold_left (+) 0`
is already a summing function. `List.fold_left` is implemented tail-recursively
in the stdlib, so it runs in constant stack space.

## Rust Approach

Rust exposes the same shape as `Iterator::fold(init, f)`. The closure takes
the accumulator by value and returns the new one, so the type `A` need not be
`Copy` or `Clone`. For specialised cases (`sum`, `product`) the stdlib
iterator adapters already exist, but the generic `fold` is the direct
translation. A slice pattern match (`[] | [h, rest @ ..]`) mirrors the OCaml
recursive definition exactly, and the recursive call stays in tail position.

## Key Differences

1. **Accumulator ownership:** OCaml passes a value; Rust also passes `A` by value — the closure returns the new accumulator, so nothing is mutated aliasing-wise.
2. **Operators as functions:** OCaml writes `(+)` and `( * )`; Rust must wrap them in closures `|acc, x| acc + x`.
3. **String building:** OCaml's `^` copies; Rust prefers `String + &str` or `String::push_str` to avoid extra allocations.
4. **Tail recursion:** OCaml guarantees TCO; Rust does not, so `fold` (which loops internally) is preferred over the recursive variant for long lists.
