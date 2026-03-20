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
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

pub fn dijkstra(
    graph: &HashMap<String, Vec<(String, u32)>>,
    start: &str,
) -> HashMap<String, u32> {
    let mut dist: HashMap<String, u32> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(u32, String)>> = BinaryHeap::new();

    dist.insert(start.to_string(), 0);
    heap.push(Reverse((0, start.to_string())));

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist.get(&u).is_some_and(|&cur| d > cur) {
            continue; // stale entry — skip
        }
        if let Some(neighbors) = graph.get(&u) {
            for (v, w) in neighbors {
                let alt = d.saturating_add(*w);
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

### Rust (functional/recursive — mirrors OCaml structure)
```rust
use std::collections::{BTreeMap, BTreeSet};

pub fn dijkstra_functional(
    graph: &BTreeMap<String, Vec<(String, u32)>>,
    start: &str,
) -> BTreeMap<String, u32> {
    let dist = BTreeMap::from([(start.to_string(), 0u32)]);
    let pq = BTreeSet::from([(0u32, start.to_string())]);
    go(graph, pq, dist)
}

fn go(
    graph: &BTreeMap<String, Vec<(String, u32)>>,
    mut pq: BTreeSet<(u32, String)>,
    dist: BTreeMap<String, u32>,
) -> BTreeMap<String, u32> {
    match pq.iter().next().cloned() {
        None => dist,
        Some((d, u)) => {
            pq.remove(&(d, u.clone()));
            let empty = vec![];
            let neighbors = graph.get(&u).unwrap_or(&empty);
            let (dist, pq) = neighbors
                .iter()
                .fold((dist, pq), |(mut dist, mut pq), (v, w)| {
                    let alt = d.saturating_add(*w);
                    if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                        dist.insert(v.clone(), alt);
                        pq.insert((alt, v.clone()));
                    }
                    (dist, pq)
                });
            go(graph, pq, dist)
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust (idiomatic) | Rust (functional) |
|---------|-------|-----------------|-------------------|
| Graph type | `(int * string list) SMap.t` | `HashMap<String, Vec<(String, u32)>>` | `BTreeMap<String, Vec<(String, u32)>>` |
| Distance map | `int SMap.t` | `HashMap<String, u32>` | `BTreeMap<String, u32>` |
| Priority queue | `PQ.t` (balanced BST) | `BinaryHeap<Reverse<(u32, String)>>` | `BTreeSet<(u32, String)>` |
| Min extraction | `PQ.min_elt pq` | `heap.pop()` unwraps `Reverse` | `pq.iter().next()` |
| Relaxation fold | `List.fold_left` | imperative `for` loop | `.fold()` with owned state |

## Key Insights

1. **BTreeSet as Set.Make:** OCaml's `Set.Make` with `compare (d1,n1) (d2,n2)` produces a BST ordered lexicographically by `(distance, name)`. Rust's `BTreeSet<(u32, String)>` does exactly the same — tuples compare lexicographically, so `iter().next()` yields the minimum-distance node, just like `PQ.min_elt`.

2. **Stale entry pattern vs. pure removal:** OCaml's purely functional set means adding `(alt, v)` and keeping the old entry is impossible — `PQ.add` just updates. In Rust's `BinaryHeap` (idiomatic), duplicate entries accumulate; the `is_some_and(|&cur| d > cur)` guard skips stale pops. In the functional Rust version using `BTreeSet`, the same duplicate issue can occur, but since we only insert when we improve the distance, the old entry may linger until popped and skipped.

3. **Ownership mimics immutability:** OCaml `PQ.remove (d, u) pq` returns a new set; the old binding `pq` is never mutated. In Rust, `mut pq` with `pq.remove(...)` mutates in place — but since `pq` was moved into `go`, no other code sees the old version. Ownership gives the same aliasing guarantee as immutability.

4. **`fold` threading state:** OCaml's `List.fold_left (fun (dist, pq) (v, w) -> ...) (dist, pq) neighbors` threads `(dist, pq)` as accumulated state. The functional Rust version does the identical thing: `.fold((dist, pq), |(mut dist, mut pq), (v, w)| { ...; (dist, pq) })`. The `mut` bindings inside the closure shadow the outer pattern — they allow in-place updates of the owned values before passing them to the next iteration.

5. **`u32::MAX` vs. `max_int`:** OCaml uses `max_int` as a sentinel for "not yet reached." Rust uses `u32::MAX` (or `.unwrap_or(u32::MAX)`). The risk of overflow on `d + w` is guarded with `.saturating_add()` — in OCaml this isn't a concern since `max_int + anything` wraps to negative, which is a different kind of bug.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap) when:** you need maximum performance on large graphs — heap operations are O(log n) and the constant factor is smaller than a BST. Standard in competitive programming and production code.

**Use functional Rust (BTreeSet) when:** you are translating OCaml/Haskell code and want structural correspondence for reasoning or teaching. Also useful when you need ordered iteration over the priority queue for debugging.
