# OCaml vs Rust: DFS

Both use recursion naturally. The main difference is state handling:

## Rust
```rust
fn dfs(graph: &HashMap<i32, Vec<i32>>, v: i32, visited: &mut HashSet<i32>)
```

## OCaml
```ocaml
let rec dfs graph visited v = 
  if IntSet.mem v visited then visited
  else ...
```

OCaml passes immutable set through return value.
