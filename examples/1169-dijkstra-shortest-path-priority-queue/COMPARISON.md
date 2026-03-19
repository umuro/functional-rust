# OCaml vs Rust: Dijkstra's Shortest Path with a Priority Queue

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
pub fn dijkstra<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    dist.insert(start, 0);
    let mut heap: BinaryHeap<Reverse<(u32, &str)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > *dist.get(u).unwrap_or(&u32::MAX) {
            continue;                    // stale entry — skip
        }
        let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
        for &(v, w) in neighbors {
            let alt = d + w;
            if alt < *dist.get(v).unwrap_or(&u32::MAX) {
                dist.insert(v, alt);
                heap.push(Reverse((alt, v)));
            }
        }
    }
    dist
}
```

### Rust (functional/recursive — mirrors OCaml's `rec go`)

```rust
pub fn dijkstra_functional<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u32> {
    let dist = std::iter::once((start, 0u32)).collect();
    let heap = std::iter::once(Reverse((0u32, start))).collect();
    go(graph, heap, dist)
}

fn go<'a>(
    graph: &Graph<'a>,
    mut heap: BinaryHeap<Reverse<(u32, &'a str)>>,
    dist: HashMap<&'a str, u32>,
) -> HashMap<&'a str, u32> {
    let Some(Reverse((d, u))) = heap.pop() else {
        return dist;                     // base case: PQ.is_empty pq
    };
    if d > *dist.get(u).unwrap_or(&u32::MAX) {
        return go(graph, heap, dist);    // stale — tail-recurse unchanged
    }
    let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
    let (dist, heap) = neighbors.iter()  // List.fold_left → .iter().fold(…)
        .fold((dist, heap), |(mut d_map, mut h), &(v, w)| {
            let alt = d + w;
            if alt < *d_map.get(v).unwrap_or(&u32::MAX) {
                d_map.insert(v, alt);
                h.push(Reverse((alt, v)));
            }
            (d_map, h)
        });
    go(graph, heap, dist)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val dijkstra : int list SMap.t -> string -> int SMap.t` | `fn dijkstra<'a>(graph: &Graph<'a>, start: &'a str) -> HashMap<&'a str, u32>` |
| Priority queue type | `PQ.t` (`Set.Make` over `(int * string)`) | `BinaryHeap<Reverse<(u32, &str)>>` |
| Distance map type | `int SMap.t` (`Map.Make(String)`) | `HashMap<&str, u32>` |
| Sentinel for "infinity" | `max_int` | `u32::MAX` (or absent from map) |
| Min-element extraction | `PQ.min_elt pq` + `PQ.remove` | `heap.pop()` (single O(log n) step) |
| Neighbour iteration | `List.fold_left (fun acc (v,w) -> …) init neighbors` | `.iter().fold(init, \|acc, &(v,w)\| …)` |

## Key Insights

1. **`Set.Make` ≠ `BinaryHeap` in invariant:** OCaml's `Set` stores unique elements, so re-inserting `(alt, v)` after a relaxation replaces the old entry automatically. Rust's `BinaryHeap` allows duplicates, so outdated `(old_dist, v)` entries pile up and must be filtered with the `if d > dist[u] { continue }` guard.

2. **`Reverse` is the idiomatic min-heap:** Rust's `BinaryHeap` implements a max-heap (the most common use case for event scheduling). `std::cmp::Reverse<T>` flips the `Ord` comparison without any extra crate — it is the canonical, zero-cost way to get a min-heap from `BinaryHeap`.

3. **Ownership enables in-place mutation:** OCaml's purely functional maps return new values on every `add`; the GC collects the old ones. Rust's `HashMap::insert` mutates the existing allocation — no allocation per relaxation step — while ownership guarantees there are no aliased references to the old value.

4. **`let else` replaces pattern-matching on `Option`:** Rust's `let Some(x) = expr else { return … }` is the direct equivalent of OCaml's early-exit `if … is_empty then … else let x = …`. It keeps the happy path un-indented without needing a full `match`.

5. **Lifetime annotations encode the OCaml module boundary:** OCaml's `SMap.Make(String)` ties the key type to `string` structurally. Rust's `'a` lifetime ties all `&str` keys in the graph and the result map to the same borrow scope, preventing dangling references to node names — a guarantee OCaml gets for free from GC.

## When to Use Each Style

**Use idiomatic Rust when:** writing production code, algorithms on large graphs, or any context where readability and debuggability matter. The `while let` loop is linear in appearance and easy to step through with a debugger.

**Use recursive Rust when:** demonstrating the direct OCaml translation, teaching the equivalence between tail recursion and loops, or in contexts where the immutable-accumulator mental model aids reasoning about correctness (e.g. property-based testing with fixed state snapshots).
