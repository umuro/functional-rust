📖 **[View on hightechmind.io →](https://hightechmind.io/rust/279-pascals-triangle)**

---

# Example 279: Pascal's Triangle — Row Generation
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Generate the first N rows of Pascal's triangle, where each row is computed from the previous one using the "zip-with-add" trick: prepend and append 0 to the current row, then sum pairwise.

## Learning Outcomes

- Using `std::iter::successors` for generating sequences from a recurrence
- Iterator chaining with `once`, `chain`, and `zip` to implement zip-with-add
- Translating OCaml's `List.map2` to Rust's iterator zip pattern
- Comparing recursive, fold, and successors approaches

## OCaml Approach

OCaml uses `List.map2 (+)` to add two lists element-wise. The trick: `0 :: row` prepends a zero, `row @ [0]` appends a zero, making both lists the same length. `List.map2 (+)` then sums corresponding elements to produce the next row. Recursion accumulates rows.

## Rust Approach

Rust uses `std::iter::once(&0).chain(row.iter()).zip(row.iter().chain(once(&0)))` — the same prepend/append-zero trick expressed with iterator adapters. `std::iter::successors` generates the infinite sequence of rows lazily, and `.take(n)` limits it.

## Key Differences

1. **Zip-with-add:** OCaml's `List.map2 (+)` is a single call; Rust chains `once`, `chain`, `zip`, and `map` — more verbose but composable
2. **Sequence generation:** OCaml uses explicit recursion (`let rec go`); Rust's `successors` provides a declarative "generate from seed" pattern
3. **Lazy vs eager:** Rust's `successors` is lazy — rows are only computed when consumed; OCaml's recursion eagerly builds the full list
4. **Numeric type:** OCaml uses arbitrary-precision `int`; Rust uses `u64` which can overflow for large row numbers

## Exercises

1. Implement a `binomial` function using Pascal's triangle row as a lookup table, and verify `C(n, k) == pascals_row(n)[k]` for all `k` from 0 to `n`.
2. Use Pascal's triangle to expand `(x + y)^n` symbolically: return a `Vec<(i64, i64, i64)>` of `(coefficient, x_power, y_power)` terms.
3. Generate the Sierpinski triangle pattern by taking Pascal's triangle modulo 2: output a visual grid where odd coefficients are filled and even ones are empty — observe the fractal structure.
