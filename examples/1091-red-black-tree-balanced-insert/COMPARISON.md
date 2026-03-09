# OCaml vs Rust: Red-Black Tree — Balanced Insert (Iterator + Free Functions)

## Side-by-Side Code

### OCaml
```ocaml
let balance = function
  | Black, T (Red, T (Red, a, x, b), y, c), z, d
  | Black, T (Red, a, x, T (Red, b, y, c)), z, d
  | Black, a, x, T (Red, T (Red, b, y, c), z, d)
  | Black, a, x, T (Red, b, y, T (Red, c, z, d)) ->
    T (Red, T (Black, a, x, b), y, T (Black, c, z, d))
  | color, a, x, b -> T (color, a, x, b)

let insert x t =
  let rec ins = function
    | E -> T (Red, E, x, E)
    | T (color, a, y, b) ->
      if x < y then balance (color, ins a, y, b)
      else if x > y then balance (color, a, y, ins b)
      else T (color, a, y, b)
  in
  match ins t with T (_, a, y, b) -> T (Black, a, y, b) | E -> E

let rec to_list = function
  | E -> [] | T (_, a, v, b) -> to_list a @ [v] @ to_list b
```

### Rust (free functions — mirrors OCaml module style)
```rust
pub fn balance<T: Ord + Clone>(
    color: Color, left: RBTree<T>, value: T, right: RBTree<T>,
) -> RBTree<T> {
    match (color, left, right) {
        (Black, Node(Red, ll, y, c), d) if matches!(*ll, Node(Red, _, _, _)) => {
            let Node(Red, a, x, b) = *ll else { unreachable!() };
            Node(Red, Box::new(Node(Black, a, x, b)), y,
                 Box::new(Node(Black, c, value, Box::new(d))))
        }
        // ... three more cases with same output shape ...
        (c, left, right) => Node(c, Box::new(left), value, Box::new(right)),
    }
}

pub fn insert<T: Ord + Clone>(x: T, tree: &RBTree<T>) -> RBTree<T> {
    fn ins<T: Ord + Clone>(x: &T, tree: &RBTree<T>) -> RBTree<T> {
        match tree {
            Empty => Node(Red, Box::new(Empty), x.clone(), Box::new(Empty)),
            Node(color, a, y, b) => match x.cmp(y) {
                Ordering::Less => balance(*color, ins(x, a), y.clone(), b.as_ref().clone()),
                Ordering::Greater => balance(*color, a.as_ref().clone(), y.clone(), ins(x, b)),
                Ordering::Equal => tree.clone(),
            },
        }
    }
    match ins(&x, tree) {
        Node(_, a, y, b) => Node(Black, a, y, b),
        Empty => Empty,
    }
}
```

### Rust (stack-based iterator — replaces OCaml's eager `to_list`)
```rust
pub struct InOrder<'a, T> {
    stack: Vec<&'a RBTree<T>>,
}

impl<'a, T> InOrder<'a, T> {
    fn push_left_spine(&mut self, mut tree: &'a RBTree<T>) {
        while let Node(_, left, _, _) = tree {
            self.stack.push(tree);
            tree = left;
        }
    }
}

impl<'a, T> Iterator for InOrder<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let Node(_, _, value, right) = node {
            self.push_left_spine(right);
            Some(value)
        } else { None }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Balance | `color * 'a rbtree * 'a * 'a rbtree -> 'a rbtree` | `fn balance<T>(Color, RBTree<T>, T, RBTree<T>) -> RBTree<T>` |
| Insert | `'a -> 'a rbtree -> 'a rbtree` | `fn insert<T>(T, &RBTree<T>) -> RBTree<T>` |
| Membership | `'a -> 'a rbtree -> bool` | `fn mem<T>(&T, &RBTree<T>) -> bool` |
| Traversal | `'a rbtree -> 'a list` (eager) | `fn iter(&self) -> InOrder<'_, T>` (lazy) |
| Tree type | `'a rbtree` | `RBTree<T>` |
| Optional color | `color` (variant) | `Color` (enum) |

## Key Insights

1. **Free functions vs methods:** OCaml's module-level functions map naturally to Rust `pub fn` — the method wrappers are a thin ergonomic layer, not a requirement
2. **Ownership in balance:** Rust's `balance` takes ownership of subtrees so it can destructure `Box` and reassemble without deep cloning — OCaml's GC handles this implicitly
3. **Lazy vs eager traversal:** OCaml's `to_list` allocates O(n) with list append; Rust's `Iterator` trait enables O(log n) space traversal via an explicit stack
4. **Or-patterns:** OCaml's four `|`-separated patterns collapse to one body; Rust needs four separate arms because guards (`if matches!`) can't be shared across or-patterns
5. **Argument order:** OCaml conventionally puts the data argument last (`insert x t`) for pipelining; Rust free functions mirror this, while methods put `self` first

## When to Use Each Style

**Use free functions when:** you want a direct OCaml translation, are building a library where functions compose (e.g., `insert(3, &insert(2, &insert(1, &Empty)))`), or when the operation doesn't naturally belong to a single type.
**Use method wrappers when:** you want idiomatic Rust API ergonomics — `tree.insert(x)`, `tree.contains(&x)`, and `for v in &tree` via `IntoIterator`.
