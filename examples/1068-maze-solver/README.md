📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1068-maze-solver)**

---

# 1068-maze-solver — Maze Solver

## Problem Statement

Path finding in a 2D grid is a fundamental robotics and game AI problem: find a path from start to end while avoiding walls. DFS backtracking finds any path; BFS finds the shortest path (by number of steps). Both are foundational to robot navigation, video game pathfinding, and circuit board routing.

This example implements both DFS (finds a path, not necessarily shortest) and BFS (finds the shortest path), demonstrating the fundamental algorithmic trade-off between the two search strategies.

## Learning Outcomes

- Implement DFS backtracking for 2D grid path finding
- Implement BFS for shortest-path grid traversal
- Use a visited matrix to prevent revisiting cells
- Reconstruct the path by tracking parent pointers in BFS
- Understand when DFS vs BFS is appropriate

## Rust Application

`src/lib.rs` uses a 2D grid where 0 = open and 1 = wall. `solve_maze` uses DFS: mark the current cell as visited, try all four directions, return true on reaching the goal, and unmark on backtrack. `bfs_maze` uses a `VecDeque` queue and a parent map (`HashMap<(usize, usize), (usize, usize)>`) to reconstruct the shortest path.

The four-direction array `DIRS: [(i32, i32); 4]` is the standard encoding for grid movement. BFS guarantees the shortest path for unweighted grids; for weighted grids, use Dijkstra's algorithm.

## OCaml Approach

```ocaml
let solve_maze maze start end_ =
  let rows = Array.length maze in
  let cols = Array.length maze.(0) in
  let visited = Array.make_matrix rows cols false in
  let path = ref [] in
  let dirs = [|(0,1); (1,0); (0,-1); (-1,0)|] in
  let rec dfs r c =
    if (r, c) = end_ then (path := (r,c) :: !path; true)
    else if maze.(r).(c) = 1 || visited.(r).(c) then false
    else begin
      visited.(r).(c) <- true;
      path := (r,c) :: !path;
      if Array.exists (fun (dr, dc) ->
        let nr, nc = r + dr, c + dc in
        nr >= 0 && nr < rows && nc >= 0 && nc < cols && dfs nr nc
      ) dirs then true
      else begin path := List.tl !path; false end
    end
  in
  if dfs (fst start) (snd start) then Some (List.rev !path) else None
```

The DFS structure is identical. OCaml's mutable `path` ref mirrors Rust's mutable `path` Vec.

## Key Differences

1. **Bounds checking**: Rust uses `checked_add` on `usize` or signed arithmetic with range checks; OCaml checks bounds before casting.
2. **Path reconstruction**: Rust's DFS pushes/pops to a `Vec`; OCaml's uses a `ref` to a list with `List.tl` for backtracking.
3. **BFS parent map**: Rust uses `HashMap<(usize, usize), (usize, usize)>` for parent tracking; OCaml would use `Hashtbl`.
4. **Direction encoding**: Both use an array of `(row_delta, col_delta)` tuples — this is a universal pattern for 2D grid traversal.

## Exercises

1. Add diagonal movement (8 directions) and verify BFS still finds shortest paths.
2. Implement A* search that uses Manhattan distance as the heuristic, finding shortest paths faster than BFS on open grids.
3. Write a maze generator using recursive backtracking (carve passages) and verify the solver can solve all generated mazes.
