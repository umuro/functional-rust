[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 087 — Difference of Squares
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Compute the difference between the square of the sum and the sum of the squares of the first `n` natural numbers. Implement both an iterator-based O(n) version and a closed-form O(1) version using the formulas `(n(n+1)/2)²` and `n(n+1)(2n+1)/6`. Compare with OCaml's `List.init` and `List.fold_left` approach.

## Learning Outcomes

- Use `(1..=n).sum()` for range summation and `.map(|x| x * x).sum()` for sum of squares
- Understand Rust's `..=` inclusive range syntax
- Compare iterator-based O(n) computation with O(1) closed-form formulas
- Use the Gauss formula `n(n+1)/2` and squares formula `n(n+1)(2n+1)/6`
- Map Rust's `(1..=n)` range to OCaml's `List.init n (fun i -> i + 1)`
- Verify both implementations agree with property-based reasoning

## Rust Application

`square_of_sum(n)` collects `(1..=n).sum()` into a `u64`, then squares it. `sum_of_squares(n)` maps each element `x` to `x * x` and sums — lazy chain with zero intermediate allocation. `difference` is simply `square_of_sum(n) - sum_of_squares(n)`. The formula versions replace the ranges with arithmetic: `n * (n + 1) / 2` for the sum and `n * (n + 1) * (2 * n + 1) / 6` for the sum of squares. Both approaches produce identical results; the formula version scales to large `n` without iteration.

## OCaml Approach

`List.init n (fun i -> i + 1)` creates `[1; 2; …; n]`. `List.fold_left (+) 0` sums it. `List.fold_left (fun acc x -> acc + x * x) 0` sums squares. The closed-form formulas are identical arithmetic. OCaml's `List.init` is slightly more verbose than Rust's range iterator but maps naturally to the mathematical definition.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Range | `(1..=n)` | `List.init n (fun i -> i+1)` |
| Sum | `.sum()` | `List.fold_left (+) 0` |
| Sum of squares | `.map(|x| x*x).sum()` | `List.fold_left (fun acc x -> acc + x*x) 0` |
| Closed form | Arithmetic on `u64` | Same arithmetic |
| Overflow | Possible for large `n` with `u64` | Same |
| Laziness | Iterator chain is lazy | `List.init` is eager (allocates list) |

The two implementations illustrate a key lesson: recognise when a loop can be replaced by a closed-form expression. For this problem the O(1) formula is strictly better; the iterator version is shown to demonstrate the connection between the mathematical definition and the code.

## Exercises

1. Prove algebraically that `(n(n+1)/2)² - n(n+1)(2n+1)/6` simplifies to `n(n-1)(n+1)(3n+2)/12`.
2. Use `u128` instead of `u64` to handle larger inputs without overflow.
3. Write a property-based test that verifies `difference_formula(n) == difference(n)` for `n` in 1..=100.
4. Generalise to `difference_k(n, k)`: square of the sum of `k`th powers minus sum of `k`th-power squares.
5. In OCaml, implement the closed-form version and measure execution time for `n = 10_000_000` against the `List.init` version.
