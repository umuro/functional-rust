# OCaml vs Rust: Dijkstra's Shortest Path with Priority Queue

## Side-by-Side Code

### OCaml

```ocaml
module PQ = Set.Make (struct
  type t = int * string
  let compare (d1, n1) (d2, n2) = compare (d1, n1) (d2, n2)
end)

module SMap = Map.Make (String)

let dijkstra graph start =
  let dist = SMap.singleton start 0 in
  let pq = PQ.singleton (0, start) in
  let rec go pq dist =
    if PQ.is_empty pq then dist
    else
      let (d, u) = PQ.min_elt pq in
      let pq = PQ.remove (d, u) pq in
      let neighbors = try SMap.find u graph with Not_found -> [] in
      let dist, pq =
        List.fold_left (fun (dist, pq) (v, w) ->
          let alt = d + w in
          let current = try SMap.find v dist with Not_found -> max_int in
          if alt < current then
            (SMap.add v alt dist, PQ.add (alt, v) pq)
          else (dist, pq)
        ) (dist, pq) neighbors
      in
      go pq dist
  in
  go pq dist
```

### Rust (idiomatic)

```rust
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type Graph = HashMap<String, Vec<(String, usize)>>;

pub fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut dist: HashMap<String, usize> = HashMap::from([(start.to_string(), 0)]);
    let mut heap: BinaryHeap<Reverse<(usize, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }
        let Some(neighbors) = graph.get(&u) else { continue };
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

### Rust (functional/recursive style)

```rust
pub fn dijkstra_functional(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut dist: HashMap<String, usize> = HashMap::from([(start.to_string(), 0)]);
    let mut heap: BinaryHeap<Reverse<(usize, String)>> =
        BinaryHeap::from([Reverse((0, start.to_string()))]);

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist.get(&u).is_some_and(|&best| d > best) {
            continue;
        }

        // Mirrors OCaml's List.fold_left — compute updates before applying them
        let relaxed: Vec<(String, usize)> = graph
            .get(&u)
            .into_iter()
            .flatten()
            .filter_map(|(v, w)| {
                let alt = d + w;
                (alt < *dist.get(v).unwrap_or(&usize::MAX)).then_some((v.clone(), alt))
            })
            .collect();

        dist.extend(relaxed.iter().cloned());
        heap.extend(relaxed.into_iter().map(|(v, alt)| Reverse((alt, v))));
    }
    dist
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Priority queue | `Set.Make` (balanced BST, persistent) | `BinaryHeap<Reverse<...>>` (mutable heap) |
| Priority queue element | `int * string` | `Reverse<(usize, String)>` |
| Distance map | `Map.Make(String)` (persistent) | `HashMap<String, usize>` (mutable) |
| Graph type | `(string * int) list SMap.t` | `HashMap<String, Vec<(String, usize)>>` |
| Infinity sentinel | `max_int` | `usize::MAX` |
| Function signature | `SMap.t -> string -> int SMap.t` | `fn(&Graph, &str) -> HashMap<String, usize>` |

## Key Insights

1. **Priority queue trick:** Both languages encode a min-heap using tuple comparison. OCaml's `Set.Make` with `compare (d1,n1) (d2,n2)` gives `min_elt` in O(log n). Rust's `BinaryHeap` is a max-heap by default; wrapping elements in `Reverse<(usize, String)>` makes `pop()` yield the minimum distance.

2. **Immutability vs mutability:** OCaml threads `(pq, dist)` as immutable values through a tail-recursive function — each iteration produces *new* versions. Rust mutates `heap` and `dist` in place inside a `while let` loop. The algorithm is the same; only the plumbing differs.

3. **Stale-entry handling:** OCaml's functional PQ allows `PQ.remove` to eagerly evict an outdated entry when a shorter path is found. Rust's idiomatic style pushes a *new* entry and skips stale ones at pop time (`if d > best { continue }`). This is simpler but keeps more entries in the heap.

4. **Borrow checker forces staging:** The functional Rust version can't call `dist.get()` inside `filter_map` and then also call `dist.insert()` in the same loop body — that's two conflicting borrows. The fix is to `.collect()` the relaxed edges first, then `extend`. OCaml's immutable data structures sidestep this: reading the old `dist` and producing a new one are separate values in the type system.

5. **Error handling style:** OCaml uses `try SMap.find v dist with Not_found -> max_int`. Rust uses `dist.get(v).unwrap_or(&usize::MAX)` — no exceptions, just `Option` combinators.

## When to Use Each Style

**Use idiomatic Rust when:** you need straightforward, easy-to-read code for an imperative algorithm. The `for` loop over neighbors is clear and the heap mechanics are explicit.

**Use functional Rust when:** you want to make the "compute updates, then apply" pattern visible — this mirrors OCaml's `List.fold_left` and makes data flow explicit at the cost of an intermediate `Vec` allocation.
