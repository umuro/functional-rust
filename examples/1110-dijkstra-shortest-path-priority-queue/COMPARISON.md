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

### Rust (idiomatic)

```rust
pub fn dijkstra(
    graph: &HashMap<String, Vec<(String, i32)>>,
    start: &str,
) -> HashMap<String, i32> {
    let mut dist: HashMap<String, i32> = HashMap::from([(start.to_string(), 0)]);
    let mut heap: BinaryHeap<Reverse<(i32, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        // stale-entry filter: replaces OCaml Set's no-duplicate guarantee
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }
        let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
        for (v, w) in neighbors {
            let alt = d + w;
            if alt < dist.get(v).copied().unwrap_or(i32::MAX) {
                dist.insert(v.clone(), alt);
                heap.push(Reverse((alt, v.clone())));
            }
        }
    }
    dist
}
```

### Rust (functional / set-based)

```rust
pub fn dijkstra_functional(
    graph: &BTreeMap<String, Vec<(String, i32)>>,
    start: &str,
) -> BTreeMap<String, i32> {
    fn go(
        graph: &BTreeMap<String, Vec<(String, i32)>>,
        mut pq: BTreeSet<(i32, String)>,
        mut dist: BTreeMap<String, i32>,
    ) -> BTreeMap<String, i32> {
        let Some((d, u)) = pq.pop_first() else { return dist };

        let (dist, pq) = graph
            .get(&u)
            .map(Vec::as_slice)
            .unwrap_or(&[])
            .iter()
            .fold((dist, pq), |(mut dist, mut pq), (v, w)| {
                let alt = d + w;
                if alt < dist.get(v).copied().unwrap_or(i32::MAX) {
                    dist.insert(v.clone(), alt);
                    pq.insert((alt, v.clone()));
                }
                (dist, pq)
            });

        go(graph, pq, dist)
    }

    let dist = BTreeMap::from([(start.to_string(), 0)]);
    let pq = BTreeSet::from([(0i32, start.to_string())]);
    go(graph, pq, dist)
}
```

## Type Signatures

| Concept | OCaml | Rust (idiomatic) |
|---------|-------|------------------|
| Graph type | `(string * int) list SMap.t` | `&HashMap<String, Vec<(String, i32)>>` |
| Distance map | `int SMap.t` (persistent) | `HashMap<String, i32>` (mutable) |
| Priority queue | `PQ.t` (ordered BST, no duplicates) | `BinaryHeap<Reverse<(i32, String)>>` |
| Min extraction | `PQ.min_elt pq` + `PQ.remove` | `heap.pop()` |
| Not-found default | `try find v dist with Not_found -> max_int` | `.unwrap_or(i32::MAX)` |
| Function signature | `val dijkstra : ... SMap.t -> string -> int SMap.t` | `fn dijkstra(&HashMap<..>, &str) -> HashMap<String, i32>` |

## Key Insights

1. **Priority queue: Set vs Heap.** OCaml's `Set.Make` is a balanced BST — inserting `(alt, v)` when a shorter path is found automatically replaces any existing entry for the same `(d, v)` pair if one exists (because the comparison function treats them as distinct keys). Rust's `BinaryHeap` allows unlimited duplicates; when a node's distance improves, the old entry remains in the heap. The stale-entry check (`d > dist[u] → continue`) makes this safe with zero extra bookkeeping.

2. **Atomic pop.** OCaml requires two calls — `PQ.min_elt` to peek and `PQ.remove` to delete. The functional Rust version uses `BTreeSet::pop_first()` (stable since Rust 1.66) which atomically removes and returns the minimum, eliminating the need to clone the key just for the removal call.

3. **Persistent vs mutable maps.** OCaml's `Map.add v alt dist` returns a *new* map, leaving the original unchanged — this is what makes the fold accumulator pattern work without explicit mutation. Rust's `HashMap::insert` mutates in place. The functional Rust variant uses `mut dist` inside closures, which is idiomatic Rust — there is no `mut` at the call site because the map is passed by ownership through the fold accumulator.

4. **Tail-call optimisation.** OCaml guarantees that `let rec go pq dist = ... go pq dist` compiles to a loop (TCO). Rust has no such guarantee; the recursive `dijkstra_functional` will stack-overflow on deep graphs. The idiomatic `while let` loop in `dijkstra` is always safe. For production use, always prefer the iterative form in Rust.

5. **Exception vs Option for defaults.** OCaml uses `try SMap.find v dist with Not_found -> max_int` — an exception is thrown and caught to supply the default distance. Rust uses `HashMap::get(v).copied().unwrap_or(i32::MAX)` — the same default is expressed as a pure `Option` combinator chain, with no implicit control-flow transfer.

## When to Use Each Style

**Use idiomatic Rust (`BinaryHeap` + `HashMap`) when:** writing production or performance-sensitive code. Binary heap has O(1) amortized push and O(log n) pop; `HashMap` has O(1) amortized lookup. No stack-overflow risk.

**Use functional Rust (`BTreeSet` + `BTreeMap`) when:** porting OCaml code directly for educational purposes, or when you need the distance map to be naturally sorted in the output (e.g. for deterministic iteration order in tests or reports).
