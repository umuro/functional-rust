📖 **[View on hightechmind.io →](https://hightechmind.io/rust/264-iterator-take-while)**

---

# 264: Conditional Stopping with take_while()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Many algorithms need to consume elements from a sorted or ordered stream until a condition fails: reading log lines until a timestamp exceeds a threshold, parsing tokens until a delimiter is found, or collecting numbers until a negative value appears. The `take_while()` adapter makes this declarative — it yields elements while a predicate holds and stops the moment it first fails, discarding that element and all subsequent ones.

## Learning Outcomes

- Understand that `take_while(pred)` stops permanently at the first `false` — even if later elements would satisfy the predicate
- Distinguish `take_while()` from `filter()`: `take_while` is a prefix operation, not a global filter
- Use `take_while()` on infinite iterators to create bounded sequences
- Recognize the "first false terminates" semantics as essential for ordered data processing

## Rust Application

`Iterator::take_while(pred)` returns a `TakeWhile<I, P>` that calls `pred` on each element. At the first `false`, the iterator returns `None` for all future calls — even if subsequent elements would return `true`:

```rust
let result: Vec<i32> = [1, 2, 3, 4, 5].iter().copied()
    .take_while(|&x| x < 4)
    .collect();
// [1, 2, 3] — stops before 4, does not include 5

// Works with infinite iterators
let first_below_100: Vec<u64> = (0u64..)
    .map(|n| n * n)
    .take_while(|&sq| sq < 100)
    .collect();
// [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
```

## OCaml Approach

OCaml provides `List.take_while` via third-party libraries (like `Base` or `Core`), or it can be implemented recursively:

```ocaml
let rec take_while pred = function
  | [] -> []
  | x :: xs -> if pred x then x :: take_while pred xs else []
```

`Seq.take_while` exists for lazy sequences and has the same first-false-terminates semantics.

## Key Differences

1. **Prefix semantics**: Both Rust and OCaml's `take_while` are prefix operations — stopping permanently at the first false, unlike `filter` which examines all elements.
2. **Infinite sources**: Rust's `take_while()` is essential for consuming infinite iterators safely; OCaml's `Seq.take_while` serves the same purpose.
3. **Standard library**: `take_while` is built into Rust's `Iterator` trait; OCaml's standard `List` module lacks it (available in third-party libraries).
4. **Complementary**: `take_while(pred)` and `skip_while(pred)` are inverses — together they split an ordered sequence at the first failure point.

## Exercises

1. Parse a sequence of positive integers followed by a sentinel `-1` using `take_while()` to collect only the positive prefix.
2. Use `take_while()` on an infinite Fibonacci iterator to collect all Fibonacci numbers below one million.
3. Implement a simple CSV field parser that reads characters `take_while(|&c| c != ',')` from a character iterator.
