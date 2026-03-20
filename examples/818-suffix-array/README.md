📖 **[View on hightechmind.io →](https://hightechmind.io/rust/818-suffix-array)**

---

# Suffix Array

## Problem Statement

A suffix array enables lightning-fast substring search, counting occurrences, finding the longest repeated substring, and computing the longest common substring between two strings — all after O(n log n) preprocessing. Without a suffix array or suffix tree, answering "how many times does pattern P appear in text T?" requires O(n*m) per query. With a suffix array, binary search answers each query in O(m log n). This powers full-text search engines, bioinformatics sequence analysis, data compression (BWT transform), and plagiarism detection tools. The suffix array is a space-efficient alternative to suffix trees that fits in O(n) integers.

## Learning Outcomes

- Understand that a suffix array is a sorted array of starting indices of all suffixes of a string
- Implement the naive O(n^2 log n) construction using sort with suffix comparisons
- Recognize how binary search on a sorted suffix array answers substring queries in O(m log n)
- Learn how the LCP (Longest Common Prefix) array augments the suffix array for advanced queries
- Compare with suffix trees: suffix arrays use less memory but require more algorithmic sophistication

## Rust Application

```rust
pub fn build_suffix_array(s: &str) -> Vec<usize> {
    let bytes = s.as_bytes();
    let mut sa: Vec<usize> = (0..bytes.len()).collect();
    sa.sort_by(|&a, &b| bytes[a..].cmp(&bytes[b..]));
    sa
}
pub fn search(s: &str, sa: &[usize], pattern: &str) -> Option<usize> {
    sa.binary_search_by(|&i| s[i..].cmp(pattern)).ok() // simplified
}
```

Rust's slice comparison `bytes[a..].cmp(&bytes[b..])` gives lexicographic suffix comparison for free — no custom comparator needed. The `sort_by` closure captures `bytes` and compares suffixes by their byte slices, clean and safe. Binary search uses `partition_point` to find the range of suffixes that start with the pattern. The `as_bytes()` conversion makes the suffix array construction work on byte-level, which is standard for ASCII text; Unicode requires special handling at code-point boundaries.

## OCaml Approach

OCaml builds the suffix array with `Array.init n (fun i -> i)` then `Array.sort` with a comparator using `String.sub` for suffix extraction. The naive approach is functionally clean: `Array.sort (fun a b -> compare (String.sub s a n) (String.sub s b n)) sa`. More efficient O(n log^2 n) doubling algorithms use `Array.map` to rank suffixes and iteratively refine. OCaml's `String.compare` for lexicographic order maps directly to the comparison needed. For binary search, `Array.binary_search` (in some libraries) or manual bisection applies.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Suffix comparison | `bytes[a..].cmp(&bytes[b..])` | `String.sub s a len |> compare` |
| Sort | `.sort_by()` with closure | `Array.sort` with comparison function |
| Construction complexity | Naive O(n^2 log n) | Same for naive; SA-IS for O(n) |
| Binary search | `partition_point` for range | Manual bisection or library |
| Memory | `Vec<usize>` — cache-friendly | `int array` — GC-managed |
| Unicode | Byte-level (ASCII assumed) | `Uchar.t array` for full Unicode |

## Exercises

1. Implement the LCP array using Kasai's algorithm and use it to find the longest repeated substring.
2. Add a `search_range` function returning all positions where a pattern occurs using the LCP array.
3. Implement the O(n log^2 n) suffix array construction using the doubling technique.
4. Use the suffix array to find the longest common substring of two strings by concatenating with a sentinel.
5. Benchmark suffix array binary search against a simple sliding window search for varying pattern lengths.
