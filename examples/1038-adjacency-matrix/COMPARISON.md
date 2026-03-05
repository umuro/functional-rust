# Adjacency Matrix — Comparison

## Core Insight
A 2D boolean matrix is the simplest graph representation: `matrix[i][j] = true` means edge from i to j. Both languages use the same approach — mutable 2D arrays. The key difference is OCaml's `Array.make_matrix` vs Rust's `vec![vec![false; n]; n]`.

## OCaml Approach
- `Array.make_matrix n n false` creates the matrix
- Direct indexing: `matrix.(i).(j)`
- `Array.fold_left` for degree counting
- Triple nested loop for Warshall's algorithm
- `Array.copy` for non-destructive operations

## Rust Approach
- `vec![vec![false; n]; n]` macro for initialization
- Direct indexing: `matrix[i][j]`
- Iterator methods: `.filter(|&&c| c).count()` for degree
- `.enumerate().filter_map()` for neighbor listing
- `.clone()` on outer Vec for non-destructive copy

## Comparison Table

| Feature | OCaml | Rust |
|---|---|---|
| Creation | `Array.make_matrix n n false` | `vec![vec![false; n]; n]` |
| Edge check | `m.(i).(j)` | `m[i][j]` |
| Edge add | `m.(i).(j) <- true` | `m[i][j] = true` |
| Copy | `Array.init n (fun i -> Array.copy m.(i))` | `m.clone()` |
| Neighbor list | `filter_map` on array | `.enumerate().filter_map()` |
| Memory | O(n²) | O(n²) |
| Edge lookup | O(1) | O(1) |
