# Example 1125: List.partition — Split by Predicate

**Difficulty:** ⭐⭐
**Category:** Lists & HOF
**OCaml Source:** `List.partition`

## Problem Statement

Split a collection into two parts in a single traversal: elements that satisfy a predicate go into one group, and those that do not go into another. The function returns both groups simultaneously as a tuple, making it more efficient and expressive than two separate filter operations. This example implements a generic `partition` function and derives concrete domain helpers — splitting integers by a numeric threshold and strings by length — to show how the abstraction applies in practice.

## Learning Outcomes

- How OCaml's `List.partition pred xs` maps to Rust's `Iterator::partition`, and why both return a two-element tuple rather than requiring two passes
- How to return multiple collections simultaneously using tuple destructuring in both languages — and how nearly identical the destructuring syntax is across the two
- Why a single-pass partition is O(n) with constant overhead, while calling `filter` twice is also O(n) but doubles the work and requires the predicate to be called twice per element
- How to write a generic wrapper over Rust's iterator adapter, including the `Clone` bound needed to go from `&[T]` to owned `Vec<T>` output
- The difference between Rust's `Fn(&T) -> bool` predicate bound and OCaml's structurally-typed anonymous functions — and why Rust requires the bound to be spelled out

## OCaml Approach

OCaml's `List.partition` accepts a predicate and a list, and returns a pair `(matches, non_matches)` where both lists maintain the original relative order of elements. Internally it is implemented as a single `List.fold_left`, accumulating into two separate accumulators. The result is destructured with `let (small, big) = List.partition (fun x -> x <= 5) numbers` — pattern matching on the pair directly in the `let` binding. The predicate is an ordinary first-class function; no special syntax or trait is needed to pass it.

## Rust Application

Rust's standard library provides `Iterator::partition(|x| pred(x)) -> (Vec<T>, Vec<T>)` as a consuming iterator adapter that performs a single pass and builds two owned `Vec`s. The generic `partition<T: Clone, F>` wrapper in this example converts a borrowed `&[T]` to an iterator with `.iter().cloned()`, then delegates to the standard adapter. Concrete helpers `partition_threshold` and `partition_by_length` show how to specialize the generic function with a captured variable in the closure, demonstrating that closures over local variables work just as naturally in Rust as in OCaml. Tuple destructuring `let (small, big) = partition(...)` is syntactically identical in both languages.

## Key Differences

1. **Single pass vs. two filters:** Both `List.partition` and `Iterator::partition` traverse the collection once; two separate `filter` calls would traverse it twice and call the predicate on every element twice — important when the predicate is expensive or the collection is large.
2. **Tuple destructuring syntax:** OCaml: `let (small, big) = List.partition ...`; Rust: `let (small, big) = partition(...)` — the syntax is identical, which is a pleasant surprise when coming from OCaml.
3. **Predicate type:** OCaml accepts any function `'a -> bool` through structural typing; Rust requires the generic bound `F: Fn(&T) -> bool`, which must be declared explicitly in the function signature and is enforced by the compiler.
4. **Ownership and cloning:** OCaml lists share structure safely through garbage collection; Rust requires `T: Clone` to move values from a borrowed slice into two owned `Vec`s, making the allocation cost visible in the type signature.

## Exercises

1. Implement `partition_map` that applies a function returning `Result<A, B>` to each element and collects `Ok(a)` values into one `Vec<A>` and `Err(b)` values into another `Vec<B>`, in a single pass. This mirrors the OCaml `List.partition_map` added in OCaml 4.12.
2. Implement a three-way partition: given two thresholds `low` and `high`, return `(below, middle, above)` in a single traversal. Verify that the union of all three groups contains exactly the elements of the input with no duplicates or omissions.
3. Write a property-based test (or a parameterized Vitest-style test over many random inputs) that verifies `partition` and two separate `filter` calls always produce the same result, and that the concatenation of both output `Vec`s is a permutation of the input slice.
