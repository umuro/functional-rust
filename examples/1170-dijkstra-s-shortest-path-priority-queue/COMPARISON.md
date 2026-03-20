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
pub fn dijkstra(graph: &Graph, start: &str) -> BTreeMap<String, u32> {
    let mut dist: BTreeMap<String, u32> = BTreeMap::new();
    dist.insert(start.to_owned(), 0);

    let mut heap: BinaryHeap<(Reverse<u32>, String)> = BinaryHeap::new();
    heap.push((Reverse(0), start.to_owned()));

    while let Some((Reverse(d), u)) = heap.pop() {
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue; // stale entry — lazy deletion
        }
        let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
        for (v, w) in neighbors {
            let alt = d + w;
            let current = dist.get(v).copied().unwrap_or(u32::MAX);
            if alt < current {
                dist.insert(v.clone(), alt);
                heap.push((Reverse(alt), v.clone()));
            }
        }
    }
    dist
}
```

### Rust (functional/recursive — mirrors OCaml `go pq dist`)
```rust
fn go(
    graph: &Graph,
    mut pq: BTreeMap<(u32, String), ()>,
    dist: BTreeMap<String, u32>,
) -> BTreeMap<String, u32> {
    let Some(((d, u), _)) = pq.pop_first() else {
        return dist; // base case: empty queue
    };
    if dist.get(&u).is_some_and(|&best| d > best) {
        return go(graph, pq, dist); // stale entry guard
    }
    let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
    let (dist, pq) = neighbors
        .iter()
        .fold((dist, pq), |(mut d_map, mut q), (v, w)| {
            let alt = d + w;
            let current = d_map.get(v).copied().unwrap_or(u32::MAX);
            if alt < current {
                d_map.insert(v.clone(), alt);
                q.insert((alt, v.clone()), ());
            }
            (d_map, q)
        });
    go(graph, pq, dist)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Graph type | `(string * (string * int) list) SMap.t` | `BTreeMap<String, Vec<(String, u32)>>` |
| Priority queue | `PQ.t` (balanced BST via `Set.Make`) | `BinaryHeap<(Reverse<u32>, String)>` |
| Distance map | `int SMap.t` | `BTreeMap<String, u32>` |
| Function signature | `val dijkstra : ... -> string -> int SMap.t` | `fn dijkstra(graph: &Graph, start: &str) -> BTreeMap<String, u32>` |
| Missing key sentinel | `max_int` via exception | `u32::MAX` via `.unwrap_or(u32::MAX)` |

## Key Insights

1. **Priority queue implementation:** OCaml creates an ordered set with `Set.Make`
   whose `min_elt` gives O(log n) minimum extraction. Rust uses `BinaryHeap` with
   `std::cmp::Reverse` to invert natural max-ordering into min semantics.

2. **Decrease-key vs lazy deletion:** OCaml's `Set.Make` supports `remove` of any
   element, so old `(dist, node)` entries can be replaced when a shorter path is
   found. Rust's `BinaryHeap` has no decrease-key — stale entries accumulate and
   are skipped with a guard (`d > best`) when popped.

3. **Immutability vs mutation:** OCaml's `go` is purely functional — each call
   receives fresh immutable values. The idiomatic Rust version mutates `dist` and
   `heap` inside a `while let` loop. The recursive Rust version passes owned values
   to approximate the OCaml style.

4. **Error handling for missing keys:** OCaml uses `try SMap.find k m with Not_found -> default`.
   Rust uses `.get(k).copied().unwrap_or(default)` — a combinator that eliminates exceptions.

5. **Fold over neighbours:** Both OCaml and Rust use a left fold (`List.fold_left` /
   `.iter().fold(...)`) to process neighbours functionally, threading `(dist, pq)`
   as the accumulator. This structural parallel is exact.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap) when:** you want maximum performance — `BinaryHeap`
has better cache behaviour and lower constant factors than a balanced BST.

**Use recursive Rust (BTreeMap PQ) when:** you want to demonstrate OCaml structural
equivalence or compose the algorithm as a pure function without mutable state.
