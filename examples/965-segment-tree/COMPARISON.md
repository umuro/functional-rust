# Segment Tree — Comparison

## Core Insight
A segment tree stores aggregate values (sums, min, max) for array ranges in a complete binary tree laid out in an array. Node `i` covers a range; its children at `2i` and `2i+1` cover the halves. Both OCaml and Rust implement identical recursive build/update/query — the tree layout and algorithm are language-agnostic.

## OCaml Approach
- `Array.make (4 * n) 0` — 4n slots ensures enough space for any n
- Recursive `build`, `update`, `query` as top-level functions
- Public wrappers `st_build`, `st_update`, `st_query` start at node 1
- `st.tree.(node) <- st.tree.(2*node) + st.tree.(2*node+1)` — push-up
- Early returns `0` for out-of-range, `st.tree.(node)` for fully covered

## Rust Approach
- `vec![0i64; 4 * n]` — same layout
- Private `build_rec`, `update_rec`, `query_rec` methods on struct
- Public `build`, `update`, `query` as clean API
- `self.tree[node] = self.tree[2*node] + self.tree[2*node+1]` — push-up
- `usize` indices (no negative — avoids signed/unsigned confusion)

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Storage | `int array` (4n) | `Vec<i64>` (4n) |
| Recursion | Top-level functions with `st` arg | Private `_rec` methods |
| Index type | `int` | `usize` |
| Push-up | `tree.(n) <- tree.(2n) + tree.(2n+1)` | `tree[n] = tree[2n] + tree[2n+1]` |
| Range miss | `0` | `0` |
| Range cover | `tree.(node)` | `tree[node]` |
| Split point | `mid = (lo + hi) / 2` | `mid = (lo + hi) / 2` |
| Build init | `Array.make (4*n) 0` | `vec![0i64; 4*n]` |
