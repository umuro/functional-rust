# Comparison: Trie — OCaml vs Rust

## Core Insight

OCaml's trie is naturally functional: `Map.Make(Char)` gives an immutable sorted map for children, and every insert returns a new trie (sharing unchanged subtrees via structural sharing). Rust's idiomatic trie is mutable — `HashMap<char, Trie>` with `entry().or_default()` for elegant insertion. The functional Rust version requires explicit `.clone()` where OCaml shares structure implicitly.

## OCaml

```ocaml
module CMap = Map.Make(Char)
type trie = { is_word: bool; children: trie CMap.t }

let rec insert_go word i node =
  if i = String.length word then { node with is_word = true }
  else
    let c = word.[i] in
    let child = try CMap.find c node.children with Not_found -> empty in
    { node with children = CMap.add c (insert_go word (i+1) child) node.children }
```

## Rust — Mutable HashMap

```rust
pub fn insert(&mut self, word: &str) {
    let mut node = self;
    for c in word.chars() {
        node = node.children.entry(c).or_default();
    }
    node.is_word = true;
}
```

## Rust — Functional (BTreeMap)

```rust
pub fn insert(&self, word: &str) -> Self {
    // Clone children, recursively insert — mirrors OCaml but explicit cloning
}
```

## Comparison Table

| Aspect | OCaml | Rust (mutable) | Rust (functional) |
|--------|-------|----------------|-------------------|
| Children map | `Map.Make(Char)` (tree) | `HashMap<char, Trie>` | `BTreeMap<char, Trie>` |
| Insert style | Returns new trie | Mutates in place | Returns new trie (clone) |
| Structural sharing | Automatic (GC) | N/A | Manual clone |
| Missing child | `Not_found` exception | `entry().or_default()` | `.unwrap_or_else(empty)` |
| Lookup | `CMap.find_opt` | `.get(&c)` | `.get(&c)` |
| Performance | O(k log 26) per op | O(k) amortized | O(k * clone_cost) |

## Learner Notes

- **`entry().or_default()`**: Rust's most elegant map pattern — creates the child node if missing, returns mutable ref
- **Structural sharing**: OCaml's GC enables cheap "copy" of unchanged subtrees; Rust must clone explicitly
- **Array trie**: `[Option<Box<Trie>>; 26]` gives O(1) child lookup vs O(log n) for tree maps — great for ASCII
- **`Default` trait**: Implementing `Default` for `Trie` enables `or_default()` — Rust's way to say "empty node"
- **Recursive types**: Both languages handle `trie` containing `trie` children — Rust needs no `Box` here because `HashMap` heap-allocates values
