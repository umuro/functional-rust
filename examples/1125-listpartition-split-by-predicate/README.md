# List.partition — Split by Predicate

## Problem Statement
Split a list into two parts in a single pass: elements that satisfy a predicate and those that don't. Implement `partition` generically and derive concrete helpers for numbers and strings.

## Learning Outcomes
- How OCaml's `List.partition pred xs` maps to Rust's `Iterator::partition`
- Return two collections simultaneously — tuples as multiple return values in both languages
- The efficiency of a single-pass partition vs. two separate filter operations

## Rust Application
`Iterator::partition(|x| pred(x)) -> (Vec<T>, Vec<T>)` splits in one pass. The concrete functions `partition_threshold` and `partition_by_length` show domain-specific usage. The generic `partition<T: Clone, F>` wraps the iterator method for slice inputs.

## OCaml Approach
`List.partition (fun x -> x <= 5) numbers` returns `(small, big)` as a tuple. The traversal is a single `List.fold_left` internally. OCaml's pattern `let (small, big) = ...` destructures the result directly.

## Key Differences
1. **Single pass:** Both implementations traverse the list once — O(n) with constant overhead vs. two separate `filter` calls
2. **Tuple destructuring:** OCaml: `let (small, big) = partition ...`; Rust: `let (small, big) = partition(...)` — identical syntax
3. **Predicate ownership:** OCaml closures capture by reference freely; Rust closures require the predicate to implement `Fn(&T) -> bool`, which may require explicit lifetime annotations

## Exercises
1. Implement `partition_map` that applies a function returning `Result<A, B>` and collects `Ok` values and `Err` values into separate `Vec`s
2. Implement a three-way partition: `(low, mid, high)` based on two thresholds using a single pass
3. Verify that `partition` and two separate `filter` calls always produce the same result (write a property-based test or parameterized test)
