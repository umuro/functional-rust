# OCaml vs Rust: Red-Black Tree — Balanced Insert

## Side-by-Side Code

### OCaml
```ocaml
type color = Red | Black
type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree

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
```

### Rust (idiomatic)
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color { Red, Black }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<T> {
    Empty,
    Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

impl<T: Ord + Clone> RBTree<T> {
    pub fn insert(self, value: T) -> Self {
        fn ins<T: Ord + Clone>(tree: RBTree<T>, value: &T) -> RBTree<T> {
            match tree {
                Empty => Node(Red, Box::new(Empty), value.clone(), Box::new(Empty)),
                Node(color, left, v, right) => {
                    if *value < v { balance(color, ins(*left, value), v, *right) }
                    else if *value > v { balance(color, *left, v, ins(*right, value)) }
                    else { Node(color, left, v, right) }
                }
            }
        }
        match ins(self, &value) {
            Node(_, left, v, right) => Node(Black, left, v, right),
            Empty => Empty,
        }
    }
}
```

### Rust (balance function — functional pattern matching)
```rust
fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    match (color, left, value, right) {
        // Case 1: Left-Left red-red violation
        (Black, Node(Red, ll, lv, lr), v, r)
            if matches!(ll.as_ref(), Node(Red, _, _, _)) => { /* rotate */ }
        // Case 2: Left-Right red-red violation
        (Black, Node(Red, a, x, lr), v, r)
            if matches!(lr.as_ref(), Node(Red, _, _, _)) => { /* rotate */ }
        // Case 3: Right-Left red-red violation
        (Black, l, v, Node(Red, rl, rv, rr))
            if matches!(rl.as_ref(), Node(Red, _, _, _)) => { /* rotate */ }
        // Case 4: Right-Right red-red violation
        (Black, l, v, Node(Red, rl, rv, rr))
            if matches!(rr.as_ref(), Node(Red, _, _, _)) => { /* rotate */ }
        // No violation
        (color, left, value, right) => Node(color, Box::new(left), value, Box::new(right)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Color type | `type color = Red \| Black` | `enum Color { Red, Black }` |
| Tree type | `'a rbtree = E \| T of color * 'a rbtree * 'a * 'a rbtree` | `enum RBTree<T> { Empty, Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>) }` |
| Balance signature | `val balance : color * 'a rbtree * 'a * 'a rbtree -> 'a rbtree` | `fn balance<T>(Color, RBTree<T>, T, RBTree<T>) -> RBTree<T>` |
| Insert signature | `val insert : 'a -> 'a rbtree -> 'a rbtree` | `fn insert(self, T) -> Self` (method on `RBTree<T>`) |
| Membership | `val mem : 'a -> 'a rbtree -> bool` | `fn mem(&self, &T) -> bool` |
| To list | `val to_list : 'a rbtree -> 'a list` | `fn to_sorted_vec(&self) -> Vec<T>` |

## Key Insights

1. **Or-patterns vs match guards:** OCaml's `balance` collapses four cases with shared bindings into two lines using `|`. Rust requires separate match arms with `if matches!(...)` guards because it cannot bind different variables across or-patterns. This is the most visible translation friction in this example.

2. **Box everywhere:** Every recursive child in Rust must be `Box`-wrapped for heap allocation. OCaml's GC makes this invisible. The trade-off: Rust gives you deterministic memory layout and no GC pauses.

3. **Ownership through rotations:** OCaml's `balance` can freely reference subtrees `a, b, c, d` because the GC keeps them alive. Rust's `balance` must destructure `Box` values with `*ll`, consume them, and re-box the results. Each rotation is an explicit ownership transfer.

4. **Clone bound:** OCaml's polymorphic `'a` works with any type. Rust needs `T: Ord + Clone` — `Ord` for comparisons and `Clone` for the `ins` function to create new leaf nodes with `value.clone()`. This makes the requirements on `T` explicit in the type signature.

5. **Functional purity preserved:** Despite Rust's ownership system, the `insert` method is purely functional — it consumes `self` and returns a new tree. The caller's original tree is gone after insertion, which is exactly OCaml's semantics but enforced at compile time rather than by convention.

## When to Use Each Style

**Use idiomatic Rust when:** You need a balanced BST in production code — wrap the functional core behind a clean API with `FromIterator`, `Default`, and method syntax for ergonomic use.

**Use the functional/recursive style when:** You're studying Okasaki's algorithms or teaching functional programming concepts — the direct translation from OCaml makes the correspondence between pattern-match cases and tree rotations explicit and verifiable.
