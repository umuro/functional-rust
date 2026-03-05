# Comparison: Foldable for Binary Tree

## In-Order Fold

**OCaml:**
```ocaml
let rec fold_inorder f acc = function
  | Leaf -> acc
  | Node (l, v, r) ->
    let acc = fold_inorder f acc l in
    let acc = f acc v in
    fold_inorder f acc r
```

**Rust:**
```rust
fn fold_inorder<B>(&self, init: B, f: &mut impl FnMut(B, &T) -> B) -> B {
    match self {
        Tree::Leaf => init,
        Tree::Node(l, v, r) => {
            let acc = l.fold_inorder(init, f);
            let acc = f(acc, v);
            r.fold_inorder(acc, f)
        }
    }
}
```

## Derived Operations

**OCaml:**
```ocaml
let sum tree = fold_inorder (+) 0 tree
let all pred tree = fold_inorder (fun acc x -> acc && pred x) true tree
```

**Rust:**
```rust
fn sum(&self) -> i32 { self.fold_inorder(0, &mut |acc, x| acc + x) }
fn all(&self, pred: impl Fn(&i32) -> bool) -> bool {
    self.fold_inorder(true, &mut |acc, x| acc && pred(x))
}
```
