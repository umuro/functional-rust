# OCaml vs Rust: Red-Black Tree — Okasaki's Functional Balanced Insert

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

### Rust (idiomatic — pattern match with ownership two-phase)
```rust
fn balance(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> Self {
    match (color, &left, &right) {
        // Case 1: Left-left red violation
        (Black, Node(Red, ref ll, _, _), _)
            if matches!(ll.as_ref(), Node(Red, _, _, _)) =>
        {
            if let Node(Red, ll, y, c) = left {
                if let Node(Red, a, x, b) = *ll {
                    return Node(Red,
                        Box::new(Node(Black, a, x, b)), y,
                        Box::new(Node(Black, c, value, Box::new(right))));
                }
            }
            unreachable!()
        }
        // ... Cases 2-4 follow the same two-phase pattern
        _ => Node(color, Box::new(left), value, Box::new(right)),
    }
}
```

### Rust (recursive insert — mirrors OCaml `ins`)
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
    // Paint root black — invariant 2
    match ins(self, x) {
        Node(_, a, y, b) => Node(Black, a, y, b),
        Empty => Empty,
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `'a rbtree` | `RBTree<T>` |
| Node constructor | `T of color * 'a rbtree * 'a * 'a rbtree` | `Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>)` |
| Empty tree | `E` | `Empty` |
| Color type | `color = Red \| Black` | `enum Color { Red, Black }` |
| Balance signature | `color * 'a rbtree * 'a * 'a rbtree -> 'a rbtree` | `fn balance(Color, RBTree<T>, T, RBTree<T>) -> Self` |
| Insert signature | `'a -> 'a rbtree -> 'a rbtree` | `fn insert(&self, T) -> Self` |
| Membership | `'a -> 'a rbtree -> bool` | `fn contains(&self, &T) -> bool` |

## Key Insights

1. **Okasaki's balance translates almost directly** — both languages use pattern matching on four symmetric violation cases, producing the same rotated subtree shape
2. **Ownership forces a two-phase match** — OCaml destructures and moves in one pattern; Rust must first match on references (to check the shape) then destructure by value (to move children)
3. **`Box` replaces GC for recursive types** — OCaml allocates tree nodes on the GC heap implicitly; Rust needs explicit `Box::new()` at every child position
4. **Persistent trees cost more in Rust** — OCaml shares unchanged subtrees for free; Rust must `.clone()` every node along the insertion path because it cannot share ownership without `Rc`
5. **Type constraints are explicit** — OCaml's parametric polymorphism `'a` has no constraints; Rust requires `T: Ord + Clone` to compare elements and copy the spine during insertion

## When to Use Each Style

**Use idiomatic Rust when:** building production balanced trees — the `BTreeMap`/`BTreeSet` in std is typically better, but this pattern is valuable for understanding persistent data structures and when you need immutable snapshots

**Use recursive Rust when:** teaching functional programming concepts in Rust, or when the algorithm's correctness is easier to verify in recursive form (Okasaki's balance proof maps directly to the pattern match structure)
