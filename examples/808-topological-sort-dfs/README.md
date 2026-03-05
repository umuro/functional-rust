# 808. DFS-Based Topological Sort

**Difficulty:** 3  **Level:** Advanced

Order nodes in a directed acyclic graph so all edges point forward — O(V + E) — with cycle detection via DFS colouring.

## The Problem This Solves

Topological sort answers: "in what order must I process these tasks given their dependencies?" Build systems (make, cargo, bazel) compute compilation order by topologically sorting the dependency graph. Package managers resolve install order. Spreadsheet engines evaluate cells in dependency order. Course prerequisite systems validate that a student's course sequence is valid.

The DFS-based approach is the natural counterpart to Kahn's BFS algorithm (which uses in-degree tracking). DFS topo-sort has the advantage of detecting cycles as a side effect: a back edge (grey-to-grey) during DFS means a cycle exists and no valid topological ordering is possible.

## The Intuition

DFS with three node states — white (unvisited), grey (in current DFS path), black (fully processed). A node is added to the output *after* all its descendants are processed (post-order). Because we add after processing, and then reverse the list, we get topological order: every edge `u → v` has u before v. A back edge (visiting a grey node) means we've found a cycle — immediately return an error. OCaml expresses this with recursive DFS and an exception for cycle detection; Rust uses an explicit stack with `Vec<(node, adj_index)>` to avoid recursion depth limits.

## How It Works in Rust

```rust
// O(V + E), returns Ok(order) or Err(cycle_node)
fn topo_sort(adj: &[Vec<usize>]) -> Result<Vec<usize>, usize> {
    let n = adj.len();
    let mut state  = vec![0u8; n]; // 0=white, 1=grey, 2=black
    let mut result = Vec::new();

    for start in 0..n {
        if state[start] != 0 { continue; }
        // Iterative DFS: (node, index into adj[node])
        let mut stack: Vec<(usize, usize)> = vec![(start, 0)];
        state[start] = 1; // grey: currently on DFS path

        while let Some((u, idx)) = stack.last_mut() {
            let u = *u;
            if *idx < adj[u].len() {
                let v = adj[u][*idx]; *idx += 1;
                match state[v] {
                    1 => return Err(v),  // back edge → cycle
                    0 => { state[v] = 1; stack.push((v, 0)); }
                    _ => {}              // black: already processed, skip
                }
            } else {
                // Post-order: all neighbours done, finalise u
                state[u] = 2;    // black
                result.push(u);
                stack.pop();
            }
        }
    }
    result.reverse();  // post-order gives reverse topo order
    Ok(result)
}
```

The three-colour scheme is the standard DFS cycle-detection technique. Grey nodes form the current DFS path stack; encountering a grey node means we've found a back edge (cycle). The `Result` return type cleanly separates the success and cycle-detected cases — idiomatic Rust over using exceptions.

## What This Unlocks

- **Build systems**: Cargo, Bazel, and Make all compute topological sort over the dependency graph to determine compilation order — processes a node only after all its dependencies are built.
- **Task scheduling**: schedule jobs with precedence constraints. Topo-sort gives a valid execution order; the level structure (nodes at the same depth) identifies which tasks can run in parallel.
- **Spreadsheet evaluation**: each cell is a node; formulas create dependency edges. Topo-sort gives the cell evaluation order; back edges detect circular references.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| DFS state | `mutable` tri-color array or `Hashtbl` | `Vec<u8>` with 0/1/2 values |
| Cycle detection | `raise Cycle v` exception | `return Err(v)` — `Result` type |
| Recursion | Natural recursive DFS | Explicit `Vec<(usize, usize)>` stack |
| Post-order | Prepend to list (natural ordering) | Push to `Vec`, then `reverse()` |
| Return type | `(int list, bool)` or raises | `Result<Vec<usize>, usize>` |
