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

### Rust (idiomatic — Okasaki balance with pattern matching)
```rust
fn balance(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> Self {
    match (color, &left, &right) {
        (Black, Node(Red, ref ll, _, _), _)
            if matches!(ll.as_ref(), Node(Red, _, _, _)) =>
        {
            if let Node(Red, ll, y, c) = left {
                if let Node(Red, a, x, b) = *ll {
                    return Node(Red,
                        Box::new(Node(Black, a, x, b)),
                        y,
                        Box::new(Node(Black, c, value, Box::new(right))));
                }
            }
            unreachable!()
        }
        // ... (3 more rotation cases follow the same pattern)
        _ => Node(color, Box::new(left), value, Box::new(right)),
    }
}
```

### Rust (insert — recursive with root recoloring)
```rust
pub fn insert(&self, x: T) -> Self {
    fn ins<T: Ord + Clone>(tree: &RBTree<T>, x: T) -> RBTree<T> {
        match tree {
            Empty => Node(Red, Box::new(Empty), x, Box::new(Empty)),
            Node(color, a, y, b) => {
                if x < *y {
                    RBTree::balance(*color, ins(a, x), y.clone(), b.as_ref().clone())
                } else if x > *y {
                    RBTree::balance(*color, a.as_ref().clone(), y.clone(), ins(b, x))
                } else {
                    tree.clone()
                }
            }
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
| Balance | `val balance : color * 'a rbtree * 'a * 'a rbtree -> 'a rbtree` | `fn balance(Color, RBTree<T>, T, RBTree<T>) -> Self` |
| Insert | `val insert : 'a -> 'a rbtree -> 'a rbtree` | `fn insert(&self, T) -> Self` |
| Membership | `val mem : 'a -> 'a rbtree -> bool` | `fn contains(&self, &T) -> bool` |

## Key Insights

1. **Or-patterns vs guard clauses:** OCaml's `balance` merges all four rotation cases into a single match arm using or-patterns (`|`). Rust can't do this when different bindings are needed per case, so each case becomes a separate arm with a `matches!()` guard. This is more verbose but equally correct.

2. **Two-phase matching for ownership:** Rust's `balance` first matches on references (`&left`, `&right`) to decide which case applies, then destructures by value to actually move subtrees. This borrow-then-move pattern is necessary because Rust won't let you both inspect and consume a value in the same pattern.

3. **Explicit heap allocation:** Every `T(...)` in OCaml implicitly allocates on the GC heap. In Rust, each `Node(...)` requires explicit `Box::new()` for child pointers. This makes the allocation cost visible but also controllable.

4. **Clone for persistence:** OCaml's GC allows sharing subtrees between the old and new versions of the tree for free. Rust must `clone()` the unchanged subtree when building the balanced result, making the cost of persistence explicit. The `Clone` bound on `T` is the price of immutable data structures without GC.

5. **Inner function for recursion:** OCaml uses `let rec ins = function` inside `insert` to close over `x`. Rust uses a nested `fn ins<T>()` that takes `x` as a parameter, since inner functions can't capture from the enclosing scope.

## When to Use Each Style

**Use the pattern-matching balance approach when:** implementing any self-balancing tree (red-black, AVL, 2-3 trees). The exhaustive match ensures every rotation case is handled, and the compiler verifies completeness.

**Use the recursive insert with root recoloring when:** building persistent collections where immutability is a requirement. The functional approach — returning a new tree rather than mutating — naturally supports undo, versioning, and concurrent reads without locks.
