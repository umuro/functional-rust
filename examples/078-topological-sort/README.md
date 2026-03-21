📖 **[View on hightechmind.io →](https://hightechmind.io/rust/078-topological-sort)**

---

# 078 — Topological Sort (Ownership Focus)
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

This example revisits topological sort (see example 073) with explicit focus on ownership in the DFS algorithm. The adjacency map borrows string slices (`&str`), the visited set borrows node names, and the result vector owns `String` values. Managing these lifetimes correctly is a concrete exercise in Rust's borrow checker.

Understanding how borrowed data flows through recursive algorithms — where closures cannot be used because of borrow conflicts — illustrates why Rust sometimes requires nested function definitions with explicit lifetime parameters.

## Learning Outcomes

- Build an adjacency map `HashMap<&str, Vec<&str>>` from borrowed edge data
- Use a `HashSet<&str>` for visited tracking with lifetime annotations
- Implement DFS with a nested function that takes explicit lifetime parameters
- Understand why `visited: &mut HashSet<&str>` requires `'a` on the node references
- Compare with the HashMap-and-clone approach for simpler lifetime management

## Rust Application

The nested `fn visit<'a>(node: &'a str, adj: &HashMap<&str, Vec<&'a str>>, visited: &mut HashSet<&'a str>, order: &mut Vec<String>)` borrows the adjacency list and shares the lifetime `'a` across node references. This allows the visited set to hold `&'a str` references pointing into the original edge data. The `order` stores owned `String` values.

## OCaml Approach

OCaml's DFS has no ownership concerns — all references are GC-managed. `let visit = fun node -> if not (Hashtbl.mem visited node) then begin ... end` is a simple closure capturing `adj`, `visited`, and `order` by reference. No lifetime annotations needed; the GC ensures all borrowed values remain valid.

## Key Differences

1. **Lifetime annotations**: Rust requires explicit `'a` to connect the lifetimes of `node: &'a str` and `visited: &mut HashSet<&'a str>`. OCaml's GC eliminates this bookkeeping.
2. **Nested function vs closure**: Rust's nested `fn visit` is a free function (not a closure) and cannot capture from the outer scope — each parameter must be passed explicitly. A closure would simplify this but creates borrow conflicts.
3. **`&str` in HashSet**: `HashSet<&str>` stores borrowed references. `HashSet<String>` stores owned copies. The `&str` version is faster (no clone) but requires lifetime management. OCaml uses string values directly.
4. **`order.push(node.to_string())`**: Converting `&str` to `String` at push time converts from borrowed to owned. OCaml's `order := node :: !order` works directly with string values.

## Exercises

1. **String-owned version**: Rewrite `topo_sort` to use `HashMap<String, Vec<String>>` and `HashSet<String>` — clone-based, no lifetimes needed. Benchmark vs the `&str` version.
2. **Parallel DFS**: Describe the challenges of parallelizing DFS-based topological sort. What data structures need to be concurrent? When would this be beneficial?
3. **Incremental sort**: Write `IncrementalTopoSort` that allows adding edges one at a time and queries "is there a topological order?" and "what is the current order?" efficiently.
