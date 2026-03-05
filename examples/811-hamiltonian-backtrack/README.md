📖 **[View on hightechmind.io →](https://hightechmind.io/rust/811-hamiltonian-backtrack)**

---

# 811: Hamiltonian Cycle via Backtracking

**Difficulty:** 5  **Level:** Master

Find a cycle that visits every vertex exactly once — NP-complete, solved by exhaustive backtracking with pruning.

## The Problem This Solves

A Hamiltonian cycle is a closed path through a graph that visits every vertex exactly once. Unlike Eulerian circuits (every *edge* once, solvable in O(V+E)), the Hamiltonian version is NP-complete — no polynomial algorithm is known, and none is expected.

Use backtracking for Hamiltonian path/cycle on small graphs (up to ~20 vertices in practice). It appears in route planning (visit every city exactly once and return), puzzle solving (knight's tour on a chessboard), and as a subroutine in TSP solvers. For larger instances you'd switch to dynamic programming (Held-Karp, O(2^n · n²)) or heuristics.

The algorithm returns `Some(path)` if a Hamiltonian cycle exists, or `None` if the graph has none. The path is a vector of vertex indices forming the cycle (first vertex = last vertex implicitly).

## The Intuition

Build the path one vertex at a time. At each step, try every unvisited neighbour of the current endpoint. If you reach a dead end (no valid next vertex), backtrack and try a different choice. When all vertices are visited, check if there's an edge back to the start — if yes, you have a Hamiltonian cycle.

Worst case: O(n!) — trying every permutation. In practice, pruning (only extend via actual edges) cuts this dramatically for sparse graphs. For dense graphs approaching O(n²) edges, the exponential character dominates.

In OCaml, backtracking fits naturally into recursive functions with immutable path tracking. In Rust, you use a mutable `visited` bitset and `path` vector, undoing changes on backtrack — explicit undo/redo rather than functional state threading.

## How It Works in Rust

```rust
fn hamiltonian_cycle(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = adj.len();
    if n == 0 { return Some(vec![]); }

    let mut path = vec![0usize];       // start at vertex 0
    let mut visited = vec![false; n];
    visited[0] = true;

    if backtrack(adj, &mut path, &mut visited, n) {
        Some(path)
    } else {
        None
    }
}

fn backtrack(
    adj: &[Vec<usize>],
    path: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    n: usize,
) -> bool {
    if path.len() == n {
        // All vertices visited — check if last connects back to start
        let last  = *path.last().unwrap();
        let start = path[0];
        return adj[last].contains(&start);
    }

    let current = *path.last().unwrap();
    for &next in &adj[current] {
        if !visited[next] {
            // Extend path
            visited[next] = true;
            path.push(next);

            if backtrack(adj, path, visited, n) { return true; }

            // Undo (backtrack)
            path.pop();
            visited[next] = false;
        }
    }
    false // dead end
}
```

The mutable `visited` and `path` with explicit push/pop replace what functional languages handle via immutable persistent data structures or explicit state copying. Rust's borrow checker ensures no accidental aliasing during the recursive calls — `adj` is borrowed immutably while `path` and `visited` are mutably borrowed, which is safe because they're disjoint.

For production use, sort `adj[current]` by degree (try low-degree vertices first) to prune earlier. A `visited` bitset using `u64` shifts is faster than `Vec<bool>` for small graphs.

## What This Unlocks

- **Puzzle solving**: knight's tour, Gray codes, and combinatorial games with "visit each state once" constraints.
- **Route optimisation seed**: backtracking finds exact solutions for small TSP instances before handing off to heuristics.
- **NP-completeness demonstrations**: Hamiltonian cycle is the canonical NP-complete graph problem — understanding it grounds complexity theory.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Backtrack state | Immutable list threaded through recursion | Mutable `Vec` with push/pop (explicit undo) |
| Visited set | `bool array` with manual reset | Same — `vec![false; n]` with `visited[next] = false` |
| Recursion depth | May stack-overflow for n > ~5000 | Same risk — convert to iterative for very large n |
| Path return | Accumulator passed through | Mutable `path: &mut Vec<usize>` |
| NP hardness | Same algorithm, same complexity | Same — no Rust magic makes NP problems polynomial |
