📖 **[View on hightechmind.io →](https://hightechmind.io/rust/804-tarjan-scc)**

---

# 804. Tarjan's Strongly Connected Components

**Difficulty:** 5  **Level:** Master

Find all strongly connected components in a directed graph in a single DFS pass — O(V + E) — using discovery times and low-link values.

## The Problem This Solves

A strongly connected component (SCC) is a maximal set of nodes where every node is reachable from every other. SCCs reveal the "true structure" of directed graphs. Compilers use SCC decomposition to detect circular dependencies between modules and to order compilation units. Social network analysis uses SCCs to find tightly-knit communities where influence flows bidirectionally. Web crawlers use SCCs to detect link farms and cyclic structures. Deadlock detection in concurrent systems models resource dependencies as a directed graph — a cycle indicates deadlock, and SCC is a generalisation.

## The Intuition

During DFS, assign each node a discovery time (`disc`) when first visited, and a `low` value representing the smallest discovery time reachable from its subtree (including back edges). When we finish processing a node and `low[u] == disc[u]`, that node is the "root" of an SCC — pop everything off the auxiliary stack down to (and including) u; those nodes form one SCC. The key insight: if `low[u] == disc[u]`, no back edge from u's subtree reaches above u in the DFS tree, so u and everything below it forms an isolated component.

Recursive DFS is natural in OCaml but risks stack overflow in Rust on large graphs. This implementation uses an explicit call stack — `Vec<(node, adj_index)>` — simulating the call stack manually.

## How It Works in Rust

```rust
// O(V + E) — single DFS pass with explicit stack (no recursion overflow)
fn tarjan_scc(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut disc     = vec![usize::MAX; n]; // MAX = unvisited
    let mut low      = vec![0usize; n];
    let mut on_stack = vec![false; n];
    let mut stack    = Vec::new();          // Tarjan's auxiliary stack
    let mut timer    = 0usize;
    let mut sccs     = Vec::new();

    for start in 0..n {
        if disc[start] != usize::MAX { continue; }
        // Iterative DFS: each frame = (node, index into adj[node])
        let mut call_stack: Vec<(usize, usize)> = vec![(start, 0)];
        disc[start] = timer; low[start] = timer; timer += 1;
        stack.push(start); on_stack[start] = true;

        while let Some((u, idx)) = call_stack.last_mut() {
            let u = *u;
            if *idx < adj[u].len() {
                let v = adj[u][*idx]; *idx += 1;
                if disc[v] == usize::MAX {
                    // Tree edge: recurse into v
                    disc[v] = timer; low[v] = timer; timer += 1;
                    stack.push(v); on_stack[v] = true;
                    call_stack.push((v, 0));
                } else if on_stack[v] {
                    // Back edge: update low[u] with disc[v]
                    low[u] = low[u].min(disc[v]);
                }
            } else {
                // Done with u: propagate low to parent, check SCC root
                call_stack.pop();
                if let Some(&(parent, _)) = call_stack.last() {
                    low[parent] = low[parent].min(low[u]);
                }
                if low[u] == disc[u] {
                    // u is SCC root: pop the stack
                    let mut scc = Vec::new();
                    loop {
                        let w = stack.pop().unwrap();
                        on_stack[w] = false;
                        scc.push(w);
                        if w == u { break; }
                    }
                    sccs.push(scc);
                }
            }
        }
    }
    sccs
}
```

The `on_stack` array distinguishes back edges (to ancestors on the stack) from cross edges (to already-processed nodes). Only back edges update `low`.

## What This Unlocks

- **Circular dependency detection**: compilers and package managers decompose the module dependency graph into SCCs; cycles within an SCC must be resolved before processing nodes that depend on them.
- **Condensation graph**: replace each SCC with a single super-node to get a DAG, then run topological sort for evaluation order. This is the standard compiler IR analysis step.
- **2-SAT solving**: Boolean satisfiability with 2-literal clauses reduces to SCC on an implication graph — if a variable and its negation are in the same SCC, the formula is unsatisfiable.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| DFS implementation | Recursive (natural, but stack-limited) | Explicit `Vec<(node, adj_idx)>` call stack |
| Discovery time | Mutable `ref` counter | `usize` `timer` variable, incremented per visit |
| Auxiliary stack | `Stack.t` or list | `Vec<usize>` with `push`/`pop` |
| Low propagation | During recursive return | After `call_stack.pop()`, propagate to `call_stack.last()` |
| SCC root detection | `low.(u) = disc.(u)` | `if low[u] == disc[u]` — identical logic |
