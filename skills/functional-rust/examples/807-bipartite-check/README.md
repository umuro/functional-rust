# 807: Bipartite Graph Detection (2-Colouring)

**Difficulty:** 3  **Level:** Advanced

Determine whether a graph can be 2-coloured — equivalent to detecting odd cycles — using BFS in O(V+E).

## The Problem This Solves

A graph is bipartite if its vertices can be split into two sets such that every edge connects a vertex in one set to a vertex in the other — never two vertices in the same set. This is equivalent to asking: can you 2-colour the graph? And equivalently: does it contain any odd-length cycle?

Bipartiteness appears in matching problems (job assignments, stable marriage), scheduling (tasks and machines), and database query optimization. Any time you have a "two-sided" relationship — users and items in a recommendation system, students and courses — you're working with a bipartite structure. Checking bipartiteness before running a matching algorithm saves you from applying algorithms that require it when the input violates the assumption.

The algorithm returns either a valid 2-colouring (each vertex labelled 0 or 1) or a proof of non-bipartiteness via the odd cycle. Even for disconnected graphs, each component is checked independently.

## The Intuition

BFS naturally enforces 2-colouring. Start at any vertex, colour it 0. Colour all its neighbours 1. Colour their unvisited neighbours 0. If you ever try to colour a vertex that already has the same colour as its neighbour, you've found an odd cycle — the graph is not bipartite.

The assignment `color[w] = 1 - color[v]` is the entire logic: flip the colour at each BFS level. If a cross-edge connects two same-coloured vertices, the cycle length is even+1 = odd.

O(V+E) time. This is one of the cleanest BFS applications: no visited array needed separately — uncoloured vertices serve as "unvisited." In OCaml, you'd use `Option` to represent uncoloured. In Rust, `i8` with -1 as sentinel is compact and cache-friendly.

## How It Works in Rust

```rust
use std::collections::VecDeque;

fn is_bipartite(adj: &[Vec<usize>]) -> Option<Vec<i8>> {
    let n = adj.len();
    let mut color = vec![-1i8; n]; // -1 = uncoloured

    for start in 0..n {
        if color[start] != -1 { continue; } // already coloured

        // BFS from this component's root
        color[start] = 0;
        let mut queue = VecDeque::from([start]);

        while let Some(v) = queue.pop_front() {
            for &w in &adj[v] {
                if color[w] == -1 {
                    color[w] = 1 - color[v]; // flip colour
                    queue.push_back(w);
                } else if color[w] == color[v] {
                    return None; // same colour on both ends → odd cycle
                }
            }
        }
    }
    Some(color) // valid 2-colouring
}
```

`VecDeque` from the standard library is Rust's double-ended queue — the natural choice for BFS. `pop_front` is O(1) amortised. Using `i8` instead of an `Option<u8>` avoids the overhead of matching and keeps the color array tight in memory.

The `1 - color[v]` trick works because colours are 0 and 1 — flipping between them with subtraction avoids a branch.

## What This Unlocks

- **Maximum bipartite matching**: König's theorem, Hopcroft-Karp, Hungarian algorithm — all require a bipartite graph as input.
- **Graph colouring lower bound**: a graph that isn't bipartite requires at least 3 colours; bipartite graphs are exactly the 2-colourable ones.
- **Cycle parity in dependency graphs**: detecting odd cycles in constraint graphs (e.g., in 2-SAT preprocessing or scheduling feasibility).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Uncoloured sentinel | `None` in `int option array` | `-1i8` — avoids `Option` wrapping overhead |
| Queue | `Queue.t` module | `VecDeque<usize>` from `std::collections` |
| Colour flip | `1 - c` or `match c` | `1 - color[v]` — same idiom |
| Disconnected graph | Explicit loop over components | Same outer `for start in 0..n` loop |
| Early exit | `raise Exit` or `Result` | `return None` — idiomatic `Option` return |
