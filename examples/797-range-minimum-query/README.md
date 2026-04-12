📖 **[View on hightechmind.io →](https://hightechmind.io/rust/797-range-minimum-query)**

---

# 797-range-minimum-query — Range Minimum Query (Sparse Table)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Range Minimum Query (RMQ) asks: given an array and a query `(l, r)`, what is the minimum element in `arr[l..=r]`? Naive O(n) per query is too slow for many applications. The Sparse Table data structure preprocesses the array in O(n log n) time to answer RMQ queries in O(1) using overlapping power-of-two ranges. RMQ is used in LCA (Lowest Common Ancestor) algorithms for trees, suffix array construction, and competitive programming.

## Learning Outcomes

- Build a sparse table `table[j][i]` = index of minimum in `arr[i..i+2^j]`
- Use log precomputation `log[i] = floor(log2(i))` for O(1) query dispatch
- Answer `query(l, r)` by overlapping two ranges of size `2^k` where `k = log[r-l+1]`
- Understand why overlapping ranges are valid (minimum is idempotent: `min(min(a,b), min(b,c)) = min(a,b,c)`)
- Know when Segment Trees (O(log n) query, O(log n) update) are preferred over Sparse Tables (O(1) query, immutable)

## Rust Application

`SparseTable::new(arr)` computes `k = log2(n)+1` levels. `log[i]` is precomputed by `log[i] = log[i/2]+1`. Table `[j][i]` stores the index of the minimum in `arr[i..i+2^j]`. `query(arr, l, r)` looks up `k = log[r-l+1]`, then compares `table[k][l]` and `table[k][r-(1<<k)+1]`. Tests verify correct minimum indices for several ranges.

## OCaml Approach

OCaml implements the sparse table with `Array.make_matrix k n 0`. The log precomputation uses a `for` loop over indices. OCaml's `log2` function with `int_of_float` computes the floor log2. `min` is the stdlib comparison function. The `query` function is a two-line function selecting the minimum of two table entries. Segment trees (a related data structure) are common in OCaml competitive programming.

## Key Differences

1. **Immutability advantage**: Sparse Table's O(1) query relies on immutability; Rust's ownership system naturally prevents mutation after construction.
2. **Space**: O(n log n) space for the sparse table vs. O(n) for a simple segment tree — a trade-off between query speed and memory.
3. **RMQ vs other queries**: Sparse tables work for any idempotent function (min, max, GCD); non-idempotent functions (sum) require segment trees instead.
4. **Competitive programming**: RMQ + LCA is a classic competitive programming combo; Rust is increasingly used in competitive programming for its speed and safety.

## Exercises

1. Implement `range_gcd_query(arr, l, r) -> u64` using a sparse table (GCD is idempotent: `gcd(gcd(a,b), gcd(b,c)) = gcd(a,b,c)`).
2. Implement an alternative `SparseTable` that returns the actual value rather than the index, and compare its ergonomics for the min/max use case.
3. Implement a Segment Tree that supports both range minimum queries AND point updates in O(log n), demonstrating when it's preferred over the Sparse Table.
