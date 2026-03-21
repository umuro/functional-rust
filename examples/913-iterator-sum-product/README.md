📖 **[View on hightechmind.io →](https://hightechmind.io/rust/913-iterator-sum-product)**

---

# 913-iterator-sum-product — Iterator Sum and Product
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Reducing a sequence to a single number by addition or multiplication is so common that every language provides a dedicated abstraction. Python's `sum()`, OCaml's `List.fold_left (+) 0`, Haskell's `sum` and `product`. Rust's `Iterator::sum()` and `Iterator::product()` are zero-cost abstractions over `fold` with the identity element built in: `0` for sum, `1` for product. They work on any numeric type implementing `Sum` or `Product` and can be chained with any adapter. Factorial, sum-of-squares, inner product, and average are all expressible as one-liners using these consumers.

## Learning Outcomes

- Use `.sum()` and `.product()` as clean alternatives to explicit `.fold()` calls
- Compute factorial using `(1..=n).product()`
- Chain `.map()` with `.sum()` for sum-of-squares and weighted sums
- Compute averages by combining `.sum()` with `.len()`
- Compare with OCaml's fold-based approach for the same computations

## Rust Application

`sum_ints` uses `nums.iter().copied().sum::<i32>()`. `product_ints` uses `.product()`. `factorial(n)` is `(1..=n).product()` — no accumulator variable needed. `sum_of_squares` uses `nums.iter().map(|&x| x * x).sum()` — map then sum in one expression. `average` uses `.sum::<f64>() / prices.len() as f64`. The turbofish annotation `::<i32>` is occasionally needed when the compiler cannot infer the output type of `sum()`.

## OCaml Approach

OCaml has no `List.sum` — it uses `List.fold_left (+) 0 xs`. `List.fold_left ( * ) 1 xs` for product. Factorial: `let factorial n = List.fold_left ( * ) 1 (List.init n (fun i -> i + 1))`. Sum of squares: `List.fold_left (fun acc x -> acc + x * x) 0 xs`. Average: `float_of_int (List.fold_left (+) 0 xs) /. float_of_int (List.length xs)`. The explicit fold is more verbose but equivalent — OCaml's pipe operator makes it readable: `xs |> List.fold_left (+) 0`.

## Key Differences

1. **Named abstractions**: Rust's `.sum()` and `.product()` communicate intent clearly; OCaml uses generic `fold_left` — both express the same computation.
2. **Type annotation**: Rust sometimes needs turbofish for `.sum::<f64>()`; OCaml's type inference usually handles it automatically.
3. **Product vs fold**: Rust `(1..=n).product()` for factorial is more readable than OCaml's explicit fold with `List.init`; the computation is identical.
4. **No standard sum**: OCaml deliberately omits `List.sum` — it is considered a special case of the more general fold; Rust optimizes for the common case.

## Exercises

1. Implement `harmonic_sum(n: u64) -> f64` = 1 + 1/2 + 1/3 + ... + 1/n using `.map()` and `.sum()`.
2. Write `weighted_average(values: &[f64], weights: &[f64]) -> Option<f64>` using zip, map, and sum.
3. Compute the nth Fibonacci number using `(1..n).fold()` with state `(a, b)` and compare readability with an explicit loop.
