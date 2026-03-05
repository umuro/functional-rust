📖 **[View on hightechmind.io →](https://hightechmind.io/rust/791-palindrome-partitioning)**

---

# 791: Palindrome Partitioning

**Difficulty:** 4  **Level:** Advanced

Partition a string into palindromic substrings using the minimum number of cuts — a classic 2D DP problem.

## The Problem This Solves

Given any string, you can always partition it into palindromes (worst case: every single character). But what's the *fewest* cuts you need? This is not just an academic puzzle: it appears in DNA sequence analysis, text compression (palindromes = redundant structure), and anywhere you want to find the most "regular" decomposition of a sequence.

Brute force is exponential — there are 2^(n-1) possible cut positions. The DP approach compresses this to O(n²) by combining two sub-problems: first precompute which substrings are palindromes, then use those results to find minimum cuts. Both sub-problems use the same "expand from center" insight encoded in a bottom-up table.

This is a textbook example of **2D DP feeding into 1D DP** — a pattern that shows up whenever one DP's table becomes another DP's lookup structure.

## The Intuition

Two passes: (1) Build `is_pal[i][j]` — a boolean table where `is_pal[i][j]` is true if `s[i..=j]` is a palindrome. This is O(n²): single chars are always palindromes, adjacent equal chars are palindromes, and longer strings are palindromes if they have equal endpoints and a palindromic interior. (2) Then `cuts[i]` = minimum cuts for `s[0..=i]`. If `s[0..=i]` is already a palindrome, `cuts[i] = 0`. Otherwise, try every split point `j` where `s[j..=i]` is a palindrome, and take `cuts[j-1] + 1`. O(n²) overall.

## How It Works in Rust

```rust
fn palindrome_partition(s: &str) -> (usize, Vec<String>) {
    let b = s.as_bytes();
    let n = b.len();

    // Pass 1: fill is_pal[i][j] bottom-up
    let mut is_pal = vec![vec![false; n]; n];
    for i in 0..n { is_pal[i][i] = true; }                      // length 1
    for i in 0..n-1 { is_pal[i][i+1] = b[i] == b[i+1]; }       // length 2
    for len in 3..=n {                                            // length 3+
        for i in 0..=(n - len) {
            let j = i + len - 1;
            is_pal[i][j] = b[i] == b[j] && is_pal[i+1][j-1];   // recurrence
        }
    }

    // Pass 2: minimum cuts with backtracking
    let mut cuts = vec![usize::MAX; n];
    let mut prev = vec![0usize; n];  // start of last partition (for reconstruction)
    for i in 0..n {
        if is_pal[0][i] {
            cuts[i] = 0; prev[i] = 0;
        } else {
            for j in 1..=i {
                if is_pal[j][i] {
                    let c = cuts[j-1].saturating_add(1);
                    if c < cuts[i] { cuts[i] = c; prev[i] = j; }
                }
            }
        }
    }
    // ... reconstruct parts from prev[] table
}
```

The `prev` table records which split was chosen at each position, enabling O(n) backtracking. Byte indexing (`s.as_bytes()`) avoids UTF-8 multi-byte complexity for ASCII inputs; for Unicode correctness you'd collect `char` values first.

## What This Unlocks

- **2D palindrome table** — the `is_pal` precomputation is reusable across many string problems (longest palindromic substring, palindrome counting, etc.).
- **DP-feeding-DP pattern** — building a helper table in one pass and consuming it in another is a general technique: first DP answers "what's valid?", second answers "what's optimal?".
- **Backtracking with a `prev` array** — tracking *which* choice was made (not just the cost) is the standard way to reconstruct solutions from DP tables.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| 2D boolean table | `Array.make_matrix n n false` | `vec![vec![false; n]; n]` |
| Byte indexing | `Bytes.get s i` | `s.as_bytes()[i]` — zero-cost, returns `u8` |
| Infinity sentinel | `max_int` | `usize::MAX` with `saturating_add` |
| String slicing | `String.sub s start len` | `s[start..=end].to_string()` — checked at runtime |
