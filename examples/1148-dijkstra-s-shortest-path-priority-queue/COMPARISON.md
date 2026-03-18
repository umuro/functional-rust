# OCaml vs Rust: Dijkstra's Shortest Path — Priority Queue

## Side-by-Side Code

### OCaml
```ocaml
module PQ = Set.Make(struct
  type t = int * string
  let compare (d1,n1) (d2,n2) = compare (d1,n1) (d2,n2)
end)
module SMap = Map.Make(String)

let dijkstra graph start =
  let dist = SMap.singleton start 0 in
  let pq = PQ.singleton (0, start) in
  let rec go pq dist =
    if PQ.is_empty pq then dist
    else
      let (d, u) = PQ.min_elt pq in
      let pq = PQ.remove (d, u) pq in
      let neighbors = try SMap.find u graph with Not_found -> [] in
      let dist, pq = List.fold_left (fun (dist, pq) (v, w) ->
        let alt = d + w in
        let current = try SMap.find v dist with Not_found -> max_int in
        if alt < current then
          (SMap.add v alt dist, PQ.add (alt, v) pq)
        else (dist, pq)
      ) (dist, pq) neighbors in
      go pq dist
  in go pq dist
```

### Rust (idiomatic — BinaryHeap min-heap)
```rust
pub fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut dist: HashMap<String, usize> = HashMap::from([(start.to_string(), 0)]);
    let mut heap: BinaryHeap<Reverse<(usize, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }
        let empty = Vec::new();
        let neighbors = graph.get(&u).unwrap_or(&empty);
        for (v, w) in neighbors {
            let alt = d + w;
            if alt < *dist.get(v).unwrap_or(&usize::MAX) {
                dist.insert(v.clone(), alt);
                heap.push(Reverse((alt, v.clone())));
            }
        }
    }
    dist
}
```

### Rust (functional — BTreeSet + fold, mirroring OCaml)
```rust
pub fn dijkstra_functional(graph: &Graph, start: &str) -> BTreeMap<String, usize> {
    let mut dist: BTreeMap<String, usize> = BTreeMap::from([(start.to_string(), 0)]);
    let mut pq: BTreeSet<(usize, String)> = BTreeSet::from([(0, start.to_string())]);

    while let Some((d, u)) = pq.iter().next().cloned() {
        pq.remove(&(d, u.clone()));
        if dist.get(&u).is_some_and(|&best| d > best) { continue; }
        let empty = Vec::new();
        let neighbors = graph.get(&u).unwrap_or(&empty);

        (dist, pq) = neighbors
            .iter()
            .fold((dist, pq), |(mut dist, mut pq), (v, w)| {
                let alt = d + w;
                let current = *dist.get(v).unwrap_or(&usize::MAX);
                if alt < current {
                    dist.insert(v.clone(), alt);
                    pq.insert((alt, v.clone()));
                }
                (dist, pq)
            });
    }
    dist
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Graph type | `(string * (string * int) list) SMap.t` | `HashMap<String, Vec<(String, usize)>>` |
| Distance map | `int SMap.t` (immutable) | `HashMap<String, usize>` (mutable) |
| Priority queue | `PQ.t` (`Set.Make` — balanced BST) | `BinaryHeap<Reverse<(usize, String)>>` |
| Functional PQ | `PQ.t` (`Set.Make`) | `BTreeSet<(usize, String)>` |
| Min extraction | `PQ.min_elt pq` | `heap.pop()` / `pq.iter().next()` |
| Fold accumulator | `(int SMap.t * PQ.t)` | `(BTreeMap<String,usize>, BTreeSet<…>)` |

## Key Insights

1. **Immutable vs mutable structures:** OCaml's `Set.Make` and `Map.Make` are persistent — every update returns a new structure, with structural sharing keeping it efficient. Rust's collections are mutable in place; the functional variant (`dijkstra_functional`) uses `fold` to thread state but mutates under the hood.

2. **Stale-entry filtering:** OCaml's `Set.Make` stores unique `(dist, node)` pairs — adding a shorter path naturally supersedes the old entry (different tuple ⇒ different set element). Rust's `BinaryHeap` cannot remove arbitrary elements, so stale entries accumulate and are skipped at pop time via `d > best` guard.

3. **`BTreeSet` as ordered-set PQ:** `BTreeSet<(usize, String)>` sorts by `(cost, name)` lexicographically — exactly like OCaml's `Set.Make` with the pair comparator. `iter().next()` gives the minimum-cost entry, mirroring `PQ.min_elt`.

4. **`fold` = `List.fold_left`:** `Iterator::fold((dist, pq), |acc, item| ...)` maps directly to OCaml's `List.fold_left (fun acc item -> ...) acc list`. Both thread a two-part accumulator (distance map + priority queue) through the neighbor list.

5. **`usize::MAX` vs `max_int`:** OCaml uses `max_int` as "infinity" for unvisited nodes. Rust uses `usize::MAX`, with `unwrap_or(&usize::MAX)` as the idiomatic equivalent of OCaml's `try SMap.find v dist with Not_found -> max_int`.

## When to Use Each Style

**Use idiomatic Rust (`BinaryHeap`)** when performance matters — binary heap push/pop is O(log n) and cache-friendly; stale-entry filtering adds negligible overhead for typical graph sizes.

**Use functional Rust (`BTreeSet` + fold)** when the OCaml translation fidelity matters for learning or when you need the priority queue to also serve as an ordered set (e.g., iteration in distance order without an extra sort).
