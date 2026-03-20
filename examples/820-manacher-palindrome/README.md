📖 **[View on hightechmind.io →](https://hightechmind.io/rust/820-manacher-palindrome)**

---

# Manacher's Algorithm — Longest Palindromic Substring

## Problem Statement

Finding the longest palindromic substring naively requires O(n^2) time by expanding from each center. Manacher's algorithm finds all palindromic substrings — both odd and even length — in O(n) time by reusing previously computed palindrome radii. This is the canonical O(n) solution to "find the longest palindrome in a string," a classic interview problem with practical applications in DNA analysis (palindromic sequences indicate restriction enzyme sites), text compression, and symmetry detection. The key insight is that palindromes inside a larger known palindrome have known minimum radii, avoiding redundant character comparisons.

## Learning Outcomes

- Transform the string with sentinel characters (`#`) to unify odd/even length palindrome handling
- Understand the center/right-boundary invariant: track the palindrome extending furthest right
- Use the mirror property: `p[mirror] = p[i]` (limited by the right boundary) as a starting point
- Implement the O(n) linear scan with the amortized argument for why each character is visited O(1) times
- Recover substring positions from the transformed string's palindrome radius array

## Rust Application

```rust
pub fn manacher(s: &str) -> String {
    // Transform: "abc" -> "#a#b#c#"
    let t: Vec<char> = std::iter::once('#')
        .chain(s.chars().flat_map(|c| [c, '#']))
        .collect();
    let mut p = vec![0usize; t.len()];
    let (mut c, mut r) = (0usize, 0usize);
    // For each center i: p[i] = mirror's radius or expand
}
```

The `#`-interleaving transformation turns all palindromes into odd-length ones, simplifying the algorithm. Rust's iterator chaining with `flat_map` and `once` creates the transformed string without a separate loop. The `p` array stores palindrome radii in the transformed string. After computation, the maximum in `p` gives the longest palindrome; converting back to original string coordinates divides by 2. Rust's ownership ensures the transformed string lives exactly as long as needed for the computation.

## OCaml Approach

OCaml implements the transformation with `Buffer`, appending `'#'` between and around each character. The palindrome radius array is `Array.make (2*n+1) 0`. Center and right boundary are mutable `int ref` values. OCaml's pattern matching elegantly handles the three cases: mirror palindrome fits entirely within boundary, mirror radius equals boundary distance, or expansion needed. The `String.sub` at the end extracts the longest palindromic substring from the original coordinates.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Transformation | `flat_map` + `once` | `Buffer` with character appending |
| Radius array | `Vec<usize>` | `int array` |
| Center/right state | Two `usize` variables | `int ref` pair |
| Result extraction | `p.iter().enumerate().max_by_key()` | `Array.fold_left` for max |
| Char handling | `chars()` for Unicode | `Uchar` or byte-level |
| Sentinel choice | `#` (not in typical ASCII text) | Same; often `\000` for bytes |

## Exercises

1. Return all palindromic substrings (not just the longest) sorted by length descending.
2. Count the total number of distinct palindromic substrings using the Manacher array.
3. Find the minimum number of cuts to partition a string into palindromes (use Manacher for O(n) palindrome detection + DP).
4. Implement Eertree (palindromic tree) as an alternative data structure and compare with Manacher.
5. Extend to report palindromes of a minimum length k efficiently without O(n) post-processing.
