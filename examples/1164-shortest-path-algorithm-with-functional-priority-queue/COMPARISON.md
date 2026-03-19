# OCaml vs Rust: Shortest Path with Functional Priority Queue

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
pub fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, u32> {
    let mut dist: HashMap<String, u32> = HashMap::new();
    dist.insert(start.to_string(), 0);

    let mut heap: BinaryHeap<Reverse<(u32, String)>> = BinaryHeap::new();
    heap.push(Reverse((0, start.to_string())));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist.get(&u).copied().unwrap_or(u32::MAX) {
            continue;
        }
        if let Some(neighbors) = graph.get(&u) {
            for (v, w) in neighbors {
                let alt = d + w;
                if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                    dist.insert(v.clone(), alt);
                    heap.push(Reverse((alt, v.clone())));
                }
            }
        }
    }
    dist
}
```

### Rust (functional — BTreeSet mirrors OCaml's Set.Make)
```rust
pub fn dijkstra_functional(graph: &FunctionalGraph, start: &str) -> BTreeMap<String, u32> {
    let mut dist: BTreeMap<String, u32> = BTreeMap::new();
    dist.insert(start.to_string(), 0);

    let mut pq: BTreeSet<(u32, String)> = BTreeSet::new();
    pq.insert((0, start.to_string()));

    while let Some(entry) = pq.iter().next().cloned() {
        pq.remove(&entry);
        let (d, u) = entry;

        if d > dist.get(&u).copied().unwrap_or(u32::MAX) {
            continue;
        }
        if let Some(neighbors) = graph.get(&u) {
            for (v, w) in neighbors {
                let alt = d + w;
                if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                    dist.insert(v.clone(), alt);
                    pq.insert((alt, v.clone()));
                }
            }
        }
    }
    dist
}
```

## Type Signatures

| Concept | OCaml | Rust (idiomatic) |
|---------|-------|------------------|
| Graph type | `(string * (string * int) list) SMap.t` | `HashMap<String, Vec<(String, u32)>>` |
| Distance map | `int SMap.t` | `HashMap<String, u32>` |
| Priority queue | `PQ.t` (`Set.Make` of `int * string`) | `BinaryHeap<Reverse<(u32, String)>>` |
| Functional PQ | `PQ.t` (same — always functional) | `BTreeSet<(u32, String)>` |
| Function signature | `dijkstra : ... SMap.t -> string -> int SMap.t` | `fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, u32>` |

## Key Insights

1. **OCaml's `Set.Make` is a persistent sorted set used as a priority queue.** It keeps elements in sorted order by the custom comparator `(distance, node)`, so `PQ.min_elt` is O(log n). Rust's `BTreeSet<(u32, String)>` is the direct structural analog — `BTreeSet` is also a sorted ordered set, and its first element in iteration order is always the minimum.

2. **`BinaryHeap<Reverse<...>>` is the idiomatic Rust choice.** It's a standard binary heap with O(log n) push and pop. Wrapping with `Reverse` inverts the max-heap ordering into a min-heap. This is more cache-friendly and lower-overhead than BTreeSet for the priority queue use case.

3. **Lazy deletion is identical in both languages.** OCaml checks `if d > dist_u then go pq' dist` to skip stale PQ entries. Rust checks `if d > dist[u] { continue }`. Neither eagerly removes outdated entries from the queue — they're just discarded when encountered.

4. **Borrow checker changes the BTreeSet access pattern.** In OCaml, `PQ.min_elt pq` and `PQ.remove (d,u) pq` are two separate operations on an immutable structure — no aliasing concern. In Rust, `pq.iter().next()` borrows `pq` immutably; we must call `.cloned()` to obtain an owned copy and release the borrow *before* calling `pq.remove(...)`. This is a direct consequence of Rust's aliasing rules.

5. **Immutable threading vs. mutable update.** OCaml's `go pq dist` passes updated `pq` and `dist` as new values in each recursive call — purely functional. Rust mutates `dist` and `heap`/`pq` in place inside a `while` loop. The observable behavior is identical; the implementation strategy differs completely.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap) when:** you want maximum performance for graph algorithms — BinaryHeap is O(log n) push/pop with good cache behavior, and is what production Rust graph libraries use.

**Use functional Rust (BTreeSet) when:** you want the code structure to closely mirror an OCaml or Haskell implementation for pedagogical purposes, or when you need the set semantics (e.g., guaranteed uniqueness of entries, range queries) beyond just priority ordering.
