📖 **[View on hightechmind.io →](https://hightechmind.io/rust/803-a-star-pathfinding)**

---

# 803. A* Pathfinding with Manhattan Heuristic

**Difficulty:** 4  **Level:** Advanced

Guided shortest-path search on a 2D grid using a heuristic to focus exploration toward the goal — O(E log V) in the worst case, much faster in practice.

## The Problem This Solves

A* is the pathfinding algorithm used in virtually every game engine, robotics motion planner, and GPS navigation system. Dijkstra's algorithm expands outward uniformly in all directions; A* focuses the search toward the goal by adding a heuristic estimate of remaining cost. On grid maps with many nodes, this can reduce explored nodes by orders of magnitude compared to Dijkstra.

The choice of heuristic determines both correctness and performance: a *consistent* (monotone) heuristic guarantees optimal paths. For grid movement allowing only cardinal directions, the Manhattan distance `|dx| + |dy|` is the exact minimum cost to reach the goal, making it an ideal admissible heuristic.

## The Intuition

A* maintains a priority queue ordered by `f = g + h`, where `g` is the actual cost from start to the current node, and `h` is the heuristic estimate to the goal. Nodes with low `f` are explored first — the algorithm "aims" at the goal. Dijkstra is A* with `h = 0`. A* with Manhattan heuristic on a grid is Dijkstra with perfect cost guidance. Rust implements the min-heap with `BinaryHeap<Reverse<(f, g, row, col)>>`, using `Reverse` because `BinaryHeap` is a max-heap. OCaml would use a priority queue module or sorted list; the structure is otherwise identical.

## How It Works in Rust

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// O(E log V) worst case; in practice much faster with a good heuristic
// grid: 0=passable, b'#'=wall
fn a_star(
    grid: &[Vec<u8>],
    start: (usize, usize),
    goal:  (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut g_cost = vec![vec![u64::MAX / 2; cols]; rows];
    let mut came: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; cols]; rows];
    let mut heap = BinaryHeap::new();

    // Heuristic: Manhattan distance (admissible for cardinal movement)
    let h = |r: usize, c: usize| -> u64 {
        ((r as i64 - goal.0 as i64).abs() + (c as i64 - goal.1 as i64).abs()) as u64
    };

    g_cost[start.0][start.1] = 0;
    heap.push(Reverse((h(start.0, start.1), 0u64, start.0, start.1)));
    //                  ^f                   ^g    ^row      ^col

    while let Some(Reverse((_, g, r, c))) = heap.pop() {
        if g > g_cost[r][c] { continue; }   // stale entry
        if (r, c) == goal { /* reconstruct and return */ }

        for (dr, dc) in [(-1i64,0),(1,0),(0,-1),(0,1)] {
            // bounds check, wall check, then:
            let ng = g + 1;
            if ng < g_cost[nr][nc] {
                g_cost[nr][nc] = ng;
                came[nr][nc]   = Some((r, c));
                heap.push(Reverse((ng + h(nr, nc), ng, nr, nc)));
            }
        }
    }
    None
}
```

Path reconstruction walks `came[][]` backwards from the goal to the start, then reverses. The stale-entry guard `if g > g_cost[r][c] { continue }` is the same lazy-deletion pattern as Prim's/Dijkstra.

## What This Unlocks

- **Game AI pathfinding**: every RTS and RPG game uses A* (or a variant like Theta* or Jump Point Search) for unit navigation on tile maps.
- **Robot motion planning**: mobile robots compute collision-free paths through occupancy grid maps using A* with Euclidean or Manhattan heuristics.
- **GPS routing**: road navigation systems use A* (or Bidirectional A*) with geographic distance as the heuristic, processing millions of road network nodes in milliseconds.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Min-heap | Priority queue module | `BinaryHeap<Reverse<(u64, u64, usize, usize)>>` |
| Heuristic | Inline function or `let h = ...` | Closure `let h = \|r, c\| -> u64 { ... }` |
| Stale entry | `Hashtbl` of visited + check | `if g > g_cost[r][c] { continue }` |
| Path matrix | `(int * int) option array array` | `Vec<Vec<Option<(usize, usize)>>>` |
| Bounds check | `if r >= 0 && r < rows ...` | Cast `i64` coordinates, then `as usize` after check |
