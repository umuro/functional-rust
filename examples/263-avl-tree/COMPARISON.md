# OCaml vs Rust: AVL Tree — Self-Balancing BST

## Side-by-Side Code

### OCaml
```ocaml
type 'a avl = Empty | Node of 'a avl * 'a * 'a avl * int

let height = function Empty -> 0 | Node (_, _, _, h) -> h
let node l v r = Node (l, v, r, 1 + max (height l) (height r))

let rotate_right = function
  | Node (Node (ll, lv, lr, _), v, r, _) -> node (node ll lv lr) v r
  | t -> t

let rec insert x = function
  | Empty -> node Empty x Empty
  | Node (l, v, r, _) ->
    if x < v then rebalance (node (insert x l) v r)
    else if x > v then rebalance (node l v (insert x r))
    else node l v r
```

### Rust (idiomatic with named fields)
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Avl<T> {
    Empty,
    Node { left: Box<Avl<T>>, value: T, right: Box<Avl<T>>, height: i32 },
}

impl<T: Ord + Clone> Avl<T> {
    fn rotate_right(self) -> Self {
        match self {
            Avl::Node { left, value, right, .. } => match *left {
                Avl::Node { left: ll, value: lv, right: lr, .. } =>
                    Self::node(Self::node(*ll, lv, *lr), value, *right),
                _ => Self::node(*left, value, *right),
            },
            other => other,
        }
    }

    pub fn insert(&self, x: T) -> Self {
        match self {
            Avl::Empty => Self::node(Avl::Empty, x, Avl::Empty),
            Avl::Node { left, value, right, .. } => match x.cmp(value) {
                Ordering::Less => Self::node(left.insert(x), value.clone(), (**right).clone()).rebalance(),
                Ordering::Greater => Self::node((**left).clone(), value.clone(), right.insert(x)).rebalance(),
                Ordering::Equal => self.clone(),
            },
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `'a avl` | `Avl<T>` |
| Height | `'a avl -> int` | `fn height(&self) -> i32` |
| Insert | `'a -> 'a avl -> 'a avl` | `fn insert(&self, x: T) -> Self` |
| Rotate | `'a avl -> 'a avl` | `fn rotate_right(self) -> Self` |
| Balance factor | `'a avl -> int` | `fn balance_factor(&self) -> i32` |

## Key Insights

1. **Rotation = ownership transfer:** OCaml creates new nodes from pattern-matched pieces; Rust moves ownership through `self` by value, making the restructuring zero-copy where possible
2. **Two-level pattern matching:** OCaml matches `Node(Node(ll,lv,lr,_), v, r, _)` in one arm; Rust needs nested `match` since it can't destructure two levels of `Box` at once
3. **Named vs positional fields:** OCaml's `Node of 'a avl * 'a * 'a avl * int` uses positional access; Rust's named struct fields (`left`, `value`, `right`, `height`) are self-documenting
4. **Balance invariant verification:** Both languages can express `is_balanced` as a recursive check; Rust's type system doesn't encode the invariant statically (both rely on runtime checks)
5. **Clone cost transparency:** Rust's `.clone()` on unchanged subtrees makes the persistence cost visible; OCaml shares references silently via GC

## When to Use Each Style

**Use AVL tree when:** you need guaranteed O(log n) operations and care about worst-case performance  
**Use standard BTreeMap when:** you want a production-ready balanced tree — Rust's stdlib provides one
