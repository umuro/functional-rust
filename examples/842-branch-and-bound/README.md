# 842: Branch and Bound — TSP Optimisation

**Difficulty:** 4  **Level:** Advanced

Explore a solution tree while pruning branches that cannot beat the current best — turning exponential search into something tractable in practice.

## The Problem This Solves

Branch and bound is a general exact optimisation framework: explore all possible solutions (branching) but discard entire subtrees when a lower bound on their cost proves they can't improve on the best solution found so far (bounding). It's the standard approach for NP-hard combinatorial optimisation when you need an exact answer.

This example applies branch and bound to the Travelling Salesman Problem (TSP): find the shortest tour visiting all n cities exactly once. TSP is NP-hard — no polynomial algorithm is known — but branch and bound with a good lower bound (here: a simple reduced cost matrix bound) can solve instances of 15-20 cities quickly and 30-40 cities in seconds.

Branch and bound underpins industrial solvers: CPLEX and Gurobi use LP-relaxation bounds to solve integer programming problems with millions of variables. Understanding the framework is essential for implementing custom solvers when general-purpose tools aren't available or are too slow.

## The Intuition

Build a search tree where each level adds one city to the partial tour. At each node:
- **Branch**: try each unvisited city as the next stop — creating child nodes.
- **Bound**: compute a lower bound on the cost of completing the tour from this partial path. If `current_cost + lower_bound >= best_known`, prune — this subtree can't improve the answer.

The quality of the bound determines efficiency: a tight bound prunes more branches. Here, the bound adds the minimum outgoing edge from each unvisited city — simple to compute but often effective.

Best case: O(n²) with perfect pruning. Worst case: O(n!) with a terrible bound (same as brute force). In practice, a good bound + DFS with best-first ordering (try cheapest paths first to get a good initial upper bound quickly) makes many instances tractable.

## How It Works in Rust

```rust
struct TSP {
    n: usize,
    dist: Vec<Vec<f64>>, // distance matrix
}

impl TSP {
    fn solve(&self) -> (f64, Vec<usize>) {
        let mut best_cost = f64::INFINITY;
        let mut best_path = vec![];
        let mut path = vec![0usize]; // start at city 0
        let mut visited = vec![false; self.n];
        visited[0] = true;

        self.branch(
            &mut path, &mut visited,
            0.0, &mut best_cost, &mut best_path
        );
        (best_cost, best_path)
    }

    fn branch(
        &self,
        path: &mut Vec<usize>,
        visited: &mut Vec<bool>,
        cost: f64,
        best_cost: &mut f64,
        best_path: &mut Vec<usize>,
    ) {
        // Complete tour: close the loop back to start
        if path.len() == self.n {
            let last = *path.last().unwrap();
            let total = cost + self.dist[last][0];
            if total < *best_cost {
                *best_cost = total;
                *best_path = path.clone();
            }
            return;
        }

        // Lower bound: cost so far + minimum edge from each unvisited city
        let bound = cost + self.lower_bound(visited, *path.last().unwrap());
        if bound >= *best_cost { return; } // prune

        // Branch: try each unvisited city as next stop
        // Sort candidates by distance to get a good bound quickly (greedy heuristic order)
        let current = *path.last().unwrap();
        let mut candidates: Vec<usize> = (0..self.n)
            .filter(|&c| !visited[c])
            .collect();
        candidates.sort_by(|&a, &b| {
            self.dist[current][a].partial_cmp(&self.dist[current][b]).unwrap()
        });

        for next in candidates {
            visited[next] = true;
            path.push(next);
            self.branch(
                path, visited,
                cost + self.dist[current][next],
                best_cost, best_path
            );
            path.pop();
            visited[next] = false;
        }
    }

    fn lower_bound(&self, visited: &[bool], current: usize) -> f64 {
        // Sum of minimum outgoing edge from current + each unvisited city
        let mut bound = 0.0;
        // Min edge from current city to any unvisited city
        let min_from_current = (0..self.n)
            .filter(|&c| !visited[c])
            .map(|c| self.dist[current][c])
            .fold(f64::INFINITY, f64::min);
        bound += min_from_current;

        // Min outgoing edge from each unvisited city
        for city in 0..self.n {
            if !visited[city] {
                let min_edge = (0..self.n)
                    .filter(|&c| c != city)
                    .map(|c| self.dist[city][c])
                    .fold(f64::INFINITY, f64::min);
                bound += min_edge;
            }
        }
        bound / 2.0 // each edge counted twice (once from each endpoint)
    }
}
```

Sorting candidates by distance (`greedy heuristic order`) is key: by trying cheap options first, you find a good tour early and update `best_cost` to a tight value — making subsequent pruning more aggressive. This is the "best-first" strategy within DFS.

The bound divided by 2 comes from counting each edge from both endpoints — a standard trick in TSP lower bounds to avoid double-counting.

## What This Unlocks

- **Exact TSP for small instances**: branch and bound solves 20-city instances in milliseconds, 30-city in seconds — practical for delivery route optimisation at small scale.
- **Integer programming**: B&B is the engine inside CPLEX/Gurobi — they branch on fractional LP variables and use LP relaxations as bounds.
- **Scheduling and assignment**: job-shop scheduling, nurse rostering, and exam timetabling are all solved by B&B with domain-specific bounds.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable path/visited | Functional threading or `ref` cells | `&mut Vec<usize>` and `&mut Vec<bool>` — explicit mutable borrows |
| Backtracking | Immutable state (no undo needed) | Explicit `push`/`pop`, `visited[x] = true/false` |
| Best cost tracking | `ref float` threaded through | `&mut f64` — shared mutable reference |
| Greedy candidate sort | `List.sort` | `sort_by` with `partial_cmp` for f64 |
| Lower bound | Same algorithm | `.fold(f64::INFINITY, f64::min)` — idiomatic min over iterator |
