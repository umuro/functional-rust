# 254: Graph DFS

**Difficulty:** 2  **Level:** Intermediate

Traverse a graph by diving deep along each path before backtracking — using an explicit stack or recursion.

## The Problem This Solves

Depth-first search explores as far as possible along one path before trying alternatives. It's the algorithm for topological sorting, cycle detection, maze solving, reachability analysis, and tree traversal. When you don't need shortest-path ordering and do need to explore complete paths, DFS is the right choice.

BFS uses a queue to expand level by level. DFS uses a *stack* (last-in, first-out) to dive deep. The structural difference is one word: replace `push_back`/`pop_front` with `push`/`pop` and you have DFS. Understanding this symmetry reveals the deep relationship between the two algorithms.

OCaml's natural DFS is recursive — thread a visited set through returns, fold over neighbours. Rust can do the same recursively, but for large graphs risks stack overflow. The idiomatic Rust solution uses an explicit stack on the heap, giving the same traversal order without recursion limits.

## The Intuition

Imagine exploring a cave network. DFS says: pick any passage, follow it all the way to a dead end (or a room you've been to before), then backtrack to the last junction and try the next passage. You fully explore one branch before touching another.

The explicit stack mirrors the call stack of the recursive version. Each "push a node" is like "make a recursive call". Each "pop a node" is like "return from a recursive call". The heap stack has no depth limit; the call stack does.

OCaml's version threads the visited set as an immutable value through returns — `go` returns `(new_visited, path)`. Rust's version passes `&mut HashSet`, which is more efficient but keeps the same invariant: a node is visited at most once.

## How It Works in Rust

```rust
use std::collections::{HashMap, HashSet};

// Iterative DFS — no recursion limit, O(1) stack ops
pub fn dfs<'a>(graph: &'a HashMap<&str, Vec<&str>>, start: &'a str) -> Vec<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut stack: Vec<&str> = vec![start];
    let mut result: Vec<&str> = Vec::new();

    while let Some(node) = stack.pop() {    // LIFO: pop from back
        if !visited.insert(node) { continue; } // skip if already visited
        result.push(node);
        if let Some(neighbors) = graph.get(node) {
            // Push in reverse order so first neighbour is processed first
            for &neighbor in neighbors.iter().rev() {
                if !visited.contains(neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }
    result
}

// Recursive DFS — mirrors OCaml's `go` helper, uses &mut visited
pub fn dfs_recursive<'a>(graph: &[(&'a str, Vec<&'a str>)], start: &'a str) -> Vec<&'a str> {
    fn go<'a>(graph: &[(&'a str, Vec<&'a str>)], visited: &mut HashSet<&'a str>, node: &'a str, result: &mut Vec<&'a str>) {
        if !visited.insert(node) { return; }
        result.push(node);
        if let Some((_, neighbors)) = graph.iter().find(|(k, _)| *k == node) {
            for &n in neighbors { go(graph, visited, n, result); }
        }
    }
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    go(graph, &mut visited, start, &mut result);
    result
}
```

## What This Unlocks

- **Topological sort** — DFS post-order gives reverse topological order; essential for build systems and dependency graphs.
- **Cycle detection** — track "currently in stack" vs "fully visited"; a back edge to "in stack" means a cycle.
- **Connected components** — run DFS from each unvisited node; same pattern as BFS but explores one component fully before starting the next.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Visited set | Immutable `Set.Make(String)` threaded through returns | Mutable `HashSet<&str>` passed by `&mut` |
| Recursion vs iteration | Natural recursive style | Iterative stack preferred (no overflow risk) |
| Graph representation | Association list, `List.assoc` O(n) | `HashMap` O(1) average |
| Insert + check | `SS.mem` then `SS.add` (two operations) | `HashSet::insert` returns `bool` (one call) |
| Neighbour order | Left-to-right `List.fold_left` | Push reversed, pop in forward order |
