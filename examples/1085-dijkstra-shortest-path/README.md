# Example 1085: Dijkstra's Shortest Path

## Problem Statement
Find the shortest path between two nodes in a weighted directed graph using Dijkstra's algorithm. Return the minimum total distance, or indicate that no path exists.

## Learning Outcomes
- How to implement a priority queue with a min-heap by reversing the `Ord` on cost-node pairs
- How to construct adjacency lists functionally using `fold` over edge tuples
- How immutable-style distance tracking avoids mutable visited sets by relying on distance comparisons

## Rust Application
`Graph` is a `HashMap<usize, Vec<Edge>>` built via `build_graph` using `fold`, and `dijkstra` drives the search with a `BinaryHeap` acting as a min-heap through reversed ordering. Distance updates use cloning and rebinding rather than in-place mutation, keeping the style close to functional.

## OCaml Approach
OCaml uses a module-based priority queue (typically `Set` or a custom functor), a `Hashtbl` for mutable distance storage, and a recursive driver function with an accumulator pattern that mirrors tail-recursive iteration.

## Key Differences
1. **Priority queue:** Rust inverts `Ord` on a newtype to turn `BinaryHeap` (max-heap) into a min-heap; OCaml uses a functorized module that provides min-priority semantics directly
2. **Distance map:** Rust uses a `HashMap` rebound on each update to approximate immutability; OCaml uses `Hashtbl` with explicit mutable updates, making the mutation visible at the call site
3. **Termination:** Rust avoids a separate visited set by skipping nodes whose recorded distance is already lower than the popped cost; OCaml's recursive accumulator naturally threads this check through the call stack

## Exercises
1. Extend `dijkstra` to return the full shortest path (sequence of node IDs) in addition to the distance
2. Replace the distance `HashMap` with a purely functional approach using a recursive helper that threads state as a parameter, eliminating all interior mutation
3. Adapt the implementation to handle undirected graphs by inserting both directions in `build_graph`, and verify correctness on a graph with cycles
