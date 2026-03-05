# 274: Numeric Reductions: sum() and product()

**Difficulty:** 1  **Level:** Beginner

Fold a numeric iterator to a single value using addition or multiplication.

## The Problem This Solves

Adding up a list of numbers or computing their product are among the most common reductions in any program. You could write `.fold(0, |acc, x| acc + x)` — but that exposes the identity element and the operator as boilerplate, and signals nothing about *intent* to the reader. `sum()` and `product()` remove all that noise.

They're also more generic than they appear: both work on any type that implements the `Sum` or `Product` trait — including `f64`, `u64`, `i32`, and even `Option<T>` (where a single `None` makes the whole sum `None`). You can map + sum in one expression: `scores.iter().map(|s| s * s).sum()`.

In OCaml, you'd use `List.fold_left (+) 0` or `List.fold_left ( * ) 1`. In Rust, `sum()` and `product()` are the idiomatic one-word versions.

## The Intuition

`sum()` reduces an iterator of numbers by adding them all together (identity: 0). `product()` multiplies them all (identity: 1). Both are zero-cost abstractions over `fold` — the compiler produces identical machine code.

```rust
let nums = [1, 2, 3, 4, 5];
let total: i32 = nums.iter().sum();      // → 15
let prod: i32  = nums.iter().product();  // → 120
```

## How It Works in Rust

```rust
let nums = [1i32, 2, 3, 4, 5];

// Basic sum and product
let total: i32 = nums.iter().sum();      // → 15
let prod: i32  = nums.iter().product();  // → 120

// Factorial: product over a range
let factorial = |n: u64| -> u64 { (1..=n).product() };
println!("{}", factorial(10));  // → 3628800

// Compose with map: sum of squares
let sum_squares: i32 = nums.iter().map(|&x| x * x).sum();  // → 55

// Float: total price and average
let prices = [9.99f64, 14.50, 3.75, 22.00];
let total_price: f64 = prices.iter().sum();
let avg = total_price / prices.len() as f64;

// Gauss formula check
let gauss: i32 = (1..=100).sum();
assert_eq!(gauss, 5050);

// Empty iterator returns the identity element
let empty_sum: i32 = vec![].into_iter().sum();      // → 0
let empty_prod: i32 = vec![].into_iter().product(); // → 1

// Option<T> sum: None if any element is None
let opts: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
let opt_sum: Option<i32> = opts.into_iter().sum();  // → Some(6)
```

The return type must be annotated or inferred — `sum()` and `product()` are generic over the output type.

## What This Unlocks

- **Factorial and combinatorics** — `(1..=n).product()` is the most concise factorial in any language.
- **Statistical aggregation** — sum over mapped values for dot products, sum of squares, weighted totals.
- **Fallible aggregation** — `Option` and `Result` implement `Sum`, so a single failure propagates automatically.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sum a list | `List.fold_left (+) 0 lst` | `iter.sum::<i32>()` |
| Product a list | `List.fold_left ( * ) 1 lst` | `iter.product::<i32>()` |
| Identity element | Explicit in fold | Implicit (from `Sum`/`Product` trait) |
| Floating point | Same | Works — `f32`, `f64` implement both |
| Empty collection | Must pass identity manually | Returns identity element automatically |
