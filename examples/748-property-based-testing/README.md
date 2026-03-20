📖 **[View on hightechmind.io →](https://hightechmind.io/rust/748-property-based-testing)**

---

# 748-property-based-testing — Property-Based Testing
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Example-based tests check specific inputs. Property-based testing checks invariants that must hold for all inputs by generating hundreds of random cases automatically. Pioneered by Haskell's QuickCheck (1999), this approach finds edge cases that humans miss: off-by-one errors, integer overflow, empty collections. The `proptest` and `quickcheck` Rust crates are production-grade; this example builds a minimal stdlib-only framework to teach the core ideas.

## Learning Outcomes

- Implement a deterministic LCG (Linear Congruential Generator) for reproducible random testing
- Create an `Arbitrary` trait with `arbitrary(rng)` and `shrink()` methods
- Write a `check` function that runs N random cases and reports the smallest failing input
- Understand shrinking: when a failure is found, find the smallest input that still fails
- Formulate mathematical properties: sort idempotency, reverse involution, commutativity of addition

## Rust Application

`Lcg` provides a seeded PRNG with `next_u64`, `next_i32_in`, and `next_usize_in`. The `Arbitrary` trait is implemented for `i32`, `Vec<i32>`, and `String`. The `check` function generates N values with `Arbitrary::arbitrary`, calls the predicate, and on failure tries to shrink by calling `shrink()` repeatedly. Properties tested include: `sort` is idempotent, `reverse(reverse(v)) == v`, and `(a + b) + c == a + (b + c)`.

## OCaml Approach

OCaml's `QCheck2` is a mature property-based testing library. It provides a `Gen.t` type for generators, `Test.make` for properties, and automatic shrinking. `QCheck.find_example` searches for an input satisfying a predicate — useful for finding edge cases. The `crowbar` library integrates with AFL fuzzing for coverage-guided property testing.

## Key Differences

1. **Shrinking**: Rust's `proptest` uses a sophisticated strategy-based shrinker; OCaml's `QCheck2` uses a similar but list-based approach. Both aim to find the minimal counterexample.
2. **Type-driven generation**: Rust derives `Arbitrary` implementations; OCaml's `QCheck` uses explicit generator values (combinators like `QCheck.int`, `QCheck.list`).
3. **Reproducing failures**: Both frameworks save the seed on failure so you can replay it; Rust uses `PROPTEST_SEED` env var, OCaml uses `QCheck.Test.get_seed`.
4. **Integration**: Rust's `proptest!` macro integrates naturally with `cargo test`; OCaml's `QCheck` requires an explicit runner call.

## Exercises

1. Implement `Arbitrary` for `HashMap<String, i32>` and write a property test that verifies `map.insert(k, v).get(k) == Some(v)` for all random keys and values.
2. Add a `check_stateful` function that tests state machine properties: generate random sequences of operations and verify the invariant holds after each step.
3. Write a property test for the `is_palindrome` function from example 744: for any string `s`, verify `is_palindrome(s + reverse(s))` is always true.
