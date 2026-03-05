# Union-Find / Disjoint Set — Comparison

## Core Insight
Union-Find stores a forest where each tree represents a connected component. `find(x)` walks up to the root, compressing the path. `union(a,b)` merges two trees by rank. The algorithm is identical; OCaml's recursive `find` is elegant, while Rust prefers iterative path compression to avoid stack overflow on degenerate inputs.

## OCaml Approach
- `Array.init n (fun i -> i)` — each node is its own parent
- Recursive `find` with in-place path compression: `parent.(i) <- find uf parent.(i)`
- `mutable components: int` in record for component count
- Pattern matching on rank comparison: `if rank.(ra) < rank.(rb)`
- Returns `bool` from `union` — true if sets were merged

## Rust Approach
- `(0..n).collect()` — each node is its own parent
- Iterative two-pass path compression (find root, then compress)
- `self.components` field decremented on successful union
- `std::cmp::Ordering` match for rank comparison
- `&mut self` required on `find` (modifies parent array)

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Storage | `int array` | `Vec<usize>` |
| Init | `Array.init n (fun i -> i)` | `(0..n).collect()` |
| Find | Recursive with path compression | Iterative two-pass |
| Path compress | `parent.(i) <- find uf parent.(i)` | Loop: `parent[i] = root` |
| Rank compare | `if rank < rank` | `match rank.cmp(&rank)` |
| Mutation | Mutable record fields | `&mut self` |
| Component count | `mutable components: int` | `self.components: usize` |
| Amortized cost | O(α(n)) inverse Ackermann | O(α(n)) same |
