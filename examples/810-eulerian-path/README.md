📖 **[View on hightechmind.io →](https://hightechmind.io/rust/810-eulerian-path)**

---

# 810. Eulerian Path and Circuit (Hierholzer's Algorithm)

**Difficulty:** 4  **Level:** Advanced

Find a path or circuit that visits every edge exactly once — O(V + E) — with existence check via degree parity.

## The Problem This Solves

The Eulerian path problem is one of the oldest in graph theory — Euler solved the Königsberg bridge problem in 1736, founding the field. Today it models real traversal problems: postal delivery routes that must cover every street exactly once (Chinese postman problem variant), PCB inspection paths that traverse every trace, RNA fragment assembly where overlapping reads must be stitched into a single sequence, and genome sequencing where k-mer overlap graphs are assembled via Eulerian paths.

The degree-parity condition for existence is elegant: an Eulerian circuit exists iff all vertices have even degree; an Eulerian path exists iff exactly two vertices have odd degree (the start and end). This makes existence checking O(V) — you know before running the algorithm whether a solution exists.

## The Intuition

Hierholzer's algorithm: start at the beginning vertex, follow edges greedily until you return to start (or get stuck), forming a sub-circuit. If unvisited edges remain, find a node in your circuit that still has unvisited edges and splice in a new sub-circuit starting there. Iteratively merge sub-circuits. Rust's implementation uses an explicit stack: push nodes as you go forward; when stuck (no more edges from current node), pop into the output circuit. This naturally merges sub-circuits in the right order. OCaml would naturally use a recursive variant; Rust uses an explicit `stack` + `circuit` Vec pair.

## How It Works in Rust

```rust
// O(V + E) — Hierholzer's algorithm
fn eulerian_path(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = adj.len();
    let degree: Vec<usize> = adj.iter().map(|v| v.len()).collect();

    // Existence check: 0 or 2 odd-degree vertices
    let odd_verts: Vec<usize> = (0..n).filter(|&i| degree[i] % 2 != 0).collect();
    let start = match odd_verts.len() {
        0 => 0,           // circuit: start anywhere
        2 => odd_verts[0], // path: start at one odd-degree vertex
        _ => return None,  // no Eulerian path exists
    };

    let mut idx = vec![0usize; n];  // next unused edge index for each node
    let mut adj_mut: Vec<Vec<usize>> = adj.to_vec();

    let mut stack   = vec![start];
    let mut circuit = Vec::new();

    while let Some(&v) = stack.last() {
        if idx[v] < adj_mut[v].len() {
            let u = adj_mut[v][idx[v]];
            idx[v] += 1;
            // Mark edge as used in the other direction
            if let Some(pos) = adj_mut[u].iter().position(|&x| x == v) {
                adj_mut[u].swap_remove(pos);
                if pos < idx[u] && idx[u] > 0 { idx[u] -= 1; }
            }
            stack.push(u);
        } else {
            // Dead end: this node belongs in the circuit
            circuit.push(stack.pop().unwrap());
        }
    }
    circuit.reverse();
    Some(circuit)
}
```

`swap_remove` is O(1) edge deletion — it swaps the target with the last element and truncates. The `idx[u]` adjustment handles the case where `swap_remove` moved an element before the current scan position.

## What This Unlocks

- **Genome assembly**: short-read sequencers produce k-mer fragments; build a de Bruijn graph where k-mers are edges, then find the Eulerian path to reconstruct the genome sequence.
- **Postman route optimisation**: find minimum-length routes that cover every edge (street) at least once — Eulerian path gives the optimal route when all degrees are even; add minimum-weight matching edges to fix odd-degree vertices otherwise.
- **Circuit board inspection**: robot arm must probe every trace on a PCB; model traces as edges and find the Eulerian path to minimise repositioning.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Adjacency list (mutable) | `int list array` updated with `List.filter` | `Vec<Vec<usize>>` with `swap_remove` |
| Edge pointer | `ref int` per vertex | `idx: Vec<usize>` indexed array |
| Hierholzer stack | Recursive with accumulator | Explicit `Vec<usize>` stack and circuit |
| O(1) edge removal | `List.filter` is O(deg) | `swap_remove` is O(1) |
| Existence check | Count odd-degree nodes | `filter(odd).len()` then `match` |
