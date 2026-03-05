📖 **[View on hightechmind.io →](https://hightechmind.io/rust/813-minimum-vertex-cover)**

---

# 813: Minimum Vertex Cover (2-Approximation)

**Difficulty:** 4  **Level:** Advanced

Find a near-minimal set of vertices that touches every edge — with a provable 2× approximation guarantee plus an optional exact backtracking solver.

## The Problem This Solves

A vertex cover is a set S of vertices such that every edge in the graph has at least one endpoint in S. The minimum vertex cover (MVC) is the smallest such set. MVC is NP-hard in general graphs, but a simple greedy algorithm gives a 2-approximation: the result is at most twice the optimal size.

MVC appears in network security (place monitors on a minimal set of routers to observe all traffic), VLSI testing (select minimal probe points to test all connections), and computational biology (identify minimal gene sets that interact with all observed pathways). By König's theorem, MVC equals maximum matching size in bipartite graphs — making it polynomial in that special case.

This example implements both: the O(V+E) 2-approximation (practical for large graphs) and an exact backtracking solver (correct but exponential) so you can see the approximation ratio empirically.

## The Intuition

**2-approximation**: pick any uncovered edge (u, v). Add both u and v to the cover. Remove all edges incident to u or v. Repeat until no edges remain. This is the "maximal matching" greedy: every edge you pick is in some maximum matching (since you can't reuse vertices), and OPT must cover each matching edge with at least one vertex. So |cover| = 2 × |matching| ≤ 2 × OPT.

**Exact backtracking**: for each uncovered edge (u, v), branch: include u OR include v. Recurse on the reduced graph. Prune when current cover size exceeds best known. This is the standard FPT algorithm parameterized by the solution size k — runs in O(2^k · (V+E)).

O(V+E) for the approximation. O(2^k · n) for exact, where k is the optimal cover size.

## How It Works in Rust

```rust
use std::collections::HashSet;

// 2-approximation: maximal matching → both endpoints
fn approx_vertex_cover(adj: &[Vec<usize>]) -> HashSet<usize> {
    let n = adj.len();
    let mut cover = HashSet::new();
    let mut matched = vec![false; n]; // vertices already in cover

    for u in 0..n {
        if matched[u] { continue; }
        for &v in &adj[u] {
            if !matched[v] {
                // Edge (u,v) not yet covered — add both endpoints
                cover.insert(u);
                cover.insert(v);
                matched[u] = true;
                matched[v] = true;
                break; // move to next u
            }
        }
    }
    cover
}

// Exact solver: branch on uncovered edge (u,v) — try u alone, then v alone
fn exact_vertex_cover(adj: &[Vec<usize>]) -> HashSet<usize> {
    let mut best = HashSet::new();
    // Seed with approximation so pruning is tight from the start
    best = approx_vertex_cover(adj);
    let mut current = HashSet::new();
    backtrack_cover(adj, &mut current, &mut best);
    best
}

fn backtrack_cover(
    adj: &[Vec<usize>],
    current: &mut HashSet<usize>,
    best: &mut HashSet<usize>,
) {
    // Find first uncovered edge
    let uncovered = (0..adj.len()).flat_map(|u| {
        adj[u].iter().map(move |&v| (u, v))
    }).find(|&(u, v)| !current.contains(&u) && !current.contains(&v));

    match uncovered {
        None => {
            // All edges covered — update best if smaller
            if current.len() < best.len() { *best = current.clone(); }
        }
        Some((u, v)) => {
            // Pruning: if current already ≥ best, abandon
            if current.len() >= best.len() { return; }
            // Branch: include u
            current.insert(u);
            backtrack_cover(adj, current, best);
            current.remove(&u);
            // Branch: include v
            current.insert(v);
            backtrack_cover(adj, current, best);
            current.remove(&v);
        }
    }
}
```

`HashSet` operations (insert, remove, contains) are O(1) average. The `flat_map` iterator finds the first uncovered edge lazily — it stops as soon as it finds one, which is efficient. Seeding `best` with the approximation ensures tight pruning from the first branch.

## What This Unlocks

- **Network monitoring**: place the minimum number of IDS sensors on routers to observe every link in the network.
- **Bipartite matching duality**: on bipartite graphs, MVC = maximum matching (König's theorem) — solvable in polynomial time via max-flow.
- **Approximation algorithm study**: this is the canonical example of a polynomial-time constant-factor approximation for an NP-hard problem.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable set | `Hashtbl` or `Set` with functional threading | `HashSet<usize>` with insert/remove |
| Backtrack undo | Functional — pass new set without vertex | Imperative — `insert` then `remove` after recursion |
| Iterator over edges | `Array.fold` + `List.iter` | `flat_map` over adjacency list — lazy, zero-copy |
| Pruning condition | Guard clause before recursive call | `if current.len() >= best.len() { return; }` |
| Approximation seeding | Same technique | Clone approx result into `best` before backtracking |
