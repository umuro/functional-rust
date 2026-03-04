# 812. Graph m-Colouring with Backtracking

**Difficulty:** 5  **Level:** Master

Assign colours to graph vertices so no two adjacent vertices share a colour — using backtracking to find valid assignments and compute the chromatic number.

## The Problem This Solves

Graph colouring models any problem where conflicting entities must be assigned distinct resources. Register allocation in compilers assigns CPU registers to variables, where variables that are "live" at the same time must get different registers — this is graph colouring on the interference graph. Exam scheduling assigns time slots so that no two exams share students. Frequency assignment in wireless networks ensures adjacent cells don't interfere. Map colouring (the classic four-colour theorem) is the historical application.

The chromatic number χ(G) — the minimum colours needed — characterises graph structure: χ = 1 iff no edges, χ = 2 iff bipartite, χ = 3 for planar graphs (often), χ = 5 for the Petersen graph... Computing χ is NP-complete, but backtracking works well for small graphs.

## The Intuition

Try colours 0..m for each vertex in order. For each vertex, try the first colour that doesn't conflict with already-coloured neighbours. If all m colours conflict, backtrack: uncolour the current vertex and try the next colour for the previous vertex. This is classic constraint satisfaction backtracking. The `is_safe` check scans the adjacency list in O(degree). OCaml uses recursive backtracking naturally; Rust uses recursive inner functions (which must be defined as `fn` inside the outer function, since closures can't recursively call themselves without boxing). The key Rust-ism: inner `fn` items can be declared inside functions and called recursively.

## How It Works in Rust

```rust
// O(m^V) worst case — backtracking with pruning
fn graph_color(adj: &[Vec<usize>], m: usize) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut color = vec![usize::MAX; n];  // MAX = uncoloured

    // is_safe: check no neighbour has colour c
    fn is_safe(v: usize, c: usize, adj: &[Vec<usize>], color: &[usize]) -> bool {
        adj[v].iter().all(|&u| color[u] != c)
    }

    // Recursive backtracking: assign colour to vertex v, then recurse to v+1
    fn solve(v: usize, n: usize, m: usize, adj: &[Vec<usize>], color: &mut Vec<usize>) -> bool {
        if v == n { return true; }  // all vertices coloured: success
        for c in 0..m {
            if is_safe(v, c, adj, color) {
                color[v] = c;
                if solve(v + 1, n, m, adj, color) { return true; }
                color[v] = usize::MAX;  // backtrack
            }
        }
        false
    }

    if solve(0, n, m, adj, &mut color) { Some(color) } else { None }
}

// Find chromatic number: try m = 1, 2, 3, ... until colouring succeeds
fn chromatic_number(adj: &[Vec<usize>]) -> usize {
    let n = adj.len();
    (1..=n).find(|&m| graph_color(adj, m).is_some()).unwrap_or(n)
}
// Petersen graph → χ = 3; K5 (complete 5-graph) → χ = 5
```

Defining `is_safe` and `solve` as inner `fn` (not closures) allows them to be called recursively. The `adj` and `color` parameters are passed explicitly — Rust inner functions don't capture outer scope variables, unlike OCaml local functions.

## What This Unlocks

- **Register allocation**: live variable analysis produces an interference graph; graph colouring assigns registers. Spilling occurs when χ > available registers. This is how production compilers (LLVM, GCC) allocate registers.
- **Exam/schedule assignment**: time-slot or resource assignment where conflicts (shared students, shared equipment) must be avoided — reduce to graph colouring and find a valid assignment.
- **Sudoku solving**: Sudoku is a graph colouring problem on a 81-node graph where cells in the same row/column/box are adjacent, with m=9 colours.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive backtracking | Local `let rec solve v = ...` captures outer state | Inner `fn solve(...)` with explicit parameters (no capture) |
| Uncoloured sentinel | `None` or `-1` | `usize::MAX` |
| Safe check | `List.for_all (fun u -> color.(u) <> c)` | `adj[v].iter().all(\|&u\| color[u] != c)` |
| Chromatic number search | `let rec find m = if colorable m then m else find (m+1)` | `(1..=n).find(\|&m\| graph_color(...).is_some())` |
| Backtrack | Reset array element | `color[v] = usize::MAX` |
