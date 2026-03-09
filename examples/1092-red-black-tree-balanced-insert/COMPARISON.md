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

### Rust (idiomatic — ownership-based balance)
```rust
fn balance(color: Color, left: Self, value: T, right: Self) -> Self {
    match (color, left, right) {
        (Black, Node(Red, ll, y, c), d) if matches!(*ll, Node(Red, _, _, _)) => {
            let Node(Red, a, x, b) = *ll else { unreachable!() };
            Node(Red, Box::new(Node(Black, a, x, b)), y,
                      Box::new(Node(Black, c, value, Box::new(d))))
        }
        // ... three more symmetric cases ...
        (c, left, right) => Node(c, Box::new(left), value, Box::new(right)),
    }
}
```

### Rust (insert — recursive with root recoloring)
```rust
pub fn insert(&self, x: T) -> Self {
    fn ins<T: Ord + Clone>(tree: &RBTree<T>, x: T) -> RBTree<T> {
        match tree {
            Empty => Node(Red, Box::new(Empty), x, Box::new(Empty)),
            Node(color, a, y, b) => match x.cmp(y) {
                Ordering::Less    => RBTree::balance(*color, ins(a, x), y.clone(), b.as_ref().clone()),
                Ordering::Greater => RBTree::balance(*color, a.as_ref().clone(), y.clone(), ins(b, x)),
                Ordering::Equal   => tree.clone(),
            },
        }
    }
    match ins(self, x) {
        Node(_, a, y, b) => Node(Black, a, y, b),
        Empty => Empty,
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Color type | `type color = Red \| Black` | `enum Color { Red, Black }` |
| Tree type | `'a rbtree = E \| T of color * 'a rbtree * 'a * 'a rbtree` | `enum RBTree<T> { Empty, Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>) }` |
| Balance | `val balance : color * 'a rbtree * 'a * 'a rbtree -> 'a rbtree` | `fn balance(Color, Self, T, Self) -> Self` |
| Insert | `val insert : 'a -> 'a rbtree -> 'a rbtree` | `fn insert(&self, T) -> Self` |
| Membership | `val mem : 'a -> 'a rbtree -> bool` | `fn contains(&self, &T) -> bool` |
| Traversal | `val to_list : 'a rbtree -> 'a list` | `fn to_sorted_vec(&self) -> Vec<&T>` |

## Key Insights

1. **Or-patterns vs guards:** OCaml's `balance` uses four or-patterns (`|`) mapping to one result. Rust lacks or-patterns on complex nested enums, so each case is a separate arm with a `matches!` guard to inspect grandchild color before destructuring.

2. **Ownership enables zero-copy rotation:** Rust's `balance` takes ownership of `left` and `right`, so when it restructures the tree it moves subtrees into the new shape without cloning. OCaml relies on the GC for this — allocation is implicit.

3. **Explicit heap allocation:** OCaml's `T (color, left, value, right)` allocates on the GC heap implicitly. Rust requires `Box::new(...)` for each recursive child, making the allocation cost visible.

4. **Persistence cost:** In OCaml, structural sharing is free (GC handles it). In Rust, the unchanged subtree on the non-insertion path must be `.clone()`d because the tree is not reference-counted. Using `Rc` instead of `Box` would enable true structural sharing at the cost of runtime overhead.

5. **Comparison semantics:** OCaml uses polymorphic `<` and `>` with structural equality. Rust uses the `Ord` trait bound and `x.cmp(y)` which is exhaustive — the compiler ensures all three orderings are handled.

## When to Use Each Style

**Use idiomatic Rust (ownership-based balance) when:** you want maximum performance from a persistent tree — ownership transfer avoids unnecessary reference counting or cloning during rotations.

**Use Rc-based Rust when:** you need true structural sharing (many versions of the tree alive simultaneously) and can tolerate the reference-counting overhead — closer to OCaml's GC-based sharing model.
