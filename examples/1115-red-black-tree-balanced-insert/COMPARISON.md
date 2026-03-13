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

### Rust (idiomatic — guard clauses for box-nested patterns)

```rust
pub fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    match (color, left, value, right) {
        // Case 1: red-red on left-left
        (Black, T(Red, a, x, b), y, d) if matches!(*a, T(Red, ..)) => {
            let T(Red, aa, xx, bb) = *a else { unreachable!() };
            T(Red, Box::new(T(Black, aa, xx, bb)), x, Box::new(T(Black, b, y, Box::new(d))))
        }
        // Case 2: red-red on left-right
        (Black, T(Red, a, x, b), y, d) if matches!(*b, T(Red, ..)) => {
            let T(Red, bb, yy, cc) = *b else { unreachable!() };
            T(Red, Box::new(T(Black, a, x, bb)), yy, Box::new(T(Black, cc, y, Box::new(d))))
        }
        // Case 3: red-red on right-left
        (Black, a, x, T(Red, b, y, d)) if matches!(*b, T(Red, ..)) => {
            let T(Red, bb, yy, cc) = *b else { unreachable!() };
            T(Red, Box::new(T(Black, Box::new(a), x, bb)), yy, Box::new(T(Black, cc, y, d)))
        }
        // Case 4: red-red on right-right
        (Black, a, x, T(Red, b, y, d)) if matches!(*d, T(Red, ..)) => {
            let T(Red, cc, zz, dd) = *d else { unreachable!() };
            T(Red, Box::new(T(Black, Box::new(a), x, b)), y, Box::new(T(Black, cc, zz, dd)))
        }
        (color, a, x, b) => T(color, Box::new(a), x, Box::new(b)),
    }
}
```

### Rust (insert — root always recolored black)

```rust
pub fn insert<T: Ord>(value: T, tree: RBTree<T>) -> RBTree<T> {
    fn ins<T: Ord>(value: T, tree: RBTree<T>) -> RBTree<T> {
        match tree {
            E => T(Red, Box::new(E), value, Box::new(E)),
            T(color, left, v, right) => {
                if value < v {
                    balance(color, ins(value, *left), v, *right)
                } else if value > v {
                    balance(color, *left, v, ins(value, *right))
                } else {
                    T(color, left, v, right) // duplicate: unchanged
                }
            }
        }
    }
    match ins(value, tree) {
        T(_, left, v, right) => T(Black, left, v, right),
        E => E,
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `'a rbtree` | `RBTree<T>` |
| Recursive node | `T of color * 'a rbtree * 'a * 'a rbtree` | `T(Color, Box<RBTree<T>>, T, Box<RBTree<T>>)` |
| Insert signature | `val insert : 'a -> 'a rbtree -> 'a rbtree` | `fn insert<T: Ord>(value: T, tree: RBTree<T>) -> RBTree<T>` |
| Membership | `val mem : 'a -> 'a rbtree -> bool` | `fn mem<T: Ord>(value: &T, tree: &RBTree<T>) -> bool` |
| In-order list | `val to_list : 'a rbtree -> 'a list` | `fn to_list<T: Clone>(tree: &RBTree<T>) -> Vec<T>` |

## Key Insights

1. **Box for recursion:** OCaml's heap-allocated variants recurse freely; Rust needs explicit `Box<RBTree<T>>` to give the type a known size at compile time.

2. **Nested pattern matching through Box:** OCaml's `balance` covers all 4 rotation cases in a single or-pattern match. In stable Rust, nested box patterns require the unstable `box` syntax, so we use guard clauses (`if matches!(...)`) plus `let ... else` destructuring to achieve the same effect.

3. **Ownership vs. GC sharing:** OCaml's GC allows unchanged subtrees to be shared between old and new trees transparently. Rust moves ownership — every node is either moved into the new tree or dropped. Functional persistence requires cloning if you need the old tree afterward.

4. **Root-recoloring:** Both languages recolor the root black after insertion to satisfy the black-root invariant. In OCaml this is a one-liner pattern match; in Rust the `match ins(value, tree)` at the end of `insert` does exactly the same thing.

5. **Ord bound vs. structural comparison:** OCaml compares values with polymorphic `<` by default. Rust requires the explicit `T: Ord` trait bound, which makes the constraint visible in the signature and enforced at compile time.

## When to Use Each Style

**Use idiomatic Rust (guard clauses):** Production code targeting stable Rust. Clear, readable, and works today without nightly features.

**Use recursive Rust style:** Educational contexts where showing the OCaml parallel explicitly matters — the recursive `ins` inner function mirrors OCaml's `let rec ins` directly.
