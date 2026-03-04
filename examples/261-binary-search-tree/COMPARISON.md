# OCaml vs Rust: Binary Search Tree — Insert and Search

## Side-by-Side Code

### OCaml
```ocaml
type 'a bst = Leaf | Node of 'a bst * 'a * 'a bst

let rec insert x = function
  | Leaf -> Node (Leaf, x, Leaf)
  | Node (l, v, r) ->
    if x < v then Node (insert x l, v, r)
    else if x > v then Node (l, v, insert x r)
    else Node (l, v, r)

let rec mem x = function
  | Leaf -> false
  | Node (l, v, r) ->
    if x = v then true
    else if x < v then mem x l
    else mem x r
```

### Rust (idiomatic)
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Bst<T> {
    Leaf,
    Node(Box<Bst<T>>, T, Box<Bst<T>>),
}

impl<T: Ord + Clone> Bst<T> {
    pub fn insert(&self, x: T) -> Self {
        match self {
            Bst::Leaf => Bst::Node(Box::new(Bst::Leaf), x, Box::new(Bst::Leaf)),
            Bst::Node(left, val, right) => match x.cmp(val) {
                Ordering::Less => Bst::Node(Box::new(left.insert(x)), val.clone(), right.clone()),
                Ordering::Greater => Bst::Node(left.clone(), val.clone(), Box::new(right.insert(x))),
                Ordering::Equal => self.clone(),
            },
        }
    }

    pub fn mem(&self, x: &T) -> bool {
        match self {
            Bst::Leaf => false,
            Bst::Node(left, val, right) => match x.cmp(val) {
                Ordering::Equal => true,
                Ordering::Less => left.mem(x),
                Ordering::Greater => right.mem(x),
            },
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `'a bst` | `Bst<T>` |
| Insert | `val insert : 'a -> 'a bst -> 'a bst` | `fn insert(&self, x: T) -> Self` |
| Membership | `val mem : 'a -> 'a bst -> bool` | `fn mem(&self, x: &T) -> bool` |
| In-order | `val inorder : 'a bst -> 'a list` | `fn inorder(&self) -> Vec<T>` |
| Empty tree | `Leaf` | `Bst::Leaf` |

## Key Insights

1. **Recursive types need Box:** OCaml's GC handles recursive types transparently; Rust requires `Box` to make the type size known at compile time
2. **Trait bounds make costs explicit:** OCaml's polymorphic comparison is free but can fail at runtime for incomparable types; Rust's `T: Ord` guarantees comparability at compile time
3. **Clone reveals persistence cost:** When inserting, unchanged subtrees must be cloned. In OCaml, the GC shares references automatically; in Rust, `.clone()` makes this cost visible
4. **Pattern matching is nearly identical:** Both languages use exhaustive pattern matching on algebraic types — the syntax differs but the logic is the same
5. **Method vs function style:** OCaml uses standalone recursive functions; Rust idiomatically uses `impl` blocks with `&self` methods

## When to Use Each Style

**Use persistent BST when:** you need undo/redo, versioned data, or concurrent readers without locks  
**Use mutable BST when:** performance is critical and you don't need to preserve old versions
