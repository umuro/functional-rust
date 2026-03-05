# Phone Keypad Letter Combinations — Comparison

## Core Insight
Each digit maps to 3-4 letters, creating a tree of combinations. Three approaches: backtracking (DFS), iterative queue (BFS-like), and fold (functional). The fold is most elegant, treating each digit as a transformation on the set of prefixes.

## OCaml Approach
- `Buffer` for string building with `truncate` for backtracking
- `String.iter` to iterate over digit's letters
- `List.concat_map` for functional branching
- `Queue` module for iterative approach

## Rust Approach
- `String::push`/`pop` for backtracking
- `PHONE_MAP` as `const &[&str]` array
- `flat_map` + `format!` for fold approach — very expressive
- `Vec` as queue with swap for iterative approach

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Phone map | `let phone_map = [|...|]` | `const PHONE_MAP: &[&str]` |
| Digit to index | `Char.code d - Char.code '0'` | `(b - b'0') as usize` |
| String backtrack | `Buffer.truncate` | `String::pop()` |
| Functional | `List.concat_map` | `flat_map` + `format!` |
| Queue approach | `Queue.t` (mutable) | `Vec` swap per level |
