📖 **[View on hightechmind.io →](https://hightechmind.io/rust/274-iterator-sum-product)**

---

# 274: Numeric Reductions: sum() and product()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Summing or multiplying all elements of a collection is one of the most fundamental computational operations — from computing totals in spreadsheets to calculating factorials in mathematics, to aggregating metrics in monitoring systems. While these could be implemented with `fold()`, Rust provides `sum()` and `product()` as first-class methods that express intent clearly and can be implemented efficiently by the type system via the `Sum` and `Product` traits.

## Learning Outcomes

- Understand `sum()` and `product()` as ergonomic specializations of `fold` for numeric types
- Recognize that `sum()` returns zero and `product()` returns one for empty iterators (identity elements)
- Use `sum()` and `product()` on iterators of references by using `.copied()` or `.cloned()`
- Implement the `Sum` and `Product` traits for custom numeric types

## Rust Application

`Iterator::sum()` and `Iterator::product()` require the `Item` type to implement `Sum<Item>` and `Product<Item>` respectively. These traits are implemented for all primitive numeric types:

```rust
// Gauss's formula: sum 1..=100 = 5050
let sum: i32 = (1..=100).sum();
assert_eq!(sum, 5050);

// Factorial: 5! = 120
let fact5: u64 = (1u64..=5).product();
assert_eq!(fact5, 120);

// Sum references with .copied()
let v = [1.0f64, 2.0, 3.0];
let total: f64 = v.iter().copied().sum();

// Empty sum/product use identity elements
let empty_sum: i32 = Vec::<i32>::new().into_iter().sum(); // 0
let empty_product: i32 = Vec::<i32>::new().into_iter().product(); // 1
```

## OCaml Approach

OCaml uses `List.fold_left (+) 0` for sum and `List.fold_left ( *) 1` for product — there are no dedicated functions:

```ocaml
let sum xs = List.fold_left (+) 0 xs
let product xs = List.fold_left ( * ) 1 xs
let () = assert (sum [1;2;3;4;5] = 15)
```

`Base.List.sum` and `Base.List.product` exist in Jane Street's `Base` library.

## Key Differences

1. **Trait-based**: Rust's `sum()`/`product()` are generic over any type implementing `Sum`/`Product`; OCaml requires type-specific fold expressions.
2. **Identity elements**: Both use the mathematical identity — zero for sum, one for product — on empty iterators.
3. **Overflow behavior**: Rust's `sum()`/`product()` wrap on integer overflow in release mode; use `checked_sum` patterns or saturating arithmetic when needed.
4. **Custom types**: Implementing `std::iter::Sum` for a custom type lets it participate in `sum()` chains naturally.

## Exercises

1. Compute the sum of squares of a range using `map(|x| x*x).sum::<i64>()`.
2. Implement a `product` function for floating-point numbers that handles the case of any zero element by returning zero early.
3. Use `sum()` and `count()` together to compute the arithmetic mean of a `Vec<f64>`.
