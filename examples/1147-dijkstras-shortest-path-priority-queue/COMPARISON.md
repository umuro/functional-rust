# OCaml vs Rust: Dijkstra's Shortest Path — Functional Priority Queue

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

### Rust (idiomatic — BTreeSet priority queue)

```rust
use std::collections::{BTreeMap, BTreeSet};

type Graph = BTreeMap<String, Vec<(String, usize)>>;

fn dijkstra(graph: &Graph, start: &str) -> BTreeMap<String, usize> {
    let mut dist: BTreeMap<String, usize> = BTreeMap::from([(start.to_string(), 0)]);
    let mut pq: BTreeSet<(usize, String)> = BTreeSet::from([(0, start.to_string())]);

    while let Some((d, u)) = pq.iter().next().cloned() {
        pq.remove(&(d, u.clone()));
        if dist.get(&u).is_some_and(|&best| d > best) { continue; }
        let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
        for (v, w) in neighbors {
            let alt = d + w;
            if alt < *dist.get(v).unwrap_or(&usize::MAX) {
                dist.insert(v.clone(), alt);
                pq.insert((alt, v.clone()));
            }
        }
    }
    dist
}
```

### Rust (functional/recursive — mirrors OCaml's `let rec go`)

```rust
fn dijkstra_recursive(graph: &Graph, start: &str) -> BTreeMap<String, usize> {
    let dist = BTreeMap::from([(start.to_string(), 0)]);
    let pq = BTreeSet::from([(0usize, start.to_string())]);
    go(graph, pq, dist)
}

fn go(
    graph: &Graph,
    mut pq: BTreeSet<(usize, String)>,
    dist: BTreeMap<String, usize>,
) -> BTreeMap<String, usize> {
    let Some((d, u)) = pq.iter().next().cloned() else { return dist; };
    pq.remove(&(d, u.clone()));
    let neighbors = graph.get(&u).map(Vec::as_slice).unwrap_or(&[]);
    // Mirrors: List.fold_left (fun (dist, pq) (v, w) -> ...) (dist, pq) neighbors
    let (dist, pq) = neighbors
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
    go(graph, pq, dist)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Priority queue type | `PQ.t` (= `Set.Make(int * string)`) | `BTreeSet<(usize, String)>` |
| Distance map type | `int SMap.t` (= `Map.Make(String)`) | `BTreeMap<String, usize>` |
| Graph type | `(string * int) list SMap.t` | `BTreeMap<String, Vec<(String, usize)>>` |
| Dijkstra signature | `val dijkstra : graph -> string -> int SMap.t` | `fn dijkstra(graph: &Graph, start: &str) -> BTreeMap<String, usize>` |
| Recursive worker | `let rec go : PQ.t -> int SMap.t -> int SMap.t` | `fn go(graph: &Graph, pq: BTreeSet<…>, dist: BTreeMap<…>) -> BTreeMap<…>` |

## Key Insights

1. **`Set.Make` ≈ `BTreeSet`**: OCaml's ordered-set functor gives min-extraction
   in O(log n) via `min_elt`; `BTreeSet::iter().next()` achieves the same because
   B-trees store entries in sorted order.  Both structures treat `(dist, node)`
   tuples as the unique key, so the same node can appear at multiple distances —
   stale entries are naturally superseded.

2. **Persistent vs mutable maps**: OCaml's `Map.Make` is purely functional — each
   `SMap.add` returns a new map without touching the old one. Rust's `BTreeMap` is
   mutable. However, in the recursive `go` the maps are moved by value, so the
   ownership semantics produce the same single-threaded semantics: at any point
   only one version of `dist`/`pq` is live.

3. **`List.fold_left` ≈ `Iterator::fold`**: The OCaml `fold_left (fun (d,p) (v,w) -> ...) (dist,pq) neighbors`
   translates almost character-for-character into Rust's `.fold((dist, pq), |(mut dist, mut pq), (v, w)| {...})`.
   Both accumulate `(dist, pq)` as a pair and return the updated pair.

4. **Tail-call optimisation**: OCaml's compiler turns `let rec go … = … go pq dist`
   into a loop (TCO guaranteed in many cases). Rust makes no such guarantee — `fn go`
   recurses on the stack.  For small inputs this is fine; for large graphs, convert
   to a `while` loop (the `dijkstra` function above).

5. **Sorted output for free**: Both `Map.Make(String)` and `BTreeMap` iterate in
   ascending key order — no post-processing needed to get deterministic output, unlike
   `HashMap` which would require an explicit `.sort()`.

## When to Use Each Style

**Use the iterative BTreeSet style when:** you want a faithful structural mirror
of the OCaml `Set.Make` idiom and sorted output, but need a safe iterative loop
that handles graphs of arbitrary depth without stack risk.

**Use the recursive fold style when:** demonstrating the direct OCaml→Rust
translation for teaching, or when graphs are small and the `List.fold_left` parallel
is the primary learning goal.
