# Topological Sort — Comparison

## Core Insight
Topological sort highlights how OCaml and Rust handle mutable state differently. OCaml passes immutable tuples `(visited, order)` through recursive calls. Rust uses `&mut` references, requiring lifetime annotations when the recursive helper borrows string slices.

## OCaml Approach
- `Set.Make(String)` for visited nodes — persistent/immutable set
- State threaded as `(visited, order)` tuple through fold
- `SS.fold` iterates all nodes; inner `visit` recurses on neighbors
- No mutation — each recursive call returns updated state

## Rust Approach
- `HashSet` and `HashMap` — mutable, efficient hash-based collections
- `&mut visited` and `&mut order` passed to recursive helper
- Lifetime `'a` needed on `visit` to link borrowed `&str` to input edges
- `to_string()` at collection point to own the results

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Set type | `Set.Make(String)` | `HashSet<&str>` |
| State passing | Immutable tuple | `&mut` references |
| Lifetimes | Implicit (GC) | Explicit `'a` annotations |
| String handling | `string` (GC) | `&str` borrowed, `String` owned |
| Adjacency | `List.filter_map` | `HashMap<&str, Vec<&str>>` |

## Learner Notes
- Rust lifetime annotations on recursive functions can be tricky
- `HashMap::entry().or_default()` is idiomatic for building adjacency lists
- OCaml threading immutable state is elegant but can be harder to optimize
- Rust mutable approach is more imperative but avoids allocation per recursive call
