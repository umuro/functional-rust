# Comparison: Traversable for Binary Tree

## Traverse with Option

**OCaml:**
```ocaml
let rec traverse_option f = function
  | Leaf -> Some Leaf
  | Node (l, v, r) ->
    match traverse_option f l with
    | None -> None
    | Some l' -> match f v with
      | None -> None
      | Some v' -> match traverse_option f r with
        | None -> None
        | Some r' -> Some (Node (l', v', r'))
```

**Rust (? operator makes it clean!):**
```rust
fn traverse_option<U>(&self, f: &impl Fn(&T) -> Option<U>) -> Option<Tree<U>> {
    match self {
        Tree::Leaf => Some(Tree::Leaf),
        Tree::Node(l, v, r) => {
            let l2 = l.traverse_option(f)?;  // ? replaces nested match
            let v2 = f(v)?;
            let r2 = r.traverse_option(f)?;
            Some(Tree::node(l2, v2, r2))
        }
    }
}
```

## Pure Map (No Effect)

**OCaml:**
```ocaml
let rec map f = function
  | Leaf -> Leaf
  | Node (l, v, r) -> Node (map f l, f v, map f r)
```

**Rust:**
```rust
fn map<U>(&self, f: &impl Fn(&T) -> U) -> Tree<U> {
    match self {
        Tree::Leaf => Tree::Leaf,
        Tree::Node(l, v, r) => Tree::node(l.map(f), f(v), r.map(f)),
    }
}
```
