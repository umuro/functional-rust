# OCaml vs Rust: Graph BFS

## Side-by-Side Code

### OCaml

```ocaml
let bfs graph start =
  let visited = Hashtbl.create 16 in
  let queue = Queue.create () in
  Queue.push start queue;
  Hashtbl.add visited start true;
  let result = ref [] in
  while not (Queue.is_empty queue) do
    let node = Queue.pop queue in
    result := node :: !result;
    List.iter (fun neighbor ->
      if not (Hashtbl.mem visited neighbor) then begin
        Hashtbl.add visited neighbor true;
        Queue.push neighbor queue
      end
    ) (List.assoc node graph)
  done;
  List.rev !result
```

### Rust (idiomatic)

```rust
pub fn bfs<'a>(graph: &'a HashMap<&str, Vec<&str>>, start: &'a str) -> Vec<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut result: Vec<&str> = Vec::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        result.push(node);
        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    result
}
```

### Rust (assoc-list style, mirrors OCaml)

```rust
pub fn bfs_assoc<'a>(graph: &[(&'a str, Vec<&'a str>)], start: &'a str) -> Vec<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut result: Vec<&str> = Vec::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        result.push(node);
        if let Some((_, neighbors)) = graph.iter().find(|(n, _)| *n == node) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    result
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Graph type | `(string * string list) list` | `&HashMap<&str, Vec<&str>>` |
| Queue | `'a Queue.t` (mutable) | `VecDeque<&str>` |
| Visited set | `(string, bool) Hashtbl.t` | `HashSet<&str>` |
| Result | `string list` (via `ref` + `List.rev`) | `Vec<&str>` (pushed in order) |
| Neighbor lookup | `List.assoc node graph` — O(n) | `graph.get(node)` — O(1) average |

## Key Insights

1. **`HashSet::insert` returns `bool`:** OCaml needs two calls (`Hashtbl.mem` to check, `Hashtbl.add` to mark). Rust's `HashSet::insert` returns `false` if already present, enabling `if visited.insert(neighbor)` as a single atomic check-and-mark expression. This eliminates a TOCTOU-style bug in concurrent code and reads more clearly.

2. **`VecDeque` makes queue semantics explicit:** OCaml's `Queue.push`/`Queue.pop` always operate on the back/front respectively. Rust's `VecDeque` names the ends explicitly (`push_back`, `pop_front`), making the BFS invariant self-documenting in the code.

3. **`while let Some(...)` vs imperative `while`:** OCaml uses `while not (Queue.is_empty queue) do ... Queue.pop queue` — two operations. Rust's `while let Some(node) = queue.pop_front()` combines the emptiness check and destructuring pop into one ergonomic expression.

4. **Association list vs HashMap:** OCaml's `List.assoc` is idiomatic for small graphs but O(n) per lookup. Rust naturally uses `HashMap` for O(1) average. The `bfs_assoc` variant preserves the OCaml structure but at the cost of linear scans — a useful reminder that idioms have performance implications.

5. **Result accumulation:** OCaml prepends to a `ref` list (O(1) per step) then calls `List.rev` (O(n)) at the end. Rust pushes to a `Vec` directly in order, also O(1) amortized per step — both are linear overall, but Rust's version avoids the reversal step.

## When to Use Each Style

**Use idiomatic Rust (`HashMap`)** when: building production graph algorithms where O(1) lookup matters, working with large graphs, or when performance is a concern.

**Use assoc-list style** when: porting OCaml code directly for comparison, working with very small fixed graphs, or teaching the parallel between OCaml's `List.assoc` and Rust's iterator `find`.
