📖 **[View on hightechmind.io →](https://hightechmind.io/rust/076-gcd-lcm)**

---

# 076 — GCD and LCM (Ownership Focus)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

This example revisits GCD and LCM (see also example 071) with an explicit focus on ownership semantics. Since GCD operates on `u64` (a `Copy` type), ownership is trivial — values are copied freely. This makes GCD a clean example for understanding when Rust's ownership system has zero friction: primitive types are `Copy`, so they move like values in any other language.

The ownership-focused presentation shows that Rust's borrow checker is not an obstacle for numeric code — `Copy` types eliminate the entire class of ownership errors that arise with heap-allocated types.

## Learning Outcomes

- Understand why ownership is trivial for `Copy` types (integers)
- Use `reduce` to apply a binary operation across a collection
- Implement `gcd_iter` accepting `impl IntoIterator` for maximum flexibility
- Recognize that `gcd` and `lcm` are commutative, associative operations suitable for `reduce`
- Connect to number theory: GCD as the basis for fraction simplification and coprimeness

## Rust Application

`gcd(a, b)` is a tail-recursive Euclidean algorithm. All values are `u64` (`Copy`), so there is no move/borrow complexity. `lcm` uses `a / gcd(a, b) * b` to avoid overflow. `gcd_list` uses `nums.iter().copied().reduce(gcd).unwrap_or(0)` — `reduce` applies `gcd` pairwise. `gcd_iter` accepts any `IntoIterator<Item=u64>` — slices, ranges, or custom iterators.

## OCaml Approach

OCaml: `let rec gcd a b = if b = 0 then a else gcd b (a mod b)`. All integers are value types in OCaml too — no ownership issues. `List.fold_left gcd 0 lst` computes GCD of a list (using `gcd(0, x) = x` as the identity). `List.fold_left lcm 1 lst` computes LCM (using `lcm(1, x) = x`).

## Key Differences

1. **`Copy` eliminates ownership friction**: With `Copy` types, Rust code looks identical to OCaml code for pure numeric algorithms. Ownership only matters for heap-allocated types.
2. **`reduce` vs `fold`**: Rust's `reduce` uses the first element as the initial accumulator. OCaml's `fold_left gcd 0` uses 0 (the identity for GCD). Both compute the GCD of the collection.
3. **`IntoIterator` generality**: Rust's `gcd_iter(impl IntoIterator<Item=u64>)` works with slices, `Vec`, ranges, and custom iterators. OCaml's `List.fold_left gcd 0` works only with lists.
4. **Overflow prevention**: `a / gcd * b` vs `a * b / gcd` — the first form avoids intermediate overflow. Both give the same mathematical result for valid inputs.

## Exercises

1. **Coprime check**: Write `are_coprime(a: u64, b: u64) -> bool` using GCD. Prove that `gcd(a, b) = 1` iff a and b are coprime. Use this for RSA key validation.
2. **Totient function**: Write `totient(n: u64) -> u64` (Euler's totient function) counting integers from 1 to n that are coprime to n. Use `(1..=n).filter(|&k| are_coprime(k, n)).count()`.
3. **Farey sequence**: Generate the Farey sequence F_n: all fractions p/q with 0 ≤ p ≤ q ≤ n and gcd(p,q) = 1, in ascending order. Use GCD to filter to reduced fractions.
