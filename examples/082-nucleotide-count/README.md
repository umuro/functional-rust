[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 082 — Nucleotide Count
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Count the occurrences of each DNA nucleotide (`A`, `C`, `G`, `T`) in a string, returning a `HashMap<char, usize>` or an error indicating the first invalid character. Implement three versions — imperative loop, `try_fold`, and array-backed — and compare with OCaml's `Map.Make(Char)` and mutable `ref`-based approaches.

## Learning Outcomes

- Initialise a `HashMap` with known keys and zero counts using array-into-iter
- Use `get_mut` to increment in-place without a double lookup
- Apply `try_fold` with the `?` operator to combine accumulation and early error return
- Recognise when an array `[usize; 4]` beats `HashMap` for fixed-key counting
- Map Rust's `Result<T, E>` early return to OCaml's `failwith` exception
- Understand `chars()` vs `bytes()` for ASCII-only DNA strings

## Rust Application

`nucleotide_count` initialises a `HashMap` with `[('A', 0), ('C', 0), ('G', 0), ('T', 0)].into()`, then iterates `dna.chars()`. For each character, `get_mut` returns a mutable reference to the count or `None` for invalid characters — the `None` arm returns `Err(c)` immediately. The `try_fold` version composes this into a single expression: each step returns `Ok(acc)` or `Err(c)`, and the `?` operator propagates errors automatically. The array version maps `A/C/G/T` to indices 0–3 using a `match`, avoiding hashing entirely for a fixed alphabet.

## OCaml Approach

OCaml uses a functional map `CMap = Map.Make(Char)` with `fold_left` to initialise zero counts and `String.fold_left` to accumulate. `find_opt` returns `None` for invalid characters, triggering `failwith`. The second version uses mutable `ref` cells in an association list — closer to the imperative loop. OCaml lacks `try_fold` but achieves the same effect through exceptions, which are lightweight in OCaml and commonly used for early exit.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Error return | `Result<_, char>` with `?` | Exception via `failwith` |
| Map type | `HashMap<char, usize>` | `Map.Make(Char)` (balanced tree) |
| In-place update | `get_mut` + `*n += 1` | `CMap.add c (n+1) m` (persistent) |
| `try_fold` | Built into `Iterator` | Custom recursion or exception |
| Performance alt | `[usize; 4]` array | Array or `Bytes` |
| String iteration | `.chars()` | `String.fold_left` / `String.iter` |

The array version demonstrates an important optimisation pattern: when the key space is small and known, replace `HashMap` with an array indexed by position — O(1) access with no hashing overhead.

## Exercises

1. Extend `nucleotide_count` to also accept lowercase `a`, `c`, `g`, `t` by normalising with `.to_ascii_uppercase()`.
2. Add a `gc_content(dna: &str) -> Result<f64, char>` function that returns the fraction of `G` and `C` nucleotides.
3. Implement a parallel version using `rayon::par_iter()` that splits the string and merges partial counts.
4. Write a function `complement(dna: &str) -> Result<String, char>` that returns the complement strand (`A↔T`, `G↔C`).
5. In OCaml, implement a version using `Bytes` with mutable array slots indexed by `Char.code` for O(1) update performance.
