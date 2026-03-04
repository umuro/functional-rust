# 253: Graph BFS

**Difficulty:** 2  **Level:** Intermediate

Traverse a graph level by level, visiting all reachable nodes in order of distance from the start.

## The Problem This Solves

Breadth-first search answers "what's reachable from here, and how far away is it?" It visits every node at distance 1 before any node at distance 2. This ordering makes BFS the algorithm for shortest-path problems in unweighted graphs: social network degrees of separation, web crawl horizons, game state reachability.

The key data structure is a *queue* (first-in, first-out). You put the start node in the queue. Then loop: pull a node from the front, mark it visited, push its unvisited neighbours to the back. Because you always process the oldest entry first, nodes are visited level by level.

This example translates OCaml's BFS — which uses `Hashtbl` for visited tracking and a mutable `Queue` — to idiomatic Rust with `HashMap`, `HashSet`, and `VecDeque`. One Rust idiom stands out: `HashSet::insert` returns `false` if the element was already present, so you can test and insert in one step.

## The Intuition

Picture ripples spreading from a stone dropped in water. The first ripple reaches nodes 1 hop away. The second ripple reaches nodes 2 hops away — and so on. BFS expands these ripples one level at a time.

The queue is the pending "ripple front". Every node goes into the queue at most once (the visited set prevents re-queueing). When the queue is empty, all reachable nodes have been visited.

OCaml uses an association list `(node, [neighbours])` for the graph, looked up with `List.assoc` — O(n) per lookup. Rust uses `HashMap` for O(1) average lookup. The structural difference matters at scale.

## How It Works in Rust

```rust
use std::collections::{HashMap, HashSet, VecDeque};

pub fn bfs<'a>(graph: &'a HashMap<&str, Vec<&str>>, start: &'a str) -> Vec<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut result: Vec<&str> = Vec::new();

    queue.push_back(start);
    visited.insert(start);  // mark before entering queue (not after dequeue)

    while let Some(node) = queue.pop_front() {   // FIFO: pop from front
        result.push(node);
        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {     // insert returns false if already present
                    queue.push_back(neighbor);    // only queue truly new nodes
                }
            }
        }
    }
    result
}
```

The `visited.insert(neighbor)` idiom combines the membership check and insertion into one call — cleaner than OCaml's `Hashtbl.mem` + `Hashtbl.add` two-step.

## What This Unlocks

- **Shortest path** — extend the visited set to track distance; BFS guarantees the first time you reach a node is via the shortest path.
- **Connected components** — run BFS from each unvisited node; each run discovers one component.
- **Level-order tree traversal** — trees are graphs without cycles; BFS gives breadth-first level order naturally.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Graph representation | `(string * string list) list` | `HashMap<&str, Vec<&str>>` |
| Lookup cost | `List.assoc` — O(n) | `HashMap::get` — O(1) average |
| Queue | Mutable `Queue` module | `VecDeque` with `push_back`/`pop_front` |
| Visited test + insert | `Hashtbl.mem` then `Hashtbl.add` | `HashSet::insert` returns `bool` |
| Result accumulation | `ref` list + `List.rev` at end | `Vec::push` in traversal order |
