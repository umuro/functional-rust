# 380: Weighted Graph and Dijkstra

**Difficulty:** 3  **Level:** Advanced

Graphs with edge costs, and Dijkstra's algorithm for shortest paths.

## The Problem This Solves

Real graphs have costs. Road networks have distances. Network topologies have latencies. Dependency graphs have build times. You need not just "is there a path?" but "what is the cheapest path?" — and for non-negative edge weights, Dijkstra's algorithm answers this in `O((V + E) log V)`.

This is the algorithm behind Google Maps routing, IP routing protocols (OSPF), game AI pathfinding (with a heuristic it becomes A*), and any optimization problem that can be framed as shortest path. If you've ever called `.shortest_path()` in any framework, something like Dijkstra is running underneath.

## The Intuition

Dijkstra maintains a priority queue of `(cost, node)` pairs, always expanding the cheapest-to-reach node first. Start with source node at cost 0. For each expanded node, check all its neighbors: if the path through this node is cheaper than the currently known best path to that neighbor, update and enqueue. Repeat until the queue is empty or you've found your target.

The key insight: once you pop a node from the priority queue, you've found the cheapest path to it. You never need to revisit it. This greedy property holds because all edge weights are non-negative. Negative edges break Dijkstra — use Bellman-Ford instead.

Rust's `BinaryHeap` is a max-heap; wrap costs in `std::cmp::Reverse` to get a min-heap.

## How It Works in Rust

```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

type Graph = HashMap<usize, Vec<(usize, u32)>>; // node → [(neighbor, cost)]

fn dijkstra(graph: &Graph, start: usize) -> HashMap<usize, u32> {
    let mut dist: HashMap<usize, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Reverse((0u32, start)));

    while let Some(Reverse((cost, node))) = heap.pop() {
        if cost > *dist.get(&node).unwrap_or(&u32::MAX) {
            continue; // already found a cheaper path
        }
        for &(neighbor, edge_cost) in graph.get(&node).unwrap_or(&vec![]) {
            let new_cost = cost + edge_cost;
            if new_cost < *dist.get(&neighbor).unwrap_or(&u32::MAX) {
                dist.insert(neighbor, new_cost);
                heap.push(Reverse((new_cost, neighbor)));
            }
        }
    }
    dist
}
```

For production use: `petgraph::algo::dijkstra` with full graph API support.

## What This Unlocks

- **Routing algorithms** — GPS navigation, network packet routing, flight path optimization.
- **Game AI** — A* is Dijkstra with a heuristic; pathfinding on grids, navmeshes, and graphs.
- **Build optimization** — schedule tasks on the critical path of a weighted task DAG.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Priority queue | `Set` (balanced BST, acts as heap) | `BinaryHeap<T>` (max-heap; use `Reverse` for min) |
| Weighted graph | Manual adjacency with weights | `HashMap<usize, Vec<(usize, u32)>>` or `petgraph` |
| Dijkstra | `ocamlgraph` or manual | `petgraph::algo::dijkstra` or manual |
| Negative weights | Bellman-Ford via `ocamlgraph` | `petgraph::algo::bellman_ford` |
