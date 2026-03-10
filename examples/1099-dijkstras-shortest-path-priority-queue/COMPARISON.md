# OCaml vs Rust: Dijkstra's Shortest Path with Priority Queue

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

### Rust (idiomatic)
```rust
pub fn dijkstra_idiomatic<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u64> {
    let mut dist: HashMap<&'a str, u64> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(u64, &'a str)>> = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > *dist.get(u).unwrap_or(&u64::MAX) {
            continue;
        }
        if let Some(neighbors) = graph.get(u) {
            for &(v, w) in neighbors {
                let alt = d + w;
                let current = *dist.get(v).unwrap_or(&u64::MAX);
                if alt < current {
                    dist.insert(v, alt);
                    heap.push(Reverse((alt, v)));
                }
            }
        }
    }
    dist
}
```

### Rust (functional/recursive)
```rust
pub fn dijkstra_functional<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u64> {
    type Pq<'a> = BTreeSet<(u64, &'a str)>;

    fn go<'a>(graph: &Graph<'a>, pq: Pq<'a>, dist: HashMap<&'a str, u64>) -> HashMap<&'a str, u64> {
        let &(d, u) = match pq.iter().next() {
            None => return dist,
            Some(min) => min,
        };
        let mut pq = pq;
        pq.remove(&(d, u));

        if d > *dist.get(u).unwrap_or(&u64::MAX) {
            return go(graph, pq, dist);
        }

        let empty: Vec<(&str, u64)> = Vec::new();
        let neighbors = graph.get(u).unwrap_or(&empty);
        let (dist, pq) = neighbors.iter()
            .fold((dist, pq), |(mut dist, mut pq), &(v, w)| {
                let alt = d + w;
                let current = *dist.get(v).unwrap_or(&u64::MAX);
                if alt < current {
                    dist.insert(v, alt);
                    pq.insert((alt, v));
                }
                (dist, pq)
            });
        go(graph, pq, dist)
    }

    let mut dist = HashMap::new();
    dist.insert(start, 0);
    let mut pq = BTreeSet::new();
    pq.insert((0u64, start));
    go(graph, pq, dist)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Graph type | `(string * int) list SMap.t` | `HashMap<&str, Vec<(&str, u64)>>` |
| Priority queue | `PQ.t` (Set of `int * string`) | `BinaryHeap<Reverse<(u64, &str)>>` |
| Distance map | `int SMap.t` | `HashMap<&str, u64>` |
| Function signature | `val dijkstra : (string * int) list SMap.t -> string -> int SMap.t` | `fn dijkstra<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u64>` |
| Missing key | `Not_found` exception | `.get().unwrap_or(&u64::MAX)` |

## Key Insights

1. **Priority queue duality:** OCaml's `Set.Make` serves as both a sorted collection and a priority queue (via `min_elt`). Rust separates these — `BinaryHeap` for priority queues, `BTreeSet` for ordered sets. The functional Rust version uses `BTreeSet` to mirror the OCaml approach directly.
2. **Ownership threading:** OCaml's `go pq dist` passes immutable values that are structurally shared. Rust's functional version moves ownership of `HashMap` and `BTreeSet` through each recursive call — no cloning needed because we transfer, not share.
3. **Error handling vs defaults:** OCaml uses `try ... with Not_found` exception handling for missing map keys. Rust uses `.get().unwrap_or(&u64::MAX)` — a combinator approach that avoids exceptions entirely and is branch-predictor friendly.
4. **Stale entry handling:** Both versions may push duplicate entries into the priority queue. OCaml's set automatically deduplicates by `(distance, node)` pair. Rust's `BinaryHeap` does not deduplicate, so we check `d > dist[u]` to skip stale entries.
5. **Lifetime annotations:** Rust requires explicit `'a` lifetimes to prove that node references (`&'a str`) in the result live as long as the input graph. OCaml's GC handles this implicitly — the string values just stay alive as long as they're referenced.

## When to Use Each Style

**Use idiomatic Rust when:** Building production graph algorithms — `BinaryHeap` with `Reverse` is the standard Rust pattern for Dijkstra, offers excellent cache performance, and the imperative loop is clear and efficient.

**Use recursive Rust when:** Teaching the algorithm's structure or when you want a direct translation from a functional specification — the `BTreeSet` + `fold` + recursion pattern maps 1:1 to the OCaml and makes correctness reasoning easier.
