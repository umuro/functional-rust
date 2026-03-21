📖 **[View on hightechmind.io →](https://hightechmind.io/rust/379-directed-acyclic-graph)**

---

# 379: Directed Acyclic Graph (DAG) and Topological Sort
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Many real-world problems require ordering tasks where some must precede others: build systems (compile A before B if B imports A), package managers (install dependencies before dependents), course prerequisites, and spreadsheet cell evaluation. A Directed Acyclic Graph (DAG) models these dependency relationships, and topological sort produces a valid linear ordering. If the graph contains a cycle, no valid ordering exists — which signals a circular dependency error.

Topological sort powers `make`, `cargo`, `npm`, `pip`, Makefiles, Apache Airflow DAG scheduling, and CPU instruction scheduling in compilers.

## Learning Outcomes

- Understand why cycles make topological ordering impossible and how to detect them
- Learn Kahn's algorithm (in-degree based, BFS) vs. DFS post-order reversal
- Understand how in-degree tracking drives the BFS-based approach
- See how `Option<Vec<usize>>` encodes success/failure of cycle detection in Rust
- Learn why DAGs are foundational for dependency resolution systems

## Rust Application

The `topological_sort` function in `src/lib.rs` implements Kahn's algorithm: compute in-degrees, enqueue zero-in-degree vertices, repeatedly dequeue and reduce neighbors' in-degrees. If the result length equals `n`, the graph is acyclic. `has_cycle` delegates to this and checks for `None`. The DFS version uses a three-color visited array (0=unvisited, 1=in-progress, 2=done) with an inner recursive function — Rust's closure recursion requires a separate named `fn` inside the outer function.

## OCaml Approach

OCaml's functional style handles topological sort with recursive DFS naturally. The visited state is a mutable integer array or a `Hashtbl`. The DFS post-order collects into an accumulator list, then reverses it. OCaml's pattern matching handles the three-state visited array cleanly. Kahn's algorithm uses `Queue.t` for the BFS frontier.

## Key Differences

1. **Recursion style**: OCaml handles recursive DFS cleanly as a local recursive function; Rust requires an inner `fn` (not closure) for recursive calls since closures cannot call themselves.
2. **Result encoding**: Rust uses `Option<Vec<usize>>` where `None` means cycle detected; OCaml returns `('a list, unit) result` or raises an exception.
3. **Mutability**: Rust's in-degree array is `mut` with explicit `&mut` references; OCaml uses a mutable `int array` with direct assignment.
4. **Visited marking**: Rust uses `u8` array with constants (0/1/2); OCaml typically uses `Hashtbl` with variant values like `type color = White | Gray | Black`.

## Exercises

1. **Longest path in DAG**: Implement a DP algorithm using topological order to find the longest path in a weighted DAG — the critical path in project scheduling.
2. **All topological orderings**: Generate all valid topological orderings of a small DAG (backtracking over zero-in-degree choices at each step).
3. **Dependency resolver**: Model a package manager: packages have version requirements and dependencies. Build a DAG and use topological sort to produce a valid install order, returning an error message identifying any circular dependencies.
