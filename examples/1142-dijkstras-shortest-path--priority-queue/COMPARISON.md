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

### Rust (idiomatic — imperative heap)

```rust
pub fn dijkstra<'a>(graph: &'a Graph, start: &'a str) -> HashMap<&'a str, u32> {
    let mut dist: HashMap<&str, u32> = HashMap::new();
    dist.insert(start, 0);

    let mut heap: BinaryHeap<(Reverse<u32>, &str)> = BinaryHeap::new();
    heap.push((Reverse(0), start));

    while let Some((Reverse(d), u)) = heap.pop() {
        if dist.get(u).is_some_and(|&best| d > best) { continue; }
        let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
        for &(v, w) in neighbors {
            let alt = d + w;
            let current = dist.get(v).copied().unwrap_or(u32::MAX);
            if alt < current {
                dist.insert(v, alt);
                heap.push((Reverse(alt), v));
            }
        }
    }
    dist
}
```

### Rust (functional/recursive — mirrors OCaml's `go`)

```rust
fn go<'a>(
    graph: &'a Graph,
    mut heap: BinaryHeap<(Reverse<u32>, &'a str)>,
    dist: HashMap<&'a str, u32>,
) -> HashMap<&'a str, u32> {
    match heap.pop() {
        None => dist,
        Some((Reverse(d), u)) => {
            if dist.get(u).is_some_and(|&best| d > best) {
                return go(graph, heap, dist);
            }
            let neighbors = graph.get(u).map(Vec::as_slice).unwrap_or(&[]);
            let (dist, heap) =
                neighbors.iter().fold((dist, heap), |(mut dist, mut heap), &(v, w)| {
                    let alt = d + w;
                    let current = dist.get(v).copied().unwrap_or(u32::MAX);
                    if alt < current {
                        dist.insert(v, alt);
                        heap.push((Reverse(alt), v));
                    }
                    (dist, heap)
                });
            go(graph, heap, dist)
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Graph type | `(string * (string * int) list) list` via `SMap` | `HashMap<&str, Vec<(&str, u32)>>` |
| Distance map | `int SMap.t` (persistent) | `HashMap<&str, u32>` (mutable) |
| Priority queue | `PQ.t` (ordered `Set` of `int * string`) | `BinaryHeap<(Reverse<u32>, &str)>` |
| Result | `int SMap.t` | `HashMap<&str, u32>` |
| Infinity sentinel | `max_int` | `u32::MAX` |

## Key Insights

1. **`Reverse` as min-heap adapter:** OCaml's `Set` naturally gives `min_elt` in O(log n). Rust's `BinaryHeap` is a max-heap; wrapping the priority in `Reverse<u32>` flips the ordering with zero cost — the compiler optimises it away entirely.

2. **Stale entries are harmless:** OCaml's `Set` is a mathematical set — adding `(alt, v)` when a better entry `(old, v)` already exists simply inserts both, but `min_elt` will always pop the smaller one first and the larger is popped when `dist[v]` is already better. Rust's heap can accumulate duplicates; the `d > best` guard skips them identically.

3. **Persistent vs mutable maps:** OCaml's `SMap.add` returns a *new* map (structural sharing makes this cheap). Rust's `HashMap::insert` mutates in place with O(1) amortised cost. The functional Rust version threads `dist` ownership through the recursion, giving the same *structural* behaviour as OCaml even though the underlying HashMap is mutable.

4. **`fold` as the OCaml parallel:** The OCaml `List.fold_left` over neighbours appears verbatim as `.iter().fold(...)` in Rust. Both accumulate `(dist, pq)` through the neighbour list — the translation is almost mechanical.

5. **Lifetime annotations:** Rust nodes are `&'a str` slices that borrow from the caller-owned graph. OCaml strings are GC-managed — no lifetime tracking needed. The `'a` annotations make the borrow relationships explicit without any runtime cost.

## When to Use Each Style

**Use idiomatic Rust (imperative heap) when:** you want maximum performance and clarity — the `while let` loop is familiar to any Rust developer and compiles to tight code with no recursion overhead.

**Use recursive Rust (functional style) when:** you are porting OCaml or Haskell code and want the structure to remain recognisable, or when you want to reason about the algorithm as a pure state transformation that a type checker can verify.
