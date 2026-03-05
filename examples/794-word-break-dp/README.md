# 794. Word Break (Dictionary DP)

**Difficulty:** 3  **Level:** Advanced

Determine whether a string can be segmented into dictionary words — and enumerate all valid segmentations.

## The Problem This Solves

Word break is the algorithmic core of text tokenisation: given a sequence of characters with no spaces, can it be decomposed into known tokens? Input method editors (Chinese/Japanese/Korean text input) use exactly this to segment character streams into words. Search engine query processing, URL slug parsing, and natural language processing pipelines all need efficient word segmentation when whitespace is absent or unreliable.

The enumeration variant (all valid segmentations) is relevant for parser generators and grammar checkers that need to explore ambiguous parses — the same O(n²) DP structure, extended to collect all parse trees.

## The Intuition

`dp[i]` is true if `s[0..i]` can be segmented from the dictionary. For each position `i`, scan all earlier positions `j`; if `dp[j]` is true and `s[j..i]` is a dictionary word, then `dp[i]` is true. Once `dp[n]` is true, you know a segmentation exists. The `HashSet` lookup makes each substring check O(1) on average, giving O(n²) overall. OCaml would express this recursively with memoisation via `Hashtbl`; Rust uses a `HashSet<&str>` for the dictionary and a `Vec<bool>` for the DP table. The `prev` array variant stores all `j` values that contributed to each `dp[i]`, enabling full reconstruction.

## How It Works in Rust

```rust
// O(n²) time — HashSet lookup is O(1) average
fn word_break(s: &str, dict: &[&str]) -> bool {
    let dict_set: HashSet<&str> = dict.iter().copied().collect();
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..=n {
        for j in 0..i {
            if dp[j] && dict_set.contains(&s[j..i]) {
                dp[i] = true;
                break;  // found one valid split, no need to continue
            }
        }
    }
    dp[n]
}

// Enumerate all segmentations: replace bool dp with Vec<Vec<usize>>
// prev[i] = all j values such that dp[j] && s[j..i] is in dict
fn word_break_all(s: &str, dict: &[&str]) -> Vec<String> {
    let mut prev: Vec<Vec<usize>> = vec![vec![]; n + 1];
    // ... same DP, no early break, collect all j in prev[i]

    // Recursive reconstruction from prev
    fn collect(s: &str, prev: &Vec<Vec<usize>>, i: usize) -> Vec<String> {
        if i == 0 { return vec![String::new()]; }
        let mut results = Vec::new();
        for &j in &prev[i] {
            let word = &s[j..i];
            for base in collect(s, prev, j) {
                results.push(if base.is_empty() { word.into() }
                             else { format!("{base} {word}") });
            }
        }
        results
    }
    if dp[n] { collect(s, &prev, n) } else { vec![] }
}
```

Note the `break` in the boolean variant — once one valid split is found, there's no need to check the rest. The `all` variant omits this to collect every parse path.

## What This Unlocks

- **Input method editors**: segment CJK character streams into morpheme candidates; combine with a language model to rank ambiguous parses.
- **URL routing**: parse slug-based URLs against a route dictionary, identifying valid path component boundaries.
- **Compiler lexers**: tokenise identifier-heavy languages without whitespace delimiters, verifying that token streams decompose into valid lexemes.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Dictionary lookup | `Hashtbl.mem` or `Set.mem` | `HashSet<&str>::contains` — zero-copy `&str` slices |
| DP array | `Array.make (n+1) false` | `vec![false; n+1]` |
| Substring slice | `String.sub s j (i-j)` | `&s[j..i]` — O(1) slice, no allocation |
| All-parses backtrack | Recursive with list accumulator | Recursive inner fn on `&Vec<Vec<usize>>` |
| Early termination | `raise Exit` or boolean flag | `break` in inner loop |
