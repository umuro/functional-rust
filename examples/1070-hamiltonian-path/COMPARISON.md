# Hamiltonian Path — Comparison

## Core Insight
Finding a Hamiltonian path is NP-complete. Backtracking explores all orderings; bitmask DP (Held-Karp style) checks existence in O(2^n × n^2). The bitmask approach represents visited-node sets as integers, enabling efficient DP.

## OCaml Approach
- `Array.fill` for resetting path/visited between start vertices
- `ref` flag for early exit in inner loop
- `Array.to_list` for result conversion
- No bitmask DP shown (less natural in OCaml)

## Rust Approach
- `vec![false; n]` for visited tracking
- Inner `fn` with explicit mutable references
- Bitmask DP: `vec![vec![false; n]; 1 << n]` — 2D table indexed by (mask, node)
- `(0..n).any()` for checking if any ending node reaches full mask

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Reset arrays | `Array.fill path 0 n (-1)` | Recreate `vec![0; n]` per start |
| Bitmask DP | Not idiomatic | `1 << n` + bitwise ops — natural |
| Early exit | `ref` flag | `return true` |
| Complexity | O(n!) backtracking | O(2^n × n^2) bitmask DP |
| Path reconstruction | `Array.to_list` | Return `Vec<usize>` directly |
