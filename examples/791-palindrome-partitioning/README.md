📖 **[View on hightechmind.io →](https://hightechmind.io/rust/791-palindrome-partitioning)**

---

# 791-palindrome-partitioning — Palindrome Partitioning
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Palindrome partitioning asks: what is the minimum number of cuts needed to divide a string into parts where each part is a palindrome? This combines two DP subproblems: checking if substrings are palindromes (interval DP) and finding minimum cuts (linear DP). It appears in DNA restriction site analysis, string compression algorithms, and has connections to the Longest Palindromic Subsequence problem.

## Learning Outcomes

- Pre-compute a 2D boolean table `is_pal[i][j]` for all substring palindrome checks
- Use the palindrome expansion technique: `is_pal[i][j] = chars[i]==chars[j] && is_pal[i+1][j-1]`
- Apply minimum cuts DP: `dp[i] = min over j: dp[j-1] + 1` when `is_pal[j][i]`
- Understand the O(n²) time complexity for both the palindrome check and the cuts DP
- See how this combines interval DP (palindrome table) with linear DP (cuts)

## Rust Application

`min_cuts(s)` collects chars, initializes `is_pal[i][i] = true` and `is_pal[i][i+1] = (chars[i]==chars[i+1])`. The expansion loop fills longer palindromes using shorter ones. The cuts DP initializes `dp[i] = i` (at most i cuts) and checks if `is_pal[0][i]` (whole prefix is palindrome → 0 cuts). Tests include `"aab"` (1 cut: "aa|b"), `"a"` (0 cuts), and palindromes (0 cuts).

## OCaml Approach

OCaml implements both DP tables with `Array.make_matrix`. The palindrome expansion is idiomatic with nested `for` loops. The minimum cuts DP uses `Array.init` for initialization and imperative updates. A recursive approach with memoization is also natural in OCaml: `min_cuts s i = min over j in 0..i: [if is_pal s j i then 1 + min_cuts s (j-1) else infinity]`.

## Key Differences

1. **Two-phase DP**: Both languages use the same two-phase approach (palindrome table then cuts); the structure is algorithm-determined, not language-determined.
2. **Boundary conditions**: Rust's `dp[i] = i` initialization bounds cuts; OCaml uses the same; careful initialization avoids off-by-one errors.
3. **Combination**: This problem requires combining two DP patterns — interval DP and linear DP — which is the same in both languages.
4. **O(n) space improvement**: Manacher's algorithm computes palindrome checks in O(n) time; combined with the cuts DP, achieves O(n) total space.

## Exercises

1. Implement `palindrome_partition_list(s) -> Vec<Vec<String>>` that returns all valid palindrome partitions (exponential output), using backtracking on the precomputed `is_pal` table.
2. Use Manacher's algorithm (example 820) to compute the palindrome table in O(n) time, improving the total complexity.
3. Extend to `minimum_palindrome_partition_k(s, k)` — minimum cuts to get exactly k palindrome parts, adding a third dimension to the DP.
