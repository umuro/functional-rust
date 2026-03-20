📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1069-graph-coloring)**

---

# 1069-graph-coloring — Graph Coloring

## Problem Statement

Graph coloring assigns colors to vertices such that no two adjacent vertices share a color, using at most k colors. It models register allocation in compilers (colors = registers, vertices = variables, edges = interference), exam scheduling (colors = time slots, vertices = exams, edges = shared students), and map coloring.

Graph coloring is NP-complete in general but tractable for small k or special graph families. The backtracking approach tries each color and backtracks when a conflict is found.

## Learning Outcomes

- Implement graph coloring via backtracking with constraint checking
- Understand the `is_safe` predicate that checks all neighbors
- Find the minimum chromatic number by trying k = 1, 2, 3, ...
- Connect to compiler register allocation (chordal graph coloring)
- Understand the Four Color Theorem for planar graphs

## Rust Application

`src/lib.rs` uses an adjacency matrix (`adj: &[Vec<i32>]`) representation. `is_safe(node, color, adj, colors)` checks that no neighbor already has the given color. `solve` recursively colors vertices 0 to n-1, trying each color and backtracking when no valid color exists. Returns `Some(colors)` on success or `None` if coloring is impossible.

The adjacency matrix enables O(1) edge checking at the cost of O(V²) space. For sparse graphs, the adjacency list representation would be more efficient.

## OCaml Approach

```ocaml
let graph_color adj k =
  let n = Array.length adj in
  let colors = Array.make n 0 in
  let is_safe node color =
    Array.for_all2 (fun neighbor c -> adj.(node).(neighbor) = 0 || c <> color) adj.(node) colors
  in
  let rec solve node =
    if node = n then true
    else
      let rec try_colors c =
        if c > k then false
        else if is_safe node c then begin
          colors.(node) <- c;
          if solve (node + 1) then true
          else begin colors.(node) <- 0; try_colors (c + 1) end
        end else try_colors (c + 1)
      in
      try_colors 1
  in
  if solve 0 then Some (Array.to_list colors) else None
```

Structurally identical. Both implement the same backtracking search.

## Key Differences

1. **`is_safe` complexity**: Rust's `is_safe` iterates all vertices and checks `adj[node][i] == 1`; OCaml's `Array.for_all2` is equivalent.
2. **Try loop**: Rust uses a `for c in 1..=num_colors` loop; OCaml's tail-recursive `try_colors` is equivalent.
3. **Return type**: Both return `Option<Vec<usize>>` / `option (int list)` — `None` when coloring is impossible.
4. **Bipartite check**: 2-colorability can be checked in O(V+E) with BFS; for k > 2, no polynomial algorithm is known (NP-complete).

## Exercises

1. Implement `chromatic_number(adj: &[Vec<i32>]) -> usize` that finds the minimum number of colors needed by trying k = 1, 2, ... until a valid coloring exists.
2. Extend to weighted graph coloring where each vertex has a processing time and colors represent time slots — find a valid schedule minimizing the last end time.
3. Write a `is_bipartite(adj: &[Vec<i32>]) -> bool` function using BFS 2-coloring as a special case.
