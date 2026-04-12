📖 **[View on hightechmind.io →](https://hightechmind.io/rust/809-max-flow-ford-fulkerson)**

---

# 809-max-flow-ford-fulkerson — Max Flow (Ford-Fulkerson with BFS)
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Maximum flow (Ford-Fulkerson, 1956) finds the maximum amount that can flow from a source to a sink in a capacitated network. The Edmonds-Karp variant uses BFS (finding shortest augmenting paths) and runs in O(VE²). Applications include network traffic routing, image segmentation (min-cut = max-flow), bipartite matching (König's theorem), and supply chain optimization. It is one of the most practically important algorithms in operations research.

## Learning Outcomes

- Implement Edmonds-Karp: BFS to find shortest augmenting paths repeatedly
- Maintain a residual capacity matrix `cap[u][v]` that decreases with forward flow and increases backward
- Find path flow as the minimum capacity along the augmenting path
- Update residual capacities: decrease forward, increase backward (enabling un-doing)
- Apply the max-flow min-cut theorem: max flow = min cut capacity

## Rust Application

`max_flow(n, edges, source, sink)` builds a capacity matrix `cap[u][v]`. BFS finds an augmenting path from source to sink using `parent[v]`. The path flow is the minimum capacity along the path. Forward edges decrease, backward edges increase. Repeats until no augmenting path exists (BFS returns without reaching sink). Tests cover a simple 4-node network.

## OCaml Approach

OCaml implements with `Array.make_matrix n n 0` for capacities and `Array.make n None` for parent tracking. The BFS uses `Queue.t`. OCaml's `Array.set` updates capacities. The `Ocamlgraph.Flow.Goldberg` module implements push-relabel (faster in practice). Flow applications in OCaml appear in network simulation and bioinformatics pipelines.

## Key Differences

1. **Residual graph**: Both languages maintain residual capacities as a 2D matrix; the backward edge increase is the key insight enabling augmentation undoing.
2. **BFS choice**: Edmonds-Karp (BFS augmenting paths) guarantees polynomial time; DFS (Ford-Fulkerson) can cycle with irrational capacities — BFS is strictly better.
3. **Push-relabel**: The push-relabel algorithm (Goldberg-Tarjan) achieves O(V²E) and is faster in practice; available in `petgraph`'s flow module for Rust.
4. **Image segmentation**: Min-cut (dual to max-flow) is used in `s-t graph cut` for image segmentation; computer vision libraries use this heavily.

## Exercises

1. Implement min-cut identification: after computing max flow, run BFS on the residual graph from source; the min cut is the set of edges from reachable to unreachable vertices.
2. Use max-flow to solve maximum bipartite matching: create a source connected to all left vertices, all right vertices connected to sink, and run max-flow.
3. Implement the push-relabel algorithm (preflow-push) and benchmark it against Edmonds-Karp on dense graphs with 100+ nodes.
