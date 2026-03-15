# OCaml vs Rust: Dijkstra's Shortest Path

## Side-by-Side Code

### OCaml

```ocaml
(* Priority queue as sorted association list — purely functional *)
module PQ = struct
  type t = (int * int) list

  let insert dist node pq =
    let rec go = function
      | [] -> [(dist, node)]
      | ((d, _) as x) :: rest ->
        if dist <= d then (dist, node) :: x :: rest
        else x :: go rest
    in go pq

  let pop = function
    | [] -> None
    | (d, n) :: rest -> Some ((d, n), rest)
end

let dijkstra graph source =
  let dist = IntMap.singleton source 0 in
  let pq = PQ.insert 0 source PQ.empty in
  let rec loop pq dist =
    match PQ.pop pq with
    | None -> dist
    | Some ((d, u), pq') ->
      let current_dist = try IntMap.find u dist with Not_found -> max_int in
      if d > current_dist then loop pq' dist
      else
        let neighbors = try IntMap.find u graph with Not_found -> [] in
        let pq'', dist' =
          List.fold_left (fun (pq_acc, dist_acc) (v, w) ->
            let new_dist = d + w in
            let old_dist = try IntMap.find v dist_acc with Not_found -> max_int in
            if new_dist < old_dist then
              (PQ.insert new_dist v pq_acc, IntMap.add v new_dist dist_acc)
            else (pq_acc, dist_acc)
          ) (pq', dist) neighbors
        in loop pq'' dist'
  in loop pq dist
```

### Rust (idiomatic — BinaryHeap min-heap via reversed Ord)

```rust
#[derive(Clone, Eq, PartialEq)]
struct State { cost: u64, node: usize }

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering: BinaryHeap is max-heap, we want min-heap
        other.cost.cmp(&self.cost).then_with(|| self.node.cmp(&other.node))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

pub fn dijkstra(graph: &Graph, source: usize) -> HashMap<usize, u64> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(source, 0);
    heap.push(State { cost: 0, node: source });

    while let Some(State { cost, node }) = heap.pop() {
        if cost > *dist.get(&node).unwrap_or(&u64::MAX) { continue; }

        if let Some(edges) = graph.get(&node) {
            for edge in edges {
                let new_dist = cost + edge.weight;
                if new_dist < *dist.get(&edge.to).unwrap_or(&u64::MAX) {
                    dist.insert(edge.to, new_dist);
                    heap.push(State { cost: new_dist, node: edge.to });
                }
            }
        }
    }
    dist
}
```

### Rust (functional graph construction — fold over edge list)

```rust
pub fn build_graph(edges: &[(usize, usize, u64)]) -> Graph {
    edges.iter().fold(HashMap::new(), |mut graph, &(from, to, weight)| {
        graph.entry(from).or_default().push(Edge { to, weight });
        graph
    })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Distance map type | `int IntMap.t` | `HashMap<usize, u64>` |
| Priority queue | `(int * int) list` (sorted) | `BinaryHeap<State>` |
| Graph type | `(int * int) list IntMap.t` | `HashMap<usize, Vec<Edge>>` |
| Dijkstra signature | `graph -> int -> int IntMap.t` | `fn dijkstra(graph: &Graph, source: usize) -> HashMap<usize, u64>` |
| Optional result | `'a option` | `Option<T>` |
| Fold accumulator | `('a * 'b)` tuple | `(pq_acc, dist_acc)` destructured |

## Key Insights

1. **Priority Queue Strategy:** OCaml builds a purely functional sorted list — insertion is O(n) but immutable. Rust uses `BinaryHeap` (O(log n)), but since Rust only provides max-heap, a custom `Ord` implementation reverses the comparison to achieve min-heap behavior. This is a classic Rust idiom: newtype + reversed `Ord`.

2. **Immutability vs Ownership:** OCaml's `IntMap.add` produces a new map each step — true immutability via structural sharing. Rust's `HashMap` is mutated in-place, but Rust's ownership model ensures no aliased mutable references exist, providing the same safety guarantees without the allocation cost.

3. **Stale-Entry Handling:** Both languages use the same logical pattern — push multiple entries for a node and skip outdated ones. OCaml checks `d > current_dist` via pattern match; Rust checks `cost > dist[node]` in the `while let` loop. The functional insight: lazy deletion is valid because we only commit a distance when it's minimal.

4. **`fold_left` vs Iterator Chains:** OCaml's `List.fold_left` explicitly threads `(pq_acc, dist_acc)` as a tuple through each neighbor. Rust expresses the same accumulation as a `for` loop over `edges` mutating `dist` and `heap` — idiomatic Rust prefers direct mutation of owned data over tuple threading when the data is already exclusively owned.

5. **Path Reconstruction:** Not present in the OCaml version (distance-only). Rust adds `shortest_path` using an iterator chain with `.flat_map`, `.filter`, `.map`, `.next()` to backtrack through the distance map — demonstrating how Rust iterator combinators replace OCaml's recursive list comprehensions for search problems.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap + HashMap):** When performance matters — this is O((V + E) log V) with low constant factors, no GC pauses, and no allocations per map update.

**Use the OCaml purely functional style:** When you want proof-friendly code or need to checkpoint algorithm state (the immutable map is a snapshot of distances at each step, enabling backtracking or parallel exploration without copying).
