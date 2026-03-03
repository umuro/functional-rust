# Topological Sort — OCaml vs Rust Comparison

## Core Insight

Topological sort reveals how each language handles graph traversal state. OCaml threads immutable `(visited_set, result_list)` pairs through recursive calls — purely functional but verbose. Rust passes mutable references to `HashSet` and `Vec` — more explicit about mutation but closer to the imperative algorithm.

## OCaml Approach

Uses `Set.Make(String)` for the visited set. The `visit` function takes and returns `(visited, order)` tuples — no mutation, but every recursive call creates new tuples. `SS.fold` iterates over all nodes to handle disconnected components. The functional style ensures referential transparency.

## Rust Approach

Two algorithms: (1) DFS with `&mut HashSet` and `&mut Vec` passed to recursive `visit` — clear ownership of mutable state. (2) Kahn's BFS algorithm using in-degree counting. The Rust version explicitly allocates `HashMap` for adjacency lists upfront, making the graph structure clear.

## Comparison Table

| Aspect        | OCaml                           | Rust                                  |
|---------------|---------------------------------|---------------------------------------|
| **Memory**    | Immutable sets (tree nodes)     | HashSet + Vec (hash table + array)    |
| **Null safety** | N/A                          | N/A                                   |
| **Errors**    | No cycle detection              | No cycle detection (add if needed)    |
| **Iteration** | Recursive with tuple threading  | Recursive with `&mut` references      |
| **State**     | Functional (pass-and-return)    | Mutable references (borrow checker)   |

## Things Rust Learners Should Notice

1. **`&mut` in recursive functions** — passing mutable refs down the call stack is idiomatic for graph algorithms
2. **`HashMap::entry().or_default()`** — builds adjacency lists cleanly
3. **Lifetime annotations `'a`** — needed when storing references to input data in the visited set
4. **Two algorithms** — DFS (post-order reverse) vs Kahn's (BFS with in-degree) — same result, different trade-offs
5. **Deterministic output** — sorting nodes before iteration ensures reproducible results in tests

## Further Reading

- [Topological sort (Wikipedia)](https://en.wikipedia.org/wiki/Topological_sorting)
- [petgraph crate](https://docs.rs/petgraph/) — production-quality graph algorithms for Rust
