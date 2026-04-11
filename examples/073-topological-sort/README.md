📖 **[View on hightechmind.io →](https://hightechmind.io/rust/073-topological-sort)**

---

# 073 — Topological Sort

## Problem Statement

Topological sort orders the nodes of a directed acyclic graph (DAG) such that for every directed edge (u → v), node u appears before v in the ordering. It answers the question: "in what order should I complete these tasks, given their dependencies?" This is one of the most widely used graph algorithms in practice.

Build systems (Bazel, Make, Cargo) topologically sort compilation units so dependencies compile before dependents. Package managers (npm, pip, apt) sort package installations so transitive dependencies install first. Spreadsheet engines recalculate cells in topological order. Database engines plan query execution and schema migrations using topological ordering.

The DFS-based algorithm (Tarjan, 1976) marks each node visited, recurses on all neighbors first, then adds the current node — this is post-order DFS. Reversing the post-order gives a valid topological ordering. Kahn's algorithm (BFS-based) is an alternative that processes nodes with in-degree 0 iteratively and naturally detects cycles.

## Learning Outcomes

- Implement DFS-based topological sort with a visited set and post-order accumulation
- Build an adjacency list from edge pairs using `HashMap<&str, Vec<&str>>`
- Use a `HashSet` for O(1) visited checking
- Understand the post-order property: a node is added AFTER all its descendants
- Recognize that the reversed post-order DFS is a valid topological ordering

## Rust Application

`topo_sort(edges: &[(&str, &str)]) -> Vec<String>` builds an adjacency list, then calls a nested `visit` function for each unvisited node. `visit` marks the node as visited, recurses on all neighbors, then `push`es the node to `order` — post-order. The final `order` is in topological order (dependencies come before dependents after a `reverse()`).

## OCaml Approach

OCaml uses a hash table for the visited set and a reference list for accumulation:

```ocaml
let topo_sort edges =
  let visited = Hashtbl.create 16 in
  let order = ref [] in
  let adj = build_adj edges in
  let rec visit node =
    if not (Hashtbl.mem visited node) then begin
      Hashtbl.add visited node ();
      List.iter visit (List.assoc_opt node adj |> Option.value ~default:[]);
      order := node :: !order
    end
  in
  List.iter (fun (a, b) -> visit a; visit b) edges;
  !order
```

The `order := node :: !order` prepends in post-order, giving the correct topological sequence without an explicit reverse step.

## Key Differences

1. **Nested function**: Rust's nested `fn visit<'a>(...)` inside `topo_sort` requires explicit lifetime annotations for the borrowed adjacency map. OCaml closures capture the environment implicitly.
2. **`HashSet` vs `Hashtbl`**: Rust's `HashSet` for visited nodes. OCaml's `Hashtbl` serves both set and map purposes. Both provide O(1) lookup.
3. **Cycle detection**: This implementation does not detect cycles (silently handles them by skipping visited nodes, which may produce incorrect output). A full implementation needs a "in progress" visited state (three-color DFS).
4. **Ordering**: The post-order accumulation produces dependencies-last. Reversing gives dependencies-first. Kahn's algorithm naturally produces dependencies-first without reversal.

## Exercises

1. **Cycle detection**: Extend `topo_sort` to return `Result<Vec<String>, Vec<String>>` where the error contains the nodes in the detected cycle. Use three-color DFS: white (unvisited), gray (in progress), black (done).
2. **Kahn's algorithm**: Implement topological sort using Kahn's algorithm: repeatedly remove nodes with in-degree 0. Return `None` if a cycle prevents completion.
3. **Build order**: Given Cargo-style `[[dependencies]]` as a list of `(package, depends_on)` pairs, produce a valid build order using topological sort.
