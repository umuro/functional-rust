📖 **[View on hightechmind.io →](https://hightechmind.io/rust/803-a-star-pathfinding)**

---

# 803-a-star-pathfinding — A* Pathfinding
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A* (Hart, Nilsson, Raphael, 1968) is the standard pathfinding algorithm in games, robotics, and navigation systems. It improves on Dijkstra by adding a heuristic that guides the search toward the goal, dramatically reducing explored nodes. The Manhattan distance heuristic is admissible (never overestimates) on grids, guaranteeing optimal paths. Used in every major game engine (Unity NavMesh, Unreal AI), Google Maps, and robot motion planning.

## Learning Outcomes

- Implement A* using an open set (`BinaryHeap<Reverse<(f_score, position)>>`) with `came_from` map
- Use the Manhattan distance `|dx| + |dy|` as an admissible heuristic for grid movement
- Track `g_score` (cost from start) and `f_score = g_score + h(goal)` per node
- Reconstruct the path by walking the `came_from` map backward from goal to start
- Understand why admissible heuristics guarantee optimal paths

## Rust Application

`astar(start, goal, obstacles)` uses `HashSet` for obstacle lookup, a `BinaryHeap<Reverse<(usize, (i32,i32))>>` for the open set, `HashMap<pos, g_score>`, and `came_from: HashMap<pos, pos>`. Expands 4-directional neighbors, skips obstacles, and updates costs. On reaching `goal`, reconstructs the path by walking `came_from`. Tests verify path existence and obstacle avoidance.

## OCaml Approach

OCaml implements A* using `Map.t` for `g_score` and `came_from`, and `Set.t` as the open set with a custom ordering by `f_score`. OCaml's `Hashtbl` provides faster mutable alternatives. The `OcamlAI` library and various game frameworks use OCaml A* implementations. The path reconstruction uses `List.rev` on the accumulated path.

## Key Differences

1. **Priority queue**: Rust's `BinaryHeap<Reverse<...>>` is efficient; OCaml's balanced BST-based `Set` has higher constant factors but equivalent asymptotic complexity.
2. **Heuristic pluggability**: Rust's closure `let heuristic = |p| ...` makes it easy to swap heuristics; OCaml uses first-class functions similarly.
3. **Tuple comparison**: Rust's `Reverse((f_score, pos))` sorts by f_score first due to tuple lexicographic order; OCaml's custom `Set.compare` must be explicit.
4. **Game use**: Unity's NavMesh, Unreal's PathFollowingComponent, and Godot's NavigationAgent all use A* variants internally; Rust game engines use the same algorithm.

## Exercises

1. Add diagonal movement (8 directions) with a cost of `√2` for diagonals, and switch the heuristic to Euclidean distance.
2. Implement weighted A* (multiply heuristic by a weight `w > 1`) to trade optimality for speed. Compare path quality and nodes expanded for `w = 1, 1.5, 2.0`.
3. Add a `max_steps` limit to A* and return the partial path if the goal is not reached within the step budget — useful for real-time game AI.
