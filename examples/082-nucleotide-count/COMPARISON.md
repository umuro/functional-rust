# Nucleotide Count — Comparison

## Core Insight
Character frequency counting shows the difference between immutable maps (OCaml) and mutable maps (Rust). OCaml rebuilds the map on each update; Rust mutates in place. Both handle invalid input, but with different error mechanisms.

## OCaml Approach
- `Map.Make(Char)` — persistent (immutable) map, O(log n) per update
- `String.fold_left` to iterate and accumulate
- `failwith` for invalid nucleotides
- Each update creates a new map (old one still valid)

## Rust Approach
- `HashMap<char, usize>` — mutable, O(1) amortized per update
- `get_mut(&c)` for in-place mutation
- `Result<HashMap, char>` for error handling
- Array variant `[usize; 4]` avoids allocation entirely

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Map type | `Map.Make(Char)` persistent | `HashMap<char, usize>` mutable |
| Update | New map per insert | In-place `*n += 1` |
| Error | `failwith` exception | `Result<T, char>` |
| Performance | O(log n) per update | O(1) amortized |
| Zero-alloc option | No | Yes (array variant) |

## Learner Notes
- `HashMap::from([...])` initializes from array of tuples
- `get_mut` returns `Option<&mut V>` — the mutable reference pattern
- `try_fold` is the Rust equivalent of OCaml fold with early exit
- Array variant shows Rust's strength: zero-allocation when structure is known
