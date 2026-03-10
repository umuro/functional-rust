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

### Rust (idiomatic — implements `FromIterator`, integrates with `.collect()`)

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Color { Red, Black }

#[derive(Debug, Clone, PartialEq)]
pub enum RbTree<T> {
    E,
    T(Color, Box<RbTree<T>>, T, Box<RbTree<T>>),
}

impl<T: Ord + Clone> RbTree<T> {
    pub fn insert(&self, x: T) -> Self {
        match self.ins(&x) {
            RbTree::T(_, a, y, b) => RbTree::T(Color::Black, a, y, b),
            RbTree::E => RbTree::E,
        }
    }
}

impl<T: Ord + Clone> FromIterator<T> for RbTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(RbTree::empty(), |t, x| t.insert(x))
    }
}
```

### Rust (Okasaki balance — four nested `if let` cases)

```rust
fn balance<T: Clone>(color: Color, left: RbTree<T>, val: T, right: RbTree<T>) -> RbTree<T> {
    use Color::{Black, Red};

    if color == Black {
        // Case 1: left-left red violation
        if let RbTree::T(Red, ref ll, ref lv, ref lr) = left {
            if let RbTree::T(Red, ref a, ref x, ref b) = **ll {
                return RbTree::T(Red,
                    Box::new(RbTree::T(Black, a.clone(), x.clone(), b.clone())),
                    lv.clone(),
                    Box::new(RbTree::T(Black, lr.clone(), val, Box::new(right))));
            }
            // Case 2: left-right red violation
            if let RbTree::T(Red, ref b, ref y, ref c) = **lr {
                return RbTree::T(Red,
                    Box::new(RbTree::T(Black, ll.clone(), lv.clone(), b.clone())),
                    y.clone(),
                    Box::new(RbTree::T(Black, c.clone(), val, Box::new(right))));
            }
        }
        // Case 3: right-left red violation
        if let RbTree::T(Red, ref rl, ref rv, ref rr) = right {
            if let RbTree::T(Red, ref b, ref y, ref c) = **rl {
                return RbTree::T(Red,
                    Box::new(RbTree::T(Black, Box::new(left), val, b.clone())),
                    y.clone(),
                    Box::new(RbTree::T(Black, c.clone(), rv.clone(), rr.clone())));
            }
            // Case 4: right-right red violation
            if let RbTree::T(Red, ref c, ref z, ref d) = **rr {
                return RbTree::T(Red,
                    Box::new(RbTree::T(Black, Box::new(left), val, rl.clone())),
                    rv.clone(),
                    Box::new(RbTree::T(Black, c.clone(), z.clone(), d.clone())));
            }
        }
    }
    RbTree::T(color, Box::new(left), val, Box::new(right))
}
```

## Type Signatures

| Concept              | OCaml                               | Rust                                      |
|----------------------|-------------------------------------|-------------------------------------------|
| Tree type            | `'a rbtree`                         | `RbTree<T>`                               |
| Empty node           | `E`                                 | `RbTree::E`                               |
| Internal node        | `T of color * 'a rbtree * 'a * 'a rbtree` | `T(Color, Box<RbTree<T>>, T, Box<RbTree<T>>)` |
| Insert signature     | `val insert : 'a -> 'a rbtree -> 'a rbtree` | `fn insert(&self, x: T) -> Self`   |
| Balance signature    | `(color * 'a rbtree * 'a * 'a rbtree) -> 'a rbtree` | `fn balance<T: Clone>(Color, RbTree<T>, T, RbTree<T>) -> RbTree<T>` |
| Membership           | `val mem : 'a -> 'a rbtree -> bool` | `fn member(&self, x: &T) -> bool`         |
| Collection building  | `List.fold_left (fun t x -> insert x t) E xs` | `iter.collect::<RbTree<_>>()`    |

## Key Insights

1. **Box for recursion:** OCaml's GC manages recursive types transparently.
   Rust requires `Box<RbTree<T>>` to give the recursive type a known size on
   the stack. Every child node is heap-allocated.

2. **`ref` bindings preserve ownership:** OCaml patterns always bind by value
   (GC semantics). In Rust, matching on an *owned* enum would move its fields.
   `ref ll` borrows instead, keeping `left` intact for the default fallthrough.
   NLL (non-lexical lifetimes) ensures those borrows end before the owned
   `left` is moved in cases 3 and 4.

3. **Box prevents deep match patterns:** OCaml's single `function` arm matches
   three levels deep in one expression. Rust stable cannot destructure through
   `Box` in a match pattern, so the four cases become nested `if let` chains —
   same logic, more syntactic scaffolding.

4. **`Clone` as the price of persistence:** Okasaki's trees share subtrees
   structurally. In Rust, every node along the path from root to the insert
   point is rebuilt (cloned), while unchanged subtrees are shared via `Box`
   (reference counting would also work but isn't needed here).

5. **`FromIterator` vs `fold_left`:** OCaml uses `List.fold_left` ad hoc.
   Rust's trait system lets `RbTree` implement `FromIterator<T>`, making it a
   first-class collection that works with `.collect()` and iterator adapters —
   a clean integration with the standard library.

## When to Use Each Style

**Use idiomatic Rust (`FromIterator` + `insert`)** when building a tree from a
data source — ranges, vecs, file lines. The `.collect()` call is zero-overhead
compared to a manual fold and expresses intent clearly.

**Use the recursive `ins` + `balance` style** when you want to see the direct
structural parallel to Okasaki's original formulation — useful for teaching,
porting other purely functional algorithms, or verifying correctness against
the textbook.
