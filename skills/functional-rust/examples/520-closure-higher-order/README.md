# 520: Higher-Order Functions

**Difficulty:** 2  **Level:** Beginner-Intermediate

Functions that take or return other functions — the backbone of Rust's iterator API and functional programming style.

## The Problem This Solves

Without higher-order functions, every data transformation is a loop with slightly different body. Want the sum of squares of even numbers? One loop. Want the maximum after a transformation? Another loop. Want to group by a property? Yet another loop. The structure is identical; only the inner logic changes.

Higher-order functions (HOFs) extract the structure and let you supply only what varies. `filter(is_even)` + `map(square)` + `sum()` replaces a hand-written loop, and each step is independently testable and named.

The secondary problem: without lazy HOFs, each step allocates an intermediate collection. Rust's iterator HOFs are lazy — they fuse into a single loop at compile time, eliminating intermediate allocations entirely.

## The Intuition

Higher-order functions treat behavior as data. `map` is a machine that applies your transformation to each element. `filter` is a machine that applies your predicate. `fold` is a machine that applies your accumulator. You supply the behavior; they supply the infrastructure.

Python has `map()`, `filter()`, and `functools.reduce()`. JavaScript has `Array.map()`, `.filter()`, `.reduce()`. Rust's iterator methods are the same, but with a crucial difference: they're **lazy**. Nothing computes until you call a consuming adapter like `.collect()` or `.sum()`.

Laziness means `(1..1_000_000).filter(is_prime).take(5)` only computes until it finds 5 primes — it doesn't check all million numbers first.

## How It Works in Rust

```rust
let nums: Vec<i32> = (1..=10).collect();

// map: transform each element
let squares: Vec<i32> = nums.iter().map(|&x| x * x).collect();

// filter: keep elements matching predicate
let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).copied().collect();

// fold: accumulate (the general HOF — map and filter are special cases)
let sum: i32 = nums.iter().fold(0, |acc, &x| acc + x);

// chained pipeline — LAZY: single pass, no intermediate allocations
let result: i32 = nums.iter()
    .filter(|&&x| x % 2 == 0)   // keep evens
    .map(|&x| x * x)             // square them
    .sum();                       // accumulate

// flat_map: map then flatten (like Python's chain of map with list results)
let pairs: Vec<i32> = [1, 2, 3].iter()
    .flat_map(|&x| [x, x * 10])  // each element becomes two elements
    .collect();  // [1, 10, 2, 20, 3, 30]

// any/all: short-circuit HOFs
println!("{}", nums.iter().any(|&x| x > 5));  // true (stops at first match)
println!("{}", nums.iter().all(|&x| x > 0));  // true

// Custom HOF: zip two slices with a combining function
fn zip_with<A, B, C, F: Fn(&A, &B) -> C>(a: &[A], b: &[B], f: F) -> Vec<C> {
    a.iter().zip(b.iter()).map(|(x, y)| f(x, y)).collect()
}
let sums = zip_with(&[1, 2, 3], &[10, 20, 30], |x, y| x + y); // [11, 22, 33]

// Custom HOF: scan (running totals — all intermediate fold values)
fn scan_left<T: Clone, U: Clone, F: Fn(U, &T) -> U>(
    items: &[T], init: U, f: F
) -> Vec<U> {
    let mut acc = init.clone();
    std::iter::once(init)
        .chain(items.iter().map(move |item| { acc = f(acc.clone(), item); acc.clone() }))
        .collect()
}
```

## What This Unlocks

- **Zero-allocation pipelines** — chain `map`, `filter`, `flat_map`, `take_while` without intermediate `Vec`s; the compiler fuses them into one loop.
- **Readable data transformations** — express "even squares less than 50" as a pipeline that reads like English rather than nested loops.
- **Custom iterator adapters** — write your own `zip_with`, `scan`, or `group_by` as HOFs that integrate seamlessly with the iterator ecosystem.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map | `List.map f xs` | `iter.map(f).collect()` |
| Filter | `List.filter pred xs` | `iter.filter(pred).collect()` |
| Fold | `List.fold_left f init xs` | `iter.fold(init, f)` |
| Flat map | `List.concat_map f xs` | `iter.flat_map(f).collect()` |
| Laziness | Eager by default | Lazy by default — consumes only on `.collect()` |
