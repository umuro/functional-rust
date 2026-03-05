# Sparse Matrix — Comparison

## Core Insight
A sparse matrix stores only non-zero entries, saving memory when most values are 0. Both languages use a hash map from `(row, col)` pairs to floats. OCaml requires a custom hashtable module (`Hashtbl.Make`) because standard `Hashtbl` needs a custom hash for tuple keys. Rust's `HashMap<(usize, usize), f64>` works out of the box — tuples derive `Hash` automatically.

## OCaml Approach
- `module IntPair = struct type t = int * int; let equal ...; let hash ... end`
- `module PairHash = Hashtbl.Make(IntPair)` — functor application for typed hashtable
- `PairHash.find_opt m.data (r,c) |> Option.value ~default:0.0`
- `PairHash.remove` when setting to 0 (keep sparsity invariant)
- `PairHash.iter` for matvec and transpose iteration
- Floats compared with `= 0.0` (works for exact zero)

## Rust Approach
- `HashMap<(usize, usize), f64>` — tuple key, hash derived automatically
- `.unwrap_or(&0.0)` for zero default
- `.remove(&(r, c))` when setting to 0.0
- `for (&(r, c), &val) in &self.data` — destructuring in for loop
- `.entry((r,c)).or_insert(0.0)` for accumulate-or-init pattern
- Same float-zero comparison: `v == 0.0`

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Tuple key hash | `Hashtbl.Make(IntPair)` functor | `HashMap<(usize,usize), f64>` (auto-Hash) |
| Default zero | `Option.value ~default:0.0` | `.unwrap_or(&0.0)` |
| Remove zero | `PairHash.remove m.data key` | `data.remove(&key)` |
| Iteration | `PairHash.iter (fun (r,c) v -> ...)` | `for (&(r,c), &v) in &data` |
| Accumulate | `existing +. v; replace` | `.entry(k).or_insert(0.0)` then `*e += v` |
| nnz | `PairHash.length` | `data.len()` |
| Index check | `failwith` | `assert!` |
