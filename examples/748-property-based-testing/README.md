# 748: Property-Based Testing (proptest Pattern)

**Difficulty:** 3  **Level:** Advanced

Instead of specific inputs, describe properties that must always hold — the framework generates random inputs and finds counterexamples, then shrinks them to the minimal failing case.

## The Problem This Solves

Example-based testing has a blind spot: you only test the inputs you think of. You test `sort([3,1,2])` but not `sort([])`, `sort([i32::MIN])`, or a 10,000-element list with duplicates. Bugs hide in the corners you didn't imagine.

Property-based testing flips the model. Instead of writing specific inputs, you describe *invariants* — facts that must be true for all valid inputs. "Sorting a list always produces a result of the same length." "The output is always non-decreasing." "Sorting twice equals sorting once." Then a generator creates hundreds of random inputs and checks each property.

When a property fails, the framework *shrinks* the counterexample: it systematically simplifies the failing input until it finds the minimal case that still fails. Instead of "your sort fails on this 47-element list", you get "your sort fails on `[-1, 0]`". That's dramatically easier to debug.

## The Intuition

In Python, the `hypothesis` library does this: you write `@given(st.lists(st.integers()))` and Hypothesis generates inputs. In JavaScript, `fast-check` is equivalent.

The real-world crate in Rust is [`proptest`](https://docs.rs/proptest). This example shows the *pattern* from scratch using a simple PRNG and an `Arbitrary` trait — so you understand what proptest does internally.

The core loop: pick a random input, check the property, if it fails, try smaller versions of the input, report the minimal failure.

## How It Works in Rust

```rust
// An Arbitrary type can generate random examples and shrink failures
trait Arbitrary: Sized + Clone + std::fmt::Debug {
    fn arbitrary(rng: &mut Lcg) -> Self;
    fn shrink(&self) -> Vec<Self> { vec![] }  // default: no shrinking
}

impl Arbitrary for Vec<i32> {
    fn arbitrary(rng: &mut Lcg) -> Self {
        let len = rng.next_usize_in(0, 20);
        (0..len).map(|_| i32::arbitrary(rng)).collect()
    }
    fn shrink(&self) -> Vec<Vec<i32>> {
        // Return simpler candidates: remove first, remove last, halve
        vec![self[1..].to_vec(), self[..self.len()-1].to_vec()]
    }
}

// The property runner: generate N inputs, check the predicate,
// shrink and report the minimal counterexample on failure
fn forall<T, F>(name: &str, tests: usize, mut prop: F) -> bool
where T: Arbitrary, F: FnMut(&T) -> bool
{
    let mut rng = Lcg::new(42);
    for i in 0..tests {
        let input = T::arbitrary(&mut rng);
        if !prop(&input) {
            // Shrink: find the simplest failing input
            let mut minimal = input.clone();
            loop {
                let smaller = minimal.shrink().into_iter().find(|c| !prop(c));
                match smaller {
                    Some(s) => minimal = s,
                    None    => break,
                }
            }
            eprintln!("✗ {name} failed after {} tests. Minimal: {:?}", i+1, minimal);
            return false;
        }
    }
    true
}

// Properties of sort — all three should hold for any input
fn my_sort(mut v: Vec<i32>) -> Vec<i32> { v.sort(); v }

// In tests:
#[test]
fn property_sort_idempotent() {
    assert!(forall::<Vec<i32>, _>("sort(sort(x)) == sort(x)",
        500, |v| my_sort(my_sort(v.clone())) == my_sort(v.clone())));
}

#[test]
fn property_sort_length_preserved() {
    assert!(forall::<Vec<i32>, _>("sort preserves length",
        500, |v| my_sort(v.clone()).len() == v.len()));
}
```

With the real `proptest` crate, this becomes:
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn sort_idempotent(v: Vec<i32>) {
        let sorted = { let mut v = v.clone(); v.sort(); v };
        let sorted_twice = { let mut s = sorted.clone(); s.sort(); s };
        assert_eq!(sorted, sorted_twice);
    }
}
```

## What This Unlocks

- **Find bugs you didn't know to look for**: edge cases with empty inputs, large values, duplicates, Unicode characters — generated automatically
- **Regression-proof sorting, parsing, encoding**: any function with a clear invariant is a candidate; especially powerful for `encode(decode(x)) == x` round-trip properties
- **Minimal counterexamples**: shrinking transforms a confusing 50-item failing case into the 2-item essence of the bug

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Property testing library | QCheck | `proptest` or `quickcheck` crates |
| Generator definition | `QCheck.Gen.t` | `Arbitrary` trait (or proptest `Strategy`) |
| Shrinking | Automatic via `QCheck.shrink` | Explicit `shrink()` method or proptest auto-shrink |
| Test macro | `QCheck.Test.make` | `proptest!` macro or manual `forall` |
| Seed control | `QCheck.Random.State` | Configurable; proptest saves failing seeds |
| Example this shows | N/A | Manual PRNG + Arbitrary — no crate needed |
