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

### Rust (functional/recursive — close to OCaml structure)

```rust
pub fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    match (color, left, value, right) {
        // Cases 1 & 2: left child is Red
        (Black, T(Red, ll, lv, lr), v, r) => match *ll {
            T(Red, a, x, b) => T(Red,
                Box::new(T(Black, a, x, b)), lv,
                Box::new(T(Black, lr, v, Box::new(r)))),
            ll => match *lr {
                T(Red, b, y, c) => T(Red,
                    Box::new(T(Black, Box::new(ll), lv, b)), y,
                    Box::new(T(Black, c, v, Box::new(r)))),
                lr => T(Black,
                    Box::new(T(Red, Box::new(ll), lv, Box::new(lr))),
                    v, Box::new(r)),
            },
        },
        // Cases 3 & 4: right child is Red
        (Black, l, v, T(Red, rl, rv, rr)) => match *rl {
            T(Red, b, y, c) => T(Red,
                Box::new(T(Black, Box::new(l), v, b)), y,
                Box::new(T(Black, c, rv, rr))),
            rl => match *rr {
                T(Red, c, z, d) => T(Red,
                    Box::new(T(Black, Box::new(l), v, Box::new(rl))), rv,
                    Box::new(T(Black, c, z, d))),
                rr => T(Black, Box::new(l), v,
                    Box::new(T(Red, Box::new(rl), rv, Box::new(rr)))),
            },
        },
        (color, l, v, r) => T(color, Box::new(l), v, Box::new(r)),
    }
}
```

### Rust (idiomatic — build from iterator)

```rust
pub fn from_iter<T: Ord>(iter: impl Iterator<Item = T>) -> RBTree<T> {
    iter.fold(E, |tree, v| insert(v, tree))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type 'a rbtree = E \| T of color * 'a rbtree * 'a * 'a rbtree` | `enum RBTree<T> { E, T(Color, Box<RBTree<T>>, T, Box<RBTree<T>>) }` |
| Balance | `val balance : color * 'a rbtree * 'a * 'a rbtree -> 'a rbtree` | `fn balance<T>(Color, RBTree<T>, T, RBTree<T>) -> RBTree<T>` |
| Insert | `val insert : 'a -> 'a rbtree -> 'a rbtree` | `fn insert<T: Ord>(T, RBTree<T>) -> RBTree<T>` |
| Membership | `val mem : 'a -> 'a rbtree -> bool` | `fn mem<T: Ord>(&T, &RBTree<T>) -> bool` |
| Build from list | `List.fold_left (fun t x -> insert x t) E xs` | `iter.fold(E, \|tree, v\| insert(v, tree))` |

## Key Insights

1. **Box breaks deep pattern matching.** OCaml's `rbtree` children are stored inline (value type), so OCaml can pattern match as deep as needed in a single arm. Rust requires `Box<RBTree<T>>` for heap indirection (the recursive type would have infinite size otherwise), and Rust's stable pattern matching cannot see through `Box`. The solution: match the outer level to bind the `Box`, then use `match *box` in a nested arm to deref and inspect.

2. **Four cases become two outer + two inner.** OCaml's `balance` uses four `|`-joined patterns at the same level. Rust splits this into two outer arms (left-red vs. right-red) each containing a nested match for the grandchild. This is semantically identical — just required by the Box-indirection in the type.

3. **Ownership tracks the functional semantics.** Both OCaml and Rust's `balance` and `insert` consume their inputs and produce new trees. In OCaml this is implicit (GC handles sharing); in Rust `balance` takes ownership (no `Clone` needed) and the type system enforces no aliasing — making the "persistent tree" property explicit.

4. **`fold` is the universal list-to-structure builder.** `List.fold_left (fun t x -> insert x t) E xs` in OCaml is `iter.fold(E, |tree, v| insert(v, tree))` in Rust. Both are left folds accumulating a tree from an initial empty value.

5. **Root-forced-black is the invariant restoration step.** After `ins` (which may return a red root), `insert` wraps it with `T(Black, ...)`. This one-line post-processing step is all that is needed to restore the red-black invariant at the root — a beautiful property of Okasaki's formulation.

## When to Use Each Style

**Use this functional RB tree when:** demonstrating persistent/immutable data structures, comparing OCaml and Rust approaches, or teaching structural pattern matching on recursive algebraic types.

**Use `std::collections::BTreeMap` / `BTreeSet` when:** you need a production-grade sorted map/set in Rust — the standard library's implementation is heavily optimized, handles all edge cases, and provides a rich iterator API.
