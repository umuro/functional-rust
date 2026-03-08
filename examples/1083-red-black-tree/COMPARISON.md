# OCaml vs Rust: Red-Black Tree (Okasaki's Functional Balancing)

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

### Rust (idiomatic — peek-then-destructure)
```rust
fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    if color != Black {
        return RBTree::node(color, left, value, right);
    }
    // Case 1: left-left — peek at shape, then destructure by move
    if left.is_red_node() {
        if let Node { left: ref ll, .. } = left {
            if ll.is_red_node() {
                if let Node { left: ll_box, value: y, right: c, .. } = left {
                    if let Node { left: a, value: x, right: b, .. } = *ll_box {
                        return RBTree::node(Red,
                            RBTree::node(Black, *a, x, *b), y,
                            RBTree::node(Black, *c, value, right));
                    }
                }
            }
        }
    }
    // ... Cases 2, 3, 4 follow the same peek-then-move pattern
    RBTree::node(Black, left, value, right)
}
```

### Rust (insert — functional path copying)
```rust
pub fn insert(self, x: T) -> Self where T: Clone {
    fn ins<T: Ord + Clone>(tree: RBTree<T>, x: &T) -> RBTree<T> {
        match tree {
            Empty => RBTree::node(Red, Empty, x.clone(), Empty),
            Node { color, left, value, right } => match x.cmp(&value) {
                Ordering::Less => balance(color, ins(*left, x), value, *right),
                Ordering::Greater => balance(color, *left, value, ins(*right, x)),
                Ordering::Equal => RBTree::node(color, *left, value, *right),
            },
        }
    }
    match ins(self, &x) {
        Node { left, value, right, .. } => Node { color: Black, left, value, right },
        Empty => Empty,
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Color type | `type color = Red \| Black` | `enum Color { Red, Black }` |
| Tree type | `'a rbtree = E \| T of color * 'a rbtree * 'a * 'a rbtree` | `enum RBTree<T> { Empty, Node { color, left: Box<RBTree<T>>, value: T, right: Box<RBTree<T>> } }` |
| Insert | `val insert : 'a -> 'a rbtree -> 'a rbtree` | `fn insert(self, x: T) -> Self where T: Ord + Clone` |
| Membership | `val mem : 'a -> 'a rbtree -> bool` | `fn mem(&self, x: &T) -> bool` |
| Balance | `val balance : color * 'a rbtree * 'a * 'a rbtree -> 'a rbtree` | `fn balance(color, left, value, right) -> RBTree<T>` |

## Key Insights

1. **Or-patterns vs sequential checks:** OCaml's `balance` uses four or-patterns in a single match arm — arguably the most elegant expression of Okasaki's algorithm. Rust lacks this capability for nested owned destructuring, requiring sequential peek-then-move checks for each case.

2. **Move semantics as path copying:** Rust's ownership system naturally implements the functional "path copying" technique. When `insert` takes `self` by value and reconstructs the spine, moved subtrees are shared for free — no reference counting or GC needed.

3. **Box for recursive types:** OCaml's recursive types are heap-allocated automatically by the runtime. Rust requires explicit `Box<T>` to break the infinite-size recursion, making the indirection visible in the type signature.

4. **Comparison traits:** OCaml uses structural equality (`=`) and comparison (`<`, `>`) via polymorphic operators. Rust requires explicit `T: Ord` trait bounds, making the ordering requirement part of the type contract.

5. **Clone for insert:** OCaml's `insert x t` captures `x` by reference implicitly (GC keeps it alive). Rust's `insert` requires `T: Clone` because the inner `ins` function may need to clone `x` at each recursive call when creating new leaf nodes.

## When to Use Each Style

**Use idiomatic Rust (peek-then-destructure) when:** building production-quality balanced trees where you need compile-time ownership guarantees and zero-cost abstraction — the verbose balance function compiles to the same efficient code as hand-written rotations.

**Use OCaml/functional style when:** prototyping tree algorithms or writing educational code where the elegance of pattern matching makes the algorithm's structure immediately visible — Okasaki's four-case balance is a canonical example of how pattern matching shines.
