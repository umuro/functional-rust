# OCaml vs Rust: Red-Black Tree with Okasaki's Functional Balancing

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

### Rust (idiomatic — method style)
```rust
impl<V: Ord> RBTree<V> {
    pub fn insert(self, value: V) -> Self {
        match Self::ins(value, self) {
            T(_, left, v, right) => T(Black, left, v, right),
            E => E,
        }
    }

    fn ins(x: V, tree: Self) -> Self {
        match tree {
            E => T(Red, Box::new(E), x, Box::new(E)),
            T(color, left, y, right) => {
                if x < y {
                    Self::balance(color, Self::ins(x, *left), y, *right)
                } else if x > y {
                    Self::balance(color, *left, y, Self::ins(x, *right))
                } else {
                    T(color, left, y, right)
                }
            }
        }
    }
}
```

### Rust (balance function with match guards)
```rust
fn balance(color: Color, left: Self, val: V, right: Self) -> Self {
    match (color, left, val, right) {
        // Case 1: left-left red-red
        (Black, T(Red, ll, y, c), z, d) if matches!(*ll, T(Red, ..)) => {
            let T(_, a, x, b) = *ll else { unreachable!() };
            balanced(*a, x, *b, y, *c, z, d)
        }
        // Case 2: left-right red-red
        (Black, T(Red, a, x, lr), z, d) if matches!(*lr, T(Red, ..)) => {
            let T(_, b, y, c) = *lr else { unreachable!() };
            balanced(*a, x, *b, y, *c, z, d)
        }
        // ... cases 3 and 4 symmetric on the right side
        (col, a, x, b) => T(col, Box::new(a), x, Box::new(b)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `'a rbtree` | `RBTree<V>` (enum with `Box` indirection) |
| Color type | `color` (sum type) | `Color` (enum) |
| Empty tree | `E` | `RBTree::E` |
| Node | `T of color * 'a rbtree * 'a * 'a rbtree` | `T(Color, Box<RBTree<V>>, V, Box<RBTree<V>>)` |
| Insert | `val insert : 'a -> 'a rbtree -> 'a rbtree` | `fn insert(self, value: V) -> Self` |
| Membership | `val mem : 'a -> 'a rbtree -> bool` | `fn mem(&self, value: &V) -> bool` |
| To list | `val to_list : 'a rbtree -> 'a list` | `fn to_sorted_vec(&self) -> Vec<V>` |

## Key Insights

1. **Or-patterns vs match guards:** OCaml collapses four balance cases into one match arm with `|`. Rust stable cannot match inside `Box`, so each case needs a separate arm with a `matches!` guard followed by `let-else` destructuring. This is the largest syntactic gap between the two translations.

2. **Ownership drives the API shape:** `insert` takes `self` by value because it deconstructs the tree. OCaml's GC allows structural sharing implicitly; in Rust, the caller gives up the old tree and receives a new one. This makes the functional "persistent" style explicit at the type level.

3. **`Box<T>` as the price of recursion:** OCaml's runtime handles recursive types transparently. Rust requires `Box` to break the infinite-size cycle. Every node construction wraps children in `Box::new(...)`, adding syntactic noise but making heap allocation visible.

4. **Method style vs free functions:** OCaml idiom is `insert x t` (free function, value first). Rust idiom is `tree.insert(x)` (method, receiver first). The `impl` block groups related operations, enabling chaining: `RBTree::new().insert(5).insert(3)`.

5. **Clone for traversal, not for structure:** The tree owns its values. In-order traversal (`to_sorted_vec`) must `clone()` each value to produce a `Vec` without consuming the tree. OCaml's GC allows returning shared references; Rust makes the copy cost explicit.

## When to Use Each Style

**Use idiomatic Rust (method style) when:** building a reusable data structure library — the `impl` block API is discoverable, chainable, and integrates with Rust's trait system (`Default`, `Debug`, `Iterator`).

**Use recursive/OCaml-style Rust when:** prototyping algorithms from functional programming literature or teaching — the structural recursion maps directly to Okasaki's pseudocode and makes the balance invariant visually obvious in the pattern matching.
