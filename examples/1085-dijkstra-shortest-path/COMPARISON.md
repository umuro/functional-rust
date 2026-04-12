# OCaml vs Rust: Dijkstra's Shortest Path

## Side-by-Side Code

### OCaml

```ocaml
module IntMap = Map.Make(Int)

let build_graph edges =
  List.fold_left (fun g (src, dst, w) ->
    let existing = try IntMap.find src g with Not_found -> [] in
    IntMap.add src ({ target = dst; weight = w } :: existing) g
  ) IntMap.empty edges

let dijkstra graph source =
  let rec loop heap dist =
    match heap with
    | [] -> dist
    | (cost, node) :: rest ->
      let current = try IntMap.find node dist with Not_found -> max_int in
      if cost > current then loop rest dist
      else
        let neighbors = try IntMap.find node graph with Not_found -> [] in
        let dist', heap' = List.fold_left (fun (d, h) edge ->
          let new_dist = cost + edge.weight in
          let old = try IntMap.find edge.target d with Not_found -> max_int in
          if new_dist < old then
            (IntMap.add edge.target new_dist d, (new_dist, edge.target) :: h)
          else (d, h)
        ) (dist, rest) neighbors in
        loop (List.sort compare heap') dist'
  in
  loop [(0, source)] (IntMap.singleton source 0)
```

### Rust (idiomatic)

```rust
pub fn dijkstra(graph: &Graph, source: usize) -> HashMap<usize, u64> {
    let mut dist: HashMap<usize, u64> = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(source, 0);
    heap.push(State { cost: 0, node: source });

    while let Some(State { cost, node }) = heap.pop() {
        if cost > *dist.get(&node).unwrap_or(&u64::MAX) { continue; }
        if let Some(neighbors) = graph.get(&node) {
            for edge in neighbors {
                let next_cost = cost + edge.weight;
                let old = *dist.get(&edge.to).unwrap_or(&u64::MAX);
                if next_cost < old {
                    dist.insert(edge.to, next_cost);
                    heap.push(State { cost: next_cost, node: edge.to });
                }
            }
        }
    }
    dist
}
```

### Rust (functional/recursive)

```rust
// Build graph using fold — mirrors the OCaml build_graph directly
pub fn build_graph(edges: &[(usize, usize, u64)]) -> Graph {
    edges.iter().fold(HashMap::new(), |mut g, &(src, dst, w)| {
        g.entry(src).or_default().push(Edge { to: dst, weight: w });
        g
    })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| graph | `edge list IntMap.t` | `HashMap<usize, Vec<Edge>>` |
| distance map | `int IntMap.t` | `HashMap<usize, u64>` |
| priority queue | `(int * int) list` re-sorted each step | `BinaryHeap<State>` (min-heap via custom `Ord`) |
| build graph | `List.fold_left` over edge list | `Iterator::fold` over edge slice |
| update check | `cost > current` skips stale entries | `cost > dist[node]` skips stale entries |

## Key Insights

1. **Priority queue efficiency:** OCaml re-sorts a list after each expansion — O(n log n) per step. Rust's `BinaryHeap` gives O(log n) per push/pop, for O((V + E) log V) total. Both produce the same distances; Rust is asymptotically better on dense graphs.
2. **Min-heap from max-heap:** Rust's `BinaryHeap` is a max-heap. The custom `Ord` on `State` reverses the comparison — `other.cost.cmp(&self.cost)` — so the state with the smallest cost is popped first. This is the standard Rust idiom; alternatively, `Reverse((cost, node))` can wrap tuples.
3. **Stale entries instead of a visited set:** Both versions skip heap entries whose recorded cost is already lower than the current distance — `if cost > current then loop rest dist` in OCaml, `if cost > dist[node] { continue; }` in Rust. This lazy deletion avoids a separate `visited` set and is safe because distances only decrease.
4. **Ownership prevents graph aliasing:** In a mutable imperative implementation, a bug could accidentally alias the graph and distance map. Rust's ownership model makes this a compile-time error — `graph` is borrowed immutably while `dist` and `heap` are mutated, and the borrow checker ensures they do not alias.
5. **Functional graph construction:** `build_graph` uses `fold` over a slice of edge tuples in both languages, building the adjacency list purely from the input. The OCaml version uses `IntMap`; Rust uses `HashMap`. The fold structure is identical.

## When to Use Each Style

**Use idiomatic Rust (while-let loop with BinaryHeap) when:** Running Dijkstra on large graphs — the heap approach is O((V+E) log V) and the while-let loop is the clearest expression in Rust.
**Use functional recursive Rust when:** Building the graph from edges via `fold` — the functional builder is a clean one-liner that matches the OCaml source directly and has no performance downside.
