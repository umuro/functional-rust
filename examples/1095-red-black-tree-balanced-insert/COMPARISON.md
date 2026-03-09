# OCaml vs Rust: Red-Black Tree with Okasaki's Balanced Insert

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

### Rust (idiomatic — match guards for nested destructure)
```rust
fn balance<V: Ord>(color: Color, left: RBTree<V>, val: V, right: RBTree<V>) -> RBTree<V> {
    fn balanced<V>(a: RBTree<V>, x: V, b: RBTree<V>, y: V,
                   c: RBTree<V>, z: V, d: RBTree<V>) -> RBTree<V> {
        T(Red, Box::new(T(Black, Box::new(a), x, Box::new(b))),
               y,
               Box::new(T(Black, Box::new(c), z, Box::new(d))))
    }

    match (color, left, val, right) {
        (Black, T(Red, ll, y, c), z, d) if matches!(*ll, T(Red, ..)) => {
            let T(_, a, x, b) = *ll else { unreachable!() };
            balanced(*a, x, *b, y, *c, z, d)
        }
        (Black, T(Red, a, x, lr), z, d) if matches!(*lr, T(Red, ..)) => {
            let T(_, b, y, c) = *lr else { unreachable!() };
            balanced(*a, x, *b, y, *c, z, d)
        }
        (Black, a, x, T(Red, rl, z, d)) if matches!(*rl, T(Red, ..)) => {
            let T(_, b, y, c) = *rl else { unreachable!() };
            balanced(a, x, *b, y, *c, z, *d)
        }
        (Black, a, x, T(Red, b, y, rr)) if matches!(*rr, T(Red, ..)) => {
            let T(_, c, z, d) = *rr else { unreachable!() };
            balanced(a, x, *b, y, *c, z, *d)
        }
        (color, a, x, b) => T(color, Box::new(a), x, Box::new(b)),
    }
}
```

### Rust (insert — functional recursive with root repaint)
```rust
pub fn insert<V: Ord>(value: V, tree: RBTree<V>) -> RBTree<V> {
    fn ins<V: Ord>(x: V, tree: RBTree<V>) -> RBTree<V> {
        match tree {
            E => T(Red, Box::new(E), x, Box::new(E)),
            T(color, left, y, right) => {
                if x < y { balance(color, ins(x, *left), y, *right) }
                else if x > y { balance(color, *left, y, ins(x, *right)) }
                else { T(color, left, y, right) }
            }
        }
    }
    match ins(value, tree) {
        T(_, left, y, right) => T(Black, left, y, right),
        E => E,
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Color type | `type color = Red \| Black` | `enum Color { Red, Black }` |
| Tree type | `'a rbtree = E \| T of color * 'a rbtree * 'a * 'a rbtree` | `enum RBTree<T> { E, T(Color, Box<RBTree<T>>, T, Box<RBTree<T>>) }` |
| Balance | `val balance : color * 'a rbtree * 'a * 'a rbtree -> 'a rbtree` | `fn balance<V: Ord>(Color, RBTree<V>, V, RBTree<V>) -> RBTree<V>` |
| Insert | `val insert : 'a -> 'a rbtree -> 'a rbtree` | `fn insert<V: Ord>(V, RBTree<V>) -> RBTree<V>` |
| Membership | `val mem : 'a -> 'a rbtree -> bool` | `fn mem<V: Ord>(&V, &RBTree<V>) -> bool` |
| To list | `val to_list : 'a rbtree -> 'a list` | `fn to_sorted_vec<V: Clone>(&RBTree<V>) -> Vec<V>` |

## Key Insights

1. **Pattern depth gap:** OCaml's balance is 10 lines with or-patterns matching 3 levels deep. Rust stable cannot destructure through `Box` in patterns, requiring match guards + inner `let` destructuring — more verbose but equally correct.

2. **Ownership replaces GC:** In OCaml, old tree nodes become garbage when unreferenced. In Rust, `insert` takes ownership of the tree (`RBTree<V>`, not `&RBTree<V>`), moves subtrees into the new path, and the compiler drops unreachable nodes automatically — zero-cost persistence without a GC.

3. **Trait bounds make comparison explicit:** OCaml's polymorphic comparison (`<`, `>`, `=`) works on any type via structural equality. Rust requires `V: Ord`, making the comparison requirement visible in the type signature.

4. **Box allocation is explicit:** Every `Box::new(...)` in Rust corresponds to an implicit heap allocation in OCaml. The Rust code makes the allocation cost visible, while OCaml hides it behind the uniform representation.

5. **`unreachable!()` for exhaustiveness:** After a match guard confirms the inner structure (e.g., `matches!(*ll, T(Red, ..))`), the `let T(_, a, x, b) = *ll else { unreachable!() }` destructure is guaranteed to succeed. OCaml's or-patterns handle this natively without the two-step check.

## When to Use Each Style

**Use the match-guard Rust style when:** you need a purely functional balanced tree on stable Rust and want to stay close to Okasaki's original structure. The match guards clearly communicate the four rotation cases.

**Use the OCaml style when:** you want the most concise expression of the algorithm. OCaml's or-patterns and GC make Okasaki's balancing rule a near-literal transcription of the mathematical specification.
