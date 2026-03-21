# Example 1173: Knapsack Problem — Dynamic Programming (Functional)
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Given a list of items, each with a weight and a value, find the maximum total value
that can fit into a knapsack of a given capacity (0/1 knapsack: each item taken at most once).

## Learning Outcomes

- How OCaml's `Hashtbl` memoization maps to Rust's `HashMap` for top-down DP
- How recursive functional decomposition translates into explicit `fn` helpers in Rust (no closures capturing `&mut`)
- How the same problem can be solved with a bottom-up DP table or a space-optimised rolling array
- When `rev()` on a range prevents double-counting items in the rolling approach

## OCaml Approach

OCaml defines a local recursive function `solve` that closes over the items array and the
hash table, checking the cache before recursing and storing results after. The functional
style naturally expresses the two branches (take vs. skip an item) as a `let … in` chain.

## Rust Approach

Rust cannot have a mutable borrow (`&mut HashMap`) inside a recursive closure that also
borrows `items`, so the inner function is lifted to a free `fn` that receives all
dependencies as parameters. The bottom-up variant uses a 2-D `Vec<Vec<usize>>`, while
the rolling variant saves memory by iterating capacity in reverse with `.rev()`.

## Key Differences

1. **Mutable captured state:** OCaml closures freely capture mutable `Hashtbl`; Rust requires
   lifting the recursive helper to a free function to satisfy the borrow checker.
2. **Tuple patterns:** OCaml uses `let (w, v) = items.(i)` destructuring; Rust uses the same
   syntax `let (w, v) = items[i]` on a slice dereference.
3. **Max combinator:** OCaml uses `max without with_item`; Rust uses `without.max(with_item)`,
   a method on `usize`, avoiding a free function import.
4. **Space optimisation:** The rolling-array approach is natural in Rust with `(w..=capacity).rev()`
   — iterating high-to-low ensures each item is used at most once per pass.
