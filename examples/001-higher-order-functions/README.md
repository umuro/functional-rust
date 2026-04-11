📖 **[View on hightechmind.io →](https://hightechmind.io/rust/001-higher-order-functions)**

---

# 001 — Higher-Order Functions

## Problem Statement

Higher-order functions (HOFs) are the foundational abstraction of functional programming, rooted in lambda calculus from the 1930s. The core insight is that functions are values: they can be passed as arguments, returned from other functions, and stored in data structures. This eliminates entire categories of boilerplate loops.

The three pillars — `map`, `filter`, and `fold` — capture the three fundamental patterns of list processing: transforming every element, selecting some elements, and reducing a collection to a single value. Every non-trivial data pipeline in any language is a composition of these three operations. They appear in NumPy, Spark, SQL (`SELECT`, `WHERE`, `GROUP BY`), Hadoop MapReduce, and every stream-processing library.

HOFs eliminate the need for hand-written loops for 90% of data-processing tasks. When you see a Python `for` loop building a result list, it almost always hides a `map` or `filter`. Making this abstraction explicit enables parallel execution (data parallelism in Rayon uses the same `map`/`filter`/`fold` API), lazy evaluation, and composable pipelines that are impossible with raw loops.

## Learning Outcomes

- Understand why functions-as-values eliminate boilerplate iteration
- Use `map`, `filter`, and `fold`/`reduce` on slices and iterators
- Implement recursive versions to understand the underlying mechanics
- Chain multiple higher-order functions into a single expressive pipeline
- Recognize when to use closures (`|x| ...`) vs named functions

- Distinguish between `fold_left` (left-to-right, tail-recursive in OCaml) and `fold_right` (right-to-left, not safe for large lists)
- Understand that Rust's `Iterator::fold` is always left-to-right and implemented as a loop

- Recognize that Rust's `Iterator` methods are zero-cost abstractions — the compiler inlines and optimizes them as efficiently as hand-written loops

## Rust Application

Rust's `Iterator` trait makes HOFs first-class on any collection. `double_all` uses `.map(|&x| x * 2)`, `evens` uses `.filter(|&&x| x % 2 == 0)`, and `sum` uses `.fold(0, |acc, &x| acc + x)`. The manual recursive implementations `my_map`/`my_filter`/`my_fold` show the structural recursion underlying these operations. The chained version `sum_of_doubled_evens` composes `filter + map + sum` into a single lazy pipeline — no intermediate allocations until `.sum()` drives the chain.

The double-dereference issue (`&&x`) arises because `.iter()` yields references into the slice (`&i32`), and closures borrow them again. Using `.copied()` before `.filter()` clears this up for `Copy` types. The `sum_of_doubled_evens` pipeline is fully lazy: no allocation occurs until `.sum()` consumes the iterator — a key performance advantage over creating intermediate `Vec`s.

## OCaml Approach

OCaml's `List.map`, `List.filter`, and `List.fold_left`/`List.fold_right` are the direct equivalents. Functions are curried by default, so `List.map (fun x -> x * 2)` partially applies to produce a new function waiting for its list argument. The `|>` pipe operator enables `list |> List.filter even |> List.map double |> List.fold_left (+) 0` in natural reading order.

The distinction between `List.fold_left` (tail-recursive, safe) and `List.fold_right` (not tail-recursive) is a critical OCaml gotcha: always prefer `fold_left` for large lists. Similarly, Rust's `Iterator::fold` is always left-to-right and implemented as a loop with no stack risk.

## Key Differences

1. **Currying**: OCaml functions are automatically curried; `List.map f` returns a new function. Rust requires explicit closures: `v.iter().map(|&x| f(x))`.
2. **Laziness**: Rust iterators are lazy — `filter().map()` does nothing until consumed by `.collect()` or `.sum()`. OCaml's `List.map` is eager and allocates immediately.
3. **Ownership**: Rust's `iter()` yields references (`&T`); you must use `copied()` or `cloned()` to get owned values. OCaml lists are garbage-collected so this distinction does not exist.
4. **Double-dereference**: Iterating a `&[i32]` yields `&&i32` inside closures, requiring `&&x` patterns. OCaml pattern matching on lists always operates on direct values.

## Exercises

1. **Compose map and filter**: Write `positive_doubled(nums: &[i32]) -> Vec<i32>` that keeps only positive numbers and doubles them, using a single iterator chain with no intermediate `Vec`.
2. **Fold-based map**: Re-implement `my_map` using only `fold` (no recursion or explicit loops) to understand how fold generalizes all list operations.
3. **Running totals**: Write `running_sum(nums: &[i32]) -> Vec<i32>` that returns prefix sums `[1, 3, 6, 10, ...]` using Rust's `.scan()` — the stateful fold variant that emits intermediate accumulator values.

4. **Parallel map**: Explore how `rayon`'s `.par_iter().map(f).collect()` has the same interface as sequential `.iter().map(f).collect()` but executes in parallel. Write a function signature that works with both.
5. **Custom filter combinator**: Implement `filter_map(f: impl Fn(&T) -> Option<U>, slice: &[T]) -> Vec<U>` which combines filtering and mapping in one pass — equivalent to OCaml's `List.filter_map`.
