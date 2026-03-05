# Regex Matching — Comparison

## Core Insight
Regex matching with `.` and `*` requires careful handling of the `*` operator, which applies to the preceding character. The DP table tracks whether `s[0..i]` matches `p[0..j]`, with `*` creating two transitions: skip the `x*` pair, or consume one character if it matches.

## OCaml Approach
- 2D boolean array DP
- Memoized recursion with `Hashtbl` — cleaner logic
- `first_match` computed inline with `&&` and `||`
- `Char` comparison with `=`

## Rust Approach
- 2D `vec![vec![false]]` DP
- `HashMap` memoization
- NFA simulation as third approach — treats pattern as finite automaton
- `HashSet` for NFA state tracking

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Char access | `s.[i]` and `p.[j]` | `Vec<char>` after `.chars().collect()` |
| Boolean DP | `Array.init` + `Array.make` | `vec![vec![false]]` |
| Star handling | `dp.(i).(j-2) \|\| dp.(i-1).(j)` | Same logic |
| NFA approach | Not shown | `HashSet<usize>` for state sets |
| Pattern matching | Character comparison | Same |
