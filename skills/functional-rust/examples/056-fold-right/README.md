# 056: Fold Right

**Difficulty:** 2  **Level:** Beginner

Process a list from right to left by recursing to the end, then combining on the way back.

## The Problem This Solves

Some operations are naturally right-associative. String concatenation from a list — `"a" ++ ("b" ++ ("c" ++ ""))` — is right-fold. Building a new list with the same order — prepending each element as you unwind — is right-fold. Any operation where the *last* element's result feeds into the second-to-last, and so on, needs `fold_right`.

`fold_left` accumulates left-to-right in a single pass. `fold_right` recurses to the end first, then combines on the way back. The two folds aren't interchangeable: `fold_right (+) [1;2;3] 0` gives the same answer for addition (commutative), but `fold_right (^) ["a";"b";"c"] ""` gives `"abc"` while a left fold gives `"cba"`.

The trade-off is stack space: right fold is not tail-recursive. Each recursive call stays on the stack until the base case is reached.

## The Intuition

Think of `fold_right f [a, b, c] init` as replacing every comma with `f` and the end with `init`:

```
[a, b, c]  →  f(a, f(b, f(c, init)))
```

The innermost call (`f(c, init)`) resolves first, its result flows outward into `f(b, ...)`, then into `f(a, ...)`. Right to left, outside-in.

For string concatenation: `f(a, f(b, f(c, ""))) = a ++ (b ++ (c ++ "")) = "abc"`. The right-fold structure produces the correct associativity automatically.

## How It Works in Rust

```rust
// Recursive right fold — mirrors OCaml's fold_right exactly
pub fn fold_right<T, A>(
    f: impl Fn(&T, A) -> A + Copy,
    xs: &[T],
    init: A,
) -> A {
    match xs {
        [] => init,
        [head, tail @ ..] => f(head, fold_right(f, tail, init)),
        //                          ^-- recurse first, combine after
    }
}

// Usage:
fold_right(|x, acc| x + acc, &[1,2,3], 0)             // → 6
fold_right(|s, acc: String| format!("{s}{acc}"), &["a","b","c"], String::new()) // → "abc"

// Rust's built-in right fold on slices:
xs.iter().rfold(init, |acc, &x| f(x, acc))
// rfold iterates backwards using index — no stack growth
```

The `Copy` bound on `f` lets the closure be copied into each recursive call rather than moved. The key structural difference from `fold_left`: the recursive call is *inside* `f(...)`, not in tail position, so the stack frame stays alive until the inner call returns.

## What This Unlocks

- **Right-associative operations** — string building, list copying, any operation where order of association matters.
- **Understanding fold duality** — knowing when to use `fold_right` vs `fold_left` is a fundamental FP skill.
- **`rfold` for safety** — Rust's `Iterator::rfold` gives right-fold semantics without stack growth, via internal loop.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Tail recursion | Not tail-recursive (stack-consuming) | Same — `fold_right` is stack-consuming |
| Safe alternative | `List.fold_right` in stdlib (same risk) | `Iterator::rfold` (loop-based, no stack growth) |
| Base case | `fold_right f [] init = init` | `[] => init` via slice pattern |
| Borrowing | Takes owned values from cons-list | Takes `&T` references from slice |
| `copy` of list | Structural sharing (free) | Allocates new `Vec` |
