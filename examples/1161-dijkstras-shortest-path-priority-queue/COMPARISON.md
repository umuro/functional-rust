# OCaml vs Rust: Dijkstra's Shortest Path with Functional Priority Queue

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
pub fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, u32> {
    let mut dist: HashMap<String, u32> = HashMap::new();
    dist.insert(start.to_string(), 0);

    let mut heap: BinaryHeap<Reverse<(u32, String)>> = BinaryHeap::new();
    heap.push(Reverse((0, start.to_string())));

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist.get(&u).copied().unwrap_or(u32::MAX) < d {
            continue; // stale entry
        }
        let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
        for (v, w) in neighbors {
            let alt = d + w;
            if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                dist.insert(v.clone(), alt);
                heap.push(Reverse((alt, v.clone())));
            }
        }
    }
    dist
}
```

### Rust (functional/recursive — mirrors OCaml `go` structure)
```rust
fn go(
    graph: &Graph,
    mut pq: BinaryHeap<Reverse<(u32, String)>>,
    dist: HashMap<String, u32>,
) -> HashMap<String, u32> {
    match pq.pop() {
        None => dist,
        Some(Reverse((d, u))) => {
            if dist.get(&u).copied().unwrap_or(u32::MAX) < d {
                return go(graph, pq, dist);
            }
            let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
            let (dist, pq) = neighbors
                .iter()
                .fold((dist, pq), |(mut dist, mut pq), (v, w)| {
                    let alt = d + w;
                    if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                        dist.insert(v.clone(), alt);
                        pq.push(Reverse((alt, v.clone())));
                    }
                    (dist, pq)
                });
            go(graph, pq, dist)
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val dijkstra : graph -> string -> int SMap.t` | `fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, u32>` |
| Graph type | `(string * int) list SMap.t` | `HashMap<String, Vec<(String, u32)>>` |
| Distance map | `int SMap.t` (immutable, persistent) | `HashMap<String, u32>` (mutable) |
| Priority queue | `PQ.t` (Set of `(int * string)`) | `BinaryHeap<Reverse<(u32, String)>>` |
| Infinity sentinel | `max_int` | `u32::MAX` |

## Key Insights

1. **Persistent vs mutable collections:** OCaml's `Set.Make` and `Map.Make` give persistent, immutable data structures — each `go` call receives new versions of `pq` and `dist`. Rust's `BinaryHeap` and `HashMap` are mutable and owned, moved through the recursion (or mutated in the loop).

2. **Min-heap trick:** OCaml's `Set` ordered by `(int, string)` comparison naturally gives `min_elt` as the cheapest node. Rust's `BinaryHeap` is a max-heap, so entries are wrapped in `std::cmp::Reverse` to flip the ordering — a common Rust idiom with no OCaml counterpart needed.

3. **Stale entry handling:** OCaml's persistent `Set` prevents duplicate entries by structural equality. Rust's heap allows multiple entries for the same node (lazy deletion); a guard `if dist[u] < d { continue }` discards outdated entries. Both achieve O(E log V) overall.

4. **`fold` as the structural parallel:** OCaml's `List.fold_left` over neighbors translates directly to Rust's `.iter().fold(...)` with the same accumulator pair `(dist, pq)`. The fold signature is nearly identical in both languages.

5. **String ownership:** OCaml strings are garbage-collected and freely aliased. In Rust, inserting a `String` into both the `HashMap` and the `BinaryHeap` requires two owned values, so `.clone()` is semantically justified — not a workaround but a necessary ownership split.

## When to Use Each Style

**Use idiomatic Rust (loop + mutation) when:** performance matters and you want cache-friendly in-place mutation with no recursion overhead. This is the production-ready form.

**Use recursive Rust (functional-style) when:** you want to clearly mirror the OCaml structure for learning or review purposes, or the graph is small enough that stack depth is not a concern (Rust has no tail-call optimization).
