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

### Rust (idiomatic — sequential if-let balance)
```rust
fn balance<T: Clone>(color: Color, left: RbTree<T>, z: T, right: RbTree<T>) -> RbTree<T> {
    use Color::{Black, Red};
    use RbTree::T;
    if color != Black {
        return RbTree::T(color, Box::new(left), z, Box::new(right));
    }
    // Case 1: left-left red grandchild
    if let T(Red, ll, y_val, c_val) = &left {
        if let T(Red, a, x_val, b) = ll.as_ref() {
            return RbTree::T(Red,
                Box::new(T(Black, a.clone(), x_val.clone(), b.clone())),
                y_val.clone(),
                Box::new(T(Black, c_val.clone(), z, Box::new(right))));
        }
        // Case 2: left-right red grandchild
        if let T(Red, b, y2_val, c2_val) = c_val.as_ref() {
            return RbTree::T(Red,
                Box::new(T(Black, ll.clone(), y_val.clone(), b.clone())),
                y2_val.clone(),
                Box::new(T(Black, c2_val.clone(), z, Box::new(right))));
        }
    }
    // Cases 3 & 4 mirror the above for the right subtree …
    RbTree::T(color, Box::new(left), z, Box::new(right))
}
```

### Rust (functional insert — path-copying)
```rust
fn ins(&self, x: &T) -> Self {
    match self {
        RbTree::E => RbTree::T(Color::Red, Box::new(RbTree::E), x.clone(), Box::new(RbTree::E)),
        RbTree::T(color, a, y, b) => {
            if x < y      { balance(color.clone(), a.ins(x), y.clone(), *b.clone()) }
            else if x > y { balance(color.clone(), *a.clone(), y.clone(), b.ins(x)) }
            else           { self.clone() }
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `'a rbtree` | `RbTree<T>` |
| Empty node | `E` | `RbTree::E` |
| Internal node | `T of color * 'a rbtree * 'a * 'a rbtree` | `T(Color, Box<RbTree<T>>, T, Box<RbTree<T>>)` |
| Insert | `val insert : 'a -> 'a rbtree -> 'a rbtree` | `fn insert(&self, x: T) -> Self` where `T: Ord + Clone` |
| Balance | `val balance : color * 'a rbtree * 'a * 'a rbtree -> 'a rbtree` | `fn balance<T: Clone>(Color, RbTree<T>, T, RbTree<T>) -> RbTree<T>` |

## Key Insights

1. **Persistent vs owned:** OCaml nodes are shared via GC; Rust uses path-copying (`Clone`) to create a new spine on every insert while leaving the original unchanged.
2. **Pattern matching depth:** OCaml's `match` can destructure nested tuples of variants in a single arm. Rust requires sequential `if let` chains to avoid consuming the value before checking alternatives.
3. **Reference ergonomics trap:** Using `ref` bindings inside a pattern that already matches through a `&` reference produces double-references (`&&T`). `.clone()` on `&&T` gives `&&T`, not `T`. The fix: match via `&left` / `ll.as_ref()` without `ref`.
4. **Box as indirection:** Rust requires explicit `Box` for recursive enum variants; OCaml's lists and trees are heap-allocated implicitly. `Box::new` and `*b.clone()` make the allocation visible.
5. **Root invariant:** Both implementations paint the root black after insertion by destructuring and rebuilding the outermost node — a minimal, elegant invariant-restoration step.

## When to Use Each Style

**Use idiomatic Rust (sequential if-let):** When translating Okasaki-style functional algorithms — the `if let` chain mirrors the four symmetric cases without needing a macro or helper enum.

**Use recursive Rust (path-copying insert):** Whenever you need a persistent data structure; the recursive `ins` pattern cleanly separates "structural recursion" from "rebalancing logic".
