# OCaml vs Rust: Topological Sort via Kahn's Algorithm

## Side-by-Side Code

### OCaml
```ocaml
module SMap = Map.Make(String)

let kahn_sort nodes edges =
  let in_deg = List.fold_left (fun m (_, b) ->
    SMap.update b (function None -> Some 1 | Some n -> Some (n+1)) m
  ) (List.fold_left (fun m n -> SMap.add n 0 m) SMap.empty nodes) edges in
  let queue = SMap.fold (fun k v acc -> if v = 0 then k :: acc else acc) in_deg [] in
  let rec go queue in_deg result =
    match queue with
    | [] -> List.rev result
    | node :: rest ->
      let out_edges = List.filter (fun (a, _) -> a = node) edges in
      let in_deg, new_queue = List.fold_left (fun (deg, q) (_, b) ->
        let d = SMap.find b deg - 1 in
        let deg = SMap.add b d deg in
        if d = 0 then (deg, b :: q) else (deg, q)
      ) (in_deg, rest) out_edges in
      go new_queue in_deg (node :: result)
  in go queue in_deg []
```

### Rust (idiomatic)
```rust
pub fn kahn_sort(nodes: &[&str], edges: &[(&str, &str)]) -> Option<Vec<String>> {
    let mut in_degree: HashMap<&str, usize> = nodes.iter().map(|&n| (n, 0)).collect();
    let mut adj: HashMap<&str, Vec<&str>> = nodes.iter().map(|&n| (n, Vec::new())).collect();

    for &(from, to) in edges {
        *in_degree.entry(to).or_insert(0) += 1;
        adj.entry(from).or_default().push(to);
    }

    let mut queue: VecDeque<&str> = in_degree.iter()
        .filter(|(_, &d)| d == 0).map(|(&n, _)| n).collect();
    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(node.to_string());
        // ... decrement neighbors, enqueue zeros
    }

    if result.len() == nodes.len() { Some(result) } else { None }
}
```

### Rust (DFS-based alternative)
```rust
fn dfs_visit(node: &str, adj: &HashMap<&str, Vec<&str>>,
             color: &mut HashMap<&str, Color>, result: &mut Vec<String>) -> bool {
    color.insert(node, Color::Gray);
    for &neighbor in adj.get(node).unwrap_or(&vec![]) {
        match color.get(neighbor) {
            Some(Color::Gray) => return false, // cycle!
            Some(Color::White) => if !dfs_visit(neighbor, adj, color, result) { return false; }
            _ => {}
        }
    }
    color.insert(node, Color::Black);
    result.push(node.to_string());
    true
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Map type | `SMap.t` (balanced BST) | `HashMap<&str, usize>` (hash table) |
| Return type | `string list` (always returns) | `Option<Vec<String>>` (None = cycle) |
| Edge type | `(string * string) list` | `&[(&str, &str)]` |
| Queue | `string list` | `VecDeque<&str>` |

## Key Insights

1. **OCaml's `Map.Make` functor vs Rust's `HashMap`** — OCaml uses a balanced tree (ordered, O(log n)), Rust uses a hash table (unordered, O(1)). For topological sort, order doesn't matter, so HashMap is faster.
2. **Immutable vs mutable state** — OCaml's `go` function threads `in_deg` immutably through recursion. Rust mutates `in_degree` in place. Both are correct; OCaml's is more functional, Rust's is more efficient.
3. **Cycle detection is elegant in both** — Kahn's detects cycles when `result.len() < nodes.len()`. DFS detects via "gray" (in-progress) nodes. Both O(V+E).
4. **OCaml's `List.fold_left` vs Rust's `for` loops** — OCaml builds the in-degree map with a fold. Rust uses an imperative loop with `entry()` API. The Rust functional variant shows both styles work.
5. **`Option<Vec<String>>` is more honest than `string list`** — OCaml's version silently returns a partial list on cycles. Rust's `Option` forces the caller to handle the error case.

## When to Use Each Style

**Use Kahn's algorithm when:** You need a BFS-based ordering, want to detect cycles naturally, or need to process nodes level by level (e.g., build systems, task scheduling).
**Use DFS-based sort when:** You're already doing graph traversal, want to detect back edges explicitly, or need post-order for other algorithms (e.g., SCC detection).
