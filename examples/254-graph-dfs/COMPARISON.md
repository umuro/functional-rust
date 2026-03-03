# OCaml vs Rust: Graph DFS

## Side-by-Side Code

### OCaml

```ocaml
module SS = Set.Make(String)

let dfs graph start =
  let rec go visited node =
    if SS.mem node visited then (visited, [])
    else
      let visited = SS.add node visited in
      let neighbors = try List.assoc node graph with Not_found -> [] in
      let visited, paths = List.fold_left (fun (vis, acc) n ->
        let vis, path = go vis n in
        (vis, acc @ path)
      ) (visited, []) neighbors in
      (visited, node :: paths)
  in
  snd (go SS.empty start)
```

### Rust (idiomatic — iterative stack)

```rust
pub fn dfs<'a>(graph: &'a HashMap<&str, Vec<&str>>, start: &'a str) -> Vec<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut stack: Vec<&str> = vec![start];
    let mut result: Vec<&str> = Vec::new();

    while let Some(node) = stack.pop() {
        if !visited.insert(node) {
            continue;
        }
        result.push(node);
        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors.iter().rev() {
                if !visited.contains(neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }
    result
}
```

### Rust (functional/recursive — mirrors OCaml)

```rust
pub fn dfs_recursive<'a>(graph: &[(&'a str, Vec<&'a str>)], start: &'a str) -> Vec<&'a str> {
    fn go<'a>(
        graph: &[(&'a str, Vec<&'a str>)],
        visited: &mut HashSet<&'a str>,
        node: &'a str,
    ) -> Vec<&'a str> {
        if !visited.insert(node) {
            return vec![];
        }
        let neighbors = graph
            .iter()
            .find(|(n, _)| *n == node)
            .map(|(_, ns)| ns.as_slice())
            .unwrap_or(&[]);

        let mut path = vec![node];
        for &neighbor in neighbors {
            path.extend(go(graph, visited, neighbor));
        }
        path
    }
    let mut visited = HashSet::new();
    go(graph, &mut visited, start)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| DFS function | `val dfs : (string * string list) list -> string -> string list` | `fn dfs<'a>(graph: &'a HashMap<&str, Vec<&str>>, start: &'a str) -> Vec<&'a str>` |
| Graph type | `(string * string list) list` | `HashMap<&str, Vec<&str>>` |
| Visited set | `SS.t` (balanced BST, O(log n)) | `HashSet<&str>` (hash table, O(1)) |
| Node type | `string` (owned, GC-managed) | `&str` (borrowed, lifetime-tracked) |
| Neighbor lookup | `List.assoc node graph` (O(n)) | `graph.get(node)` (O(1)) |

## Key Insights

1. **Pure vs. mutable state:** OCaml threads `visited` as an immutable value through every return — `go` returns `(new_visited, path)`. Rust uses `&mut HashSet` to achieve the same result with a single allocation and no copying. The observable behaviour is identical; only the mechanism differs.

2. **`insert` as a membership test:** `HashSet::insert` returns `bool` — `false` means already present. This lets Rust combine "is visited?" and "mark visited" into one atomic call, eliminating the OCaml pattern of `SS.mem` followed by `SS.add`.

3. **Stack vs. recursion:** OCaml's natural style is deep recursion with tail-call optimisation available when the compiler detects it. Rust prefers an explicit stack for DFS because Rust does not guarantee TCO, and deep recursion risks a stack overflow on large graphs.

4. **Graph representation trade-off:** OCaml's association list (`(string * string list) list`) is idiomatic for small graphs and mirrors the lambda-calculus tradition. Rust's `HashMap` gives O(1) lookups; the assoc-list variant (`&[(&str, Vec<&str>)]`) is provided for a direct structural parallel to the OCaml code.

5. **Lifetime annotations:** The `'a` lifetime ties the output `Vec<&'a str>` to the graph's borrowed string data — the compiler proves at compile time that returned node references never outlive the graph. OCaml's GC makes this invisible; Rust makes it explicit.

## When to Use Each Style

**Use idiomatic Rust (iterative stack) when:** your graph can be large or deep — iterative DFS will not overflow the call stack and avoids the overhead of allocating intermediate `Vec`s for every recursive call.

**Use recursive Rust (functional style) when:** you are translating OCaml or Haskell DFS code directly and want to preserve the structural correspondence for review or teaching, and the graph depth is bounded.
