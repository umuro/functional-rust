# Example 1110: Dijkstra's Shortest Path — Priority Queue
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Find the shortest-path distance from a start node to every reachable node in a
weighted directed graph, using Dijkstra's algorithm with a priority queue.

## Learning Outcomes

- How `BinaryHeap<Reverse<T>>` translates OCaml's `Set.Make` ordered-BST priority queue
- Why duplicate entries arise in heap-based Dijkstra and how to filter them idiomatically
- Using `BTreeSet::pop_first()` as a direct analog of OCaml's `PQ.min_elt` + `PQ.remove`
- How OCaml's `try ... with Not_found -> default` maps to Rust's `.unwrap_or(default)`

## OCaml Approach

OCaml defines a custom-compared `Set.Make` module as a priority queue: because the set
is ordered by `(distance, node)`, `PQ.min_elt` always returns the cheapest unvisited
node in O(log n). The distance map is a `Map.Make(String)`. The algorithm is expressed
as a tail-recursive `let rec go pq dist` accumulator — idiomatic functional style.

## Rust Approach

The idiomatic Rust version uses `BinaryHeap<Reverse<(i32, String)>>` — a max-heap
flipped to min-heap semantics via `Reverse`. Because `BinaryHeap` allows duplicate
entries (unlike OCaml's `Set`), stale entries are filtered with a one-line check.
The functional variant uses `BTreeSet<(i32, String)>` and `BTreeMap`, mirroring
OCaml's sorted-set PQ and functional map, with an inner recursive `go` function.

## Key Differences

1. **Priority Queue:** OCaml `Set.Make` is a balanced BST — no duplicate `(d, node)` pairs allowed; set semantics enforce uniqueness. Rust `BinaryHeap` is a binary heap — duplicates are allowed, filtered lazily by checking `d > dist[u]`.
2. **Min extraction:** OCaml `PQ.min_elt` + `PQ.remove` → two calls. Rust `BTreeSet::pop_first()` atomically removes and returns the minimum in one call.
3. **Not-found handling:** OCaml `try SMap.find v dist with Not_found -> max_int` uses exception-based control flow. Rust `.unwrap_or(i32::MAX)` expresses the same default with pure `Option` combinators.
4. **Tail recursion:** OCaml's `let rec go` is tail-recursive and the compiler emits a loop. Rust's inner `fn go` has no guaranteed TCO — for large graphs the iterative `dijkstra` avoids stack overflow.

## Exercises

1. Add a `path_to` function that reconstructs the sequence of node IDs forming the shortest path from source to target, returning `None` if the target is unreachable.
2. Extend the graph to support directed vs. undirected edges and verify that shortest paths differ appropriately for each graph type.
3. Implement A* search on the same graph representation by adding a heuristic function parameter, and compare the number of nodes expanded vs. plain Dijkstra on a grid graph.
