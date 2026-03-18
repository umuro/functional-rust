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
pub fn dijkstra<'a>(graph: &'a Graph, start: &'a str) -> HashMap<&'a str, u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    dist.insert(start, 0);

    let mut heap: BinaryHeap<Reverse<(u32, &str)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        // Lazy deletion: skip stale entries
        if dist.get(u).copied().unwrap_or(u32::MAX) < d {
            continue;
        }
        let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
        for &(v, w) in neighbors {
            let alt = d + w;
            if alt < dist.get(v).copied().unwrap_or(u32::MAX) {
                dist.insert(v, alt);
                heap.push(Reverse((alt, v)));
            }
        }
    }
    dist
}
```

### Rust (functional/recursive style)

```rust
pub fn dijkstra_functional<'a>(graph: &'a Graph, start: &'a str) -> HashMap<&'a str, u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    dist.insert(start, 0);
    let mut pq: BinaryHeap<Reverse<(u32, &str)>> = BinaryHeap::new();
    pq.push(Reverse((0, start)));

    fn go<'a>(
        pq: &mut BinaryHeap<Reverse<(u32, &'a str)>>,
        dist: &mut HashMap<&'a str, u32>,
        graph: &'a Graph,
    ) {
        let Some(Reverse((d, u))) = pq.pop() else { return };
        if dist.get(u).copied().unwrap_or(u32::MAX) < d {
            return go(pq, dist, graph);
        }
        let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
        let updates: Vec<_> = neighbors.iter()
            .filter_map(|&(v, w)| {
                let alt = d + w;
                (alt < dist.get(v).copied().unwrap_or(u32::MAX)).then_some((v, alt))
            })
            .collect();
        for (v, alt) in updates {
            dist.insert(v, alt);
            pq.push(Reverse((alt, v)));
        }
        go(pq, dist, graph)
    }

    go(&mut pq, &mut dist, graph);
    dist
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Graph type | `(string * (string * int) list) list` via `SMap` | `HashMap<&str, Vec<(&str, u32)>>` |
| Priority queue | `PQ.t` (ordered `Set` of `int * string`) | `BinaryHeap<Reverse<(u32, &str)>>` |
| Distance map | `SMap.t` (immutable functional map) | `HashMap<&str, u32>` (mutable) |
| Function signature | `SMap.t -> string -> SMap.t` | `fn<'a>(&'a Graph, &'a str) -> HashMap<&'a str, u32>` |
| Optional lookup | `try SMap.find v dist with Not_found -> max_int` | `.get(v).copied().unwrap_or(u32::MAX)` |

## Key Insights

1. **Immutable vs mutable data structures:** OCaml's `Set` and `Map` are persistent (immutable), so every update creates a new structure; Rust's `BinaryHeap` and `HashMap` are mutated in place. This changes the algorithm's memory profile but not its semantics.

2. **Priority queue ordering:** OCaml's `Set.Make` orders by `(distance, name)` lexicographically — cheapest first because `min_elt` is used. Rust's `BinaryHeap` is a max-heap; wrapping with `Reverse` flips the order to achieve the same min semantics.

3. **Lazy deletion vs eager removal:** OCaml removes the popped entry explicitly (`PQ.remove`) so the set stays exactly current. Rust pushes duplicate entries when a shorter path is found and skips stale ones at pop time — this is the standard Rust idiom and avoids a linear scan.

4. **Borrow checker interaction:** Rust cannot simultaneously hold a shared borrow (`dist.get(v)`) and a mutable borrow (`dist.insert(v, alt)`) in the same closure chain. The functional version collects updates into a `Vec` first to separate the two borrows — a pattern absent in OCaml where all data is immutable.

5. **Lifetimes for string keys:** Because the graph keys are `&str` references, Rust requires lifetime annotations (`'a`) to express that the keys in the result map live at least as long as the graph. OCaml's GC handles this automatically.

## When to Use Each Style

**Use idiomatic Rust when:** you want clear, readable imperative flow — the `while let` loop maps naturally to "process until empty" and is easy to follow for anyone familiar with Dijkstra's algorithm.

**Use recursive Rust when:** you want to mirror OCaml's `let rec go` structure to make the functional correspondence explicit — useful when teaching or documenting the translation step by step.
