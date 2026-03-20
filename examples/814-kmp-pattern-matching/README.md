📖 **[View on hightechmind.io →](https://hightechmind.io/rust/814-kmp-pattern-matching)**

---

# 814-kmp-pattern-matching — KMP Pattern Matching

## Problem Statement

Knuth-Morris-Pratt (1977) finds all occurrences of a pattern in a text in O(n+m) time, avoiding redundant comparisons. Naive search backtracks to the beginning on mismatch; KMP uses the Failure Function (LPS array) to skip ahead. It is used in `grep`, text editors (search/replace), network intrusion detection systems (signature matching), and bioinformatics (exact sequence matching). KMP is the foundational substring search algorithm that all others build upon.

## Learning Outcomes

- Compute the LPS (Longest Proper Prefix-Suffix) array for a pattern in O(m)
- Use the LPS array to avoid re-examining characters in the text
- Implement `kmp_search` returning all starting positions of pattern occurrences
- Understand the invariant: at each mismatch, jump to `lps[j-1]` to skip known matches
- Compare with naive O(nm) search and Boyer-Moore O(n/m) average

## Rust Application

`compute_lps(pattern)` fills the failure function array: when `p[i] == p[len]`, extend; when mismatch and `len > 0`, jump to `lps[len-1]`. `kmp_search(text, pattern)` uses `lps` to drive the matching: on mismatch, if `j > 0` jump to `lps[j-1]`, else advance `i`. On full match (`j == m`), record the position and jump to `lps[j-1]`. Tests verify multiple occurrences and no-match cases.

## OCaml Approach

OCaml implements KMP with `Array.make m 0` for LPS and `Buffer.t` or direct position list for results. The recursive functional style: `let rec fill i len = ...` computes LPS cleanly. OCaml's `String.get` provides O(1) character access. The `Re.execp` function in the `re` library uses a different but related approach for regex matching. OCaml's `Str` module uses NFA-based matching.

## Key Differences

1. **LPS computation**: Both languages implement the same KMP failure function loop; the logic is language-independent.
2. **String iteration**: Rust collects to `Vec<char>` for indexed access (O(1) per char); OCaml's `String.get` is O(1) for bytes; both handle ASCII correctly.
3. **Multiple matches**: Both return `Vec<usize>` / `int list` of starting positions; the jump after match (`lps[m-1]`) enables overlapping matches.
4. **Production use**: Rust's `memchr` crate uses SIMD-accelerated byte search for single characters; KMP is used for multi-character patterns.

## Exercises

1. Implement `kmp_count(text, pattern) -> usize` that counts non-overlapping occurrences efficiently.
2. Extend to 2D pattern matching: search for a 2D pattern in a 2D text by running KMP on each row to find column matches, then KMP on columns.
3. Use KMP to find the shortest string period: compute LPS of `pattern` and use it to determine if `pattern` has a period `p = m - lps[m-1]`.
