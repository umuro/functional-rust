# 814: KMP — Knuth-Morris-Pratt Pattern Matching

**Difficulty:** 4  **Level:** Advanced

Linear-time exact string search: build the failure function once in O(m), then scan text in O(n), never stepping back.

## The Problem This Solves

When searching for a pattern in a long text, the naïve approach compares character-by-character and backtracks on mismatch — giving O(n×m) worst case. This is catastrophic for repetitive patterns on repetitive text (e.g., searching "aaab" in "aaaaaaa…"), a real problem in DNA sequence alignment, log parsing, and network intrusion detection.

KMP eliminates all backtracking in the text. After a partial match of length `k` fails, KMP knows exactly how much of the already-matched prefix can be reused — because it precomputed this information in the failure function. You never re-examine a text character you've already passed.

The result: O(n + m) guaranteed, where n is text length and m is pattern length. For the bioinformatics case of searching a 3-billion-character genome, this difference is the difference between seconds and hours.

## The Intuition

The failure function `pi[i]` stores the length of the longest proper prefix of `pattern[0..=i]` that is also a suffix. On mismatch at position `j` in the pattern, jump to `pi[j-1]` — you already know the first `pi[j-1]` characters of the pattern match, so resume from there. The text pointer never moves backwards. Total work is O(n + m) because the pattern pointer can only advance n times total across the whole search.

In OCaml the same algorithm is expressed recursively; Rust's iterative version is identical in structure but more explicit about state.

## How It Works in Rust

```rust
// Step 1: Build the prefix/failure table in O(m)
fn build_prefix(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let mut pi = vec![0usize; m];
    let mut k = 0usize;
    for i in 1..m {
        // Walk back through the failure chain until a match or k==0
        while k > 0 && pattern[k] != pattern[i] { k = pi[k - 1]; }
        if pattern[k] == pattern[i] { k += 1; }
        pi[i] = k;  // Longest proper prefix-suffix of pattern[0..=i]
    }
    pi
}

// Step 2: Search in O(n) — text pointer only moves forward
fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let (t, p) = (text.as_bytes(), pattern.as_bytes());
    let (n, m) = (t.len(), p.len());
    if m == 0 { return vec![]; }

    let pi = build_prefix(p);
    let mut matches = Vec::new();
    let mut q = 0usize;  // Characters matched so far

    for i in 0..n {
        while q > 0 && p[q] != t[i] { q = pi[q - 1]; }  // Failure link
        if p[q] == t[i] { q += 1; }
        if q == m {
            matches.push(i + 1 - m);  // Match at this position
            q = pi[m - 1];            // Prepare for overlapping matches
        }
    }
    matches
}
```

Key Rust idiom: `as_bytes()` gives a `&[u8]` with O(1) indexing and no UTF-8 overhead — always use this for byte-level string algorithms.

## What This Unlocks

- **Bioinformatics**: Search DNA/protein sequences for motifs; KMP is the baseline before BLAST/BWA.
- **Log analysis**: Find error patterns in multi-gigabyte log streams without loading them fully into memory.
- **Aho-Corasick**: KMP extended to multiple patterns simultaneously — the foundation of intrusion detection systems and text sanitizers.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Failure table build | Recursive helper with `pi` array | Iterative `vec![0; m]`, same logic |
| Pattern as bytes | `Bytes.of_string pattern` | `pattern.as_bytes()` — zero-cost |
| Match state | `ref` integer or functional threading | `let mut q = 0usize` |
| Collect results | `List.rev` accumulator | `Vec::push`, no reversal needed |
| Overlap handling | `pi.(m-1)` after match | `q = pi[m - 1]` — identical |
