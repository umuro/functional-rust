📖 **[View on hightechmind.io →](https://hightechmind.io/rust/810-eulerian-path)**

---

# 810-eulerian-path — Eulerian Path
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

An Eulerian path visits every edge exactly once. Euler proved in 1736 (the Königsberg bridge problem — the first graph theory result) that an Eulerian path exists if and only if exactly 0 or 2 vertices have odd degree (directed: out-degree − in-degree = ±1 for endpoints, 0 for all others). Eulerian circuits (closed paths) require all vertices to have equal in/out-degree. Applications include printed circuit board routing, DNA sequence assembly (de Bruijn graphs), and postman route optimization.

## Learning Outcomes

- Check the Eulerian path existence condition: exactly 0 or 2 vertices with odd degree in undirected, specific in/out conditions in directed
- Find the start vertex: the vertex with out-degree > in-degree (or any vertex for Eulerian circuit)
- Implement Hierholzer's algorithm: DFS with backtracking to build the path
- Understand why Hierholzer's works: extend the path until stuck, then splice in detours
- Apply to DNA assembly: reads as edges in a de Bruijn graph → Eulerian path = assembled sequence

## Rust Application

`eulerian_path(n, edges)` checks degree conditions using `deg[v][0]` (out) and `deg[v][1]` (in). Finds start vertex (out > in → exactly one such vertex for Eulerian path) or defaults to 0 for circuit. Hierholzer's: maintain a stack; while the current vertex has unused edges, push to stack and follow the edge; when stuck, add to path. Returns `None` if conditions fail.

## OCaml Approach

OCaml implements with `Array.make n []` for adjacency and `Array.make n 0` for degree tracking. Hierholzer's stack uses a `list ref`. OCaml's `List.tl` advances the adjacency list. The de Bruijn graph construction for DNA assembly is a natural OCaml application given its string processing strengths. `Ocamlgraph.Euler` provides a clean implementation.

## Key Differences

1. **Edge consumption**: Rust removes edges from `adj[v]` using `pop`; OCaml uses list tails — both avoid revisiting edges.
2. **Degree check**: Directed Eulerian path requires exactly one vertex with out-degree = in-degree + 1 (start) and one with in-degree = out-degree + 1 (end); both languages check this identically.
3. **DNA assembly**: de Bruijn graphs for genome assembly use k-mers as vertices and (k-1)-mer overlaps as edges; the assembled genome is the Eulerian path.
4. **Hierholzer vs Fleury**: Fleury's algorithm (bridge-avoiding) is O(E²); Hierholzer's is O(E) — always use Hierholzer's.

## Exercises

1. Implement `eulerian_circuit_undirected(n, edges)` for undirected graphs: check that all vertices have even degree and find the circuit.
2. Construct the de Bruijn graph for a set of k-mer reads and find the Eulerian path to assemble the DNA sequence.
3. Apply Eulerian path to the "Chinese Postman Problem": find the minimum weight closed walk that covers all edges, adding minimum weight duplicate edges to fix odd-degree vertices.
