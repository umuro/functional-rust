# 496: String Diff / Edit Distance

**Difficulty:** 2  **Level:** Intermediate

Compare two strings to find how many edits separate them — and identify the closest match in a candidate list.

## The Problem This Solves

When you accept user input, you constantly face near-misses: a typo in a command name, a misspelled identifier, a search query that's almost-but-not-quite right. Without a distance metric you're stuck with exact matching, which means any typo silently fails.

Levenshtein edit distance gives you a number: how many single-character insertions, deletions, or substitutions turn one string into another. With that number you can rank candidates, suggest corrections ("did you mean...?"), and build fuzzy search. This is the same algorithm behind spell-checkers, DNA sequence alignment, and `git diff`.

Beyond finding the single nearest match, character-by-character comparison lets you highlight *where* strings differ — useful for test failure output, diffs, and logging.

## The Intuition

Imagine filling in a grid where rows are characters of the first string and columns are characters of the second. Each cell stores the minimum edits needed to transform the prefix ending at that row into the prefix ending at that column. If the characters match, carry the diagonal value forward for free. If they don't, take the cheapest of three options: insert, delete, or substitute (each costs 1). The bottom-right corner is your answer.

For "nearest match", just run Levenshtein against every candidate and take the minimum.

## How It Works in Rust

**Step 1 — Collect chars** to avoid byte-indexing into UTF-8:
```rust
let sv: Vec<char> = s.chars().collect();
let tv: Vec<char> = t.chars().collect();
```

**Step 2 — Fill the DP table** with classic mutable 2D vec:
```rust
let mut dp = vec![vec![0usize; n+1]; m+1];
for i in 0..=m { dp[i][0] = i; }
for j in 0..=n { dp[0][j] = j; }
for i in 1..=m {
    for j in 1..=n {
        dp[i][j] = if sv[i-1] == tv[j-1] {
            dp[i-1][j-1]
        } else {
            1 + dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1])
        };
    }
}
```

**Step 3 — Find the closest candidate** using `min_by_key`:
```rust
fn closest<'a>(query: &str, candidates: &[&'a str]) -> Option<&'a str> {
    candidates.iter()
        .min_by_key(|&&c| levenshtein(query, c))
        .copied()
}
```
The lifetime `'a` ensures the returned `&str` borrows from `candidates`, not from `query`.

**Step 4 — Simple char diff** with `zip`:
```rust
let diff: String = a.chars().zip(b.chars())
    .map(|(ac, bc)| if ac == bc { ac } else { '*' })
    .collect();
```

## What This Unlocks

- **Autocomplete and fuzzy search** — rank suggestions by edit distance to the query.
- **Error messages** — "unknown command 'buid', did you mean 'build'?" in any CLI.
- **Diff visualization** — character-level highlighting without pulling in a diff library.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| 2D DP table | `Array.make_matrix` | `vec![vec![...]]` |
| Char iteration | `String.to_seq` / `Bytes.get` | `.chars().collect::<Vec<char>>()` |
| Min of three | `min a (min b c)` | `.min(b).min(c)` chained |
| Return borrowed str | GC handles lifetimes | explicit `'a` on return type |
