# OCaml vs Rust: Trie (Prefix Tree)

## Side-by-Side Comparison

### Type Definition

**OCaml:**
```ocaml
module CharMap = Map.Make(Char)

type trie = { is_end: bool; children: trie CharMap.t }
let empty = { is_end=false; children=CharMap.empty }
```

**Rust:**
```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}
```

### Insert Operation

**OCaml:**
```ocaml
let insert t word =
  let n = String.length word in
  let rec go node i =
    if i = n then { node with is_end=true }
    else
      let c = word.[i] in
      let child = try CharMap.find c node.children with Not_found -> empty in
      let new_child = go child (i+1) in
      { node with children=CharMap.add c new_child node.children }
  in go t 0
```

**Rust:**
```rust
fn insert(&mut self, word: &str) {
    let mut node = &mut self.root;
    for c in word.chars() {
        node = node.children.entry(c).or_default();
    }
    node.is_end = true;
}
```

### Search Operation

**OCaml:**
```ocaml
let search t word =
  let n = String.length word in
  let rec go node i =
    if i=n then node.is_end
    else match CharMap.find_opt word.[i] node.children with
    | None -> false
    | Some child -> go child (i+1)
  in go t 0
```

**Rust:**
```rust
fn search(&self, word: &str) -> bool {
    let mut node = &self.root;
    for c in word.chars() {
        match node.children.get(&c) {
            None => return false,
            Some(n) => node = n,
        }
    }
    node.is_end
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Map type | `CharMap` (functor) | `HashMap<char, _>` |
| Iteration style | Recursive with index | Iterative with `for` |
| Not found handling | Exception or `find_opt` | `Option` via `get` |
| Mutability | Functional (new nodes) | In-place mutation |
| Entry API | Manual insert | `entry().or_default()` |

## Memory Model

**OCaml:** Creates new nodes on each insert (functional style). The old trie is preserved (persistent data structure).

**Rust:** Mutates nodes in place. Previous state is lost unless explicitly cloned.

## Performance Characteristics

| Operation | OCaml | Rust |
|-----------|-------|------|
| Insert | O(m) - creates new path | O(m) - mutates in place |
| Search | O(m) | O(m) |
| Prefix search | O(m + k) | O(m + k) |
| Memory | More (path copying) | Less (in-place) |

Where m = word length, k = number of results
