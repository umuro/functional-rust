# OCaml vs Rust: Box Deref Patterns

## Recursive Tree Type

### OCaml
```ocaml
(* GC handles allocation transparently *)
type tree = Leaf | Node of int * tree * tree

let rec depth = function
  | Leaf -> 0
  | Node (_, l, r) -> 1 + max (depth l) (depth r)
```

### Rust
```rust
// Box needed for recursive types
enum Tree {
    Leaf,
    Node { val: i32, left: Box<Tree>, right: Box<Tree> }
}

// Rust auto-derefs through Box in patterns
fn depth(t: &Tree) -> usize {
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, right, .. } => 
            1 + depth(left).max(depth(right)),
    }
}
```

## Key Insight: Auto-Deref

Rust automatically dereferences `Box<T>` to `T` in patterns:

```rust
// These are equivalent:
match t {
    Tree::Node { left, right, .. } => { /* left is &Box<Tree> */ }
}

match t {
    Tree::Node { left, right, .. } => { 
        depth(left)  // Works! Auto-deref to &Tree
    }
}
```

## Insert Pattern

### OCaml
```ocaml
let rec insert v = function
  | Leaf -> Node(v, Leaf, Leaf)
  | Node(x, l, r) when v < x -> Node(x, insert v l, r)
  | Node(x, l, r) when v > x -> Node(x, l, insert v r)
  | t -> t
```

### Rust
```rust
fn insert(t: Box<Tree>, v: i32) -> Box<Tree> {
    match *t {  // Deref to move out of Box
        Tree::Leaf => Tree::singleton(v),
        Tree::Node { val, left, right } => {
            if v < val { Tree::node(val, insert(left, v), right) }
            else if v > val { Tree::node(val, left, insert(right, v)) }
            else { Tree::node(val, left, right) }
        }
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Memory** | GC automatic | `Box<T>` explicit |
| **Pattern match** | Direct | Auto-deref through Box |
| **Ownership** | Implicit copy/share | Move or borrow |
| **Recursive type** | Direct | Requires indirection (Box) |
