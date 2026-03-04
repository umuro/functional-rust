# OCaml vs Rust: Tree Zipper

## Side-by-Side Code

### OCaml

```ocaml
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

type 'a crumb = Left of 'a * 'a tree | Right of 'a tree * 'a
type 'a zipper = { focus: 'a tree; trail: 'a crumb list }

let of_tree t = { focus = t; trail = [] }

let go_left z = match z.focus with
  | Leaf -> None
  | Node (l, v, r) -> Some { focus = l; trail = Left (v, r) :: z.trail }

let go_up z = match z.trail with
  | [] -> None
  | Left (v, r) :: rest -> Some { focus = Node (z.focus, v, r); trail = rest }
  | Right (l, v) :: rest -> Some { focus = Node (l, v, z.focus); trail = rest }

let set_value x z = match z.focus with
  | Leaf -> z
  | Node (l, _, r) -> { z with focus = Node (l, x, r) }

let rec to_tree z = match go_up z with
  | None    -> z.focus
  | Some z' -> to_tree z'
```

### Rust (idiomatic — iterative `to_tree`)

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

#[derive(Debug, Clone)]
pub enum Crumb<T> {
    Left(T, Tree<T>),
    Right(Tree<T>, T),
}

#[derive(Debug, Clone)]
pub struct Zipper<T> {
    pub focus: Tree<T>,
    pub trail: Vec<Crumb<T>>,
}

pub fn go_up<T>(mut z: Zipper<T>) -> Option<Zipper<T>> {
    match z.trail.pop() {
        None => None,
        Some(Crumb::Left(v, r)) => Some(Zipper {
            focus: Tree::node(z.focus, v, r),
            trail: z.trail,
        }),
        Some(Crumb::Right(l, v)) => Some(Zipper {
            focus: Tree::node(l, v, z.focus),
            trail: z.trail,
        }),
    }
}

// Idiomatic: loop + early return avoids the ownership issue
pub fn to_tree<T>(mut z: Zipper<T>) -> Tree<T> {
    loop {
        if z.trail.is_empty() {
            return z.focus;
        }
        z = go_up(z).expect("trail was non-empty");
    }
}
```

### Rust (functional/recursive)

```rust
// Mirrors the OCaml tail-recursive style
pub fn to_tree_recursive<T>(z: Zipper<T>) -> Tree<T> {
    if z.trail.is_empty() {
        return z.focus;
    }
    to_tree_recursive(go_up(z).expect("trail was non-empty"))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `'a tree = Leaf \| Node of 'a tree * 'a * 'a tree` | `enum Tree<T> { Leaf, Node(Box<Tree<T>>, T, Box<Tree<T>>) }` |
| Crumb type | `'a crumb = Left of 'a * 'a tree \| Right of 'a tree * 'a` | `enum Crumb<T> { Left(T, Tree<T>), Right(Tree<T>, T) }` |
| Zipper type | `'a zipper = { focus: 'a tree; trail: 'a crumb list }` | `struct Zipper<T> { focus: Tree<T>, trail: Vec<Crumb<T>> }` |
| `go_left` | `'a zipper -> 'a zipper option` | `fn go_left<T>(z: Zipper<T>) -> Option<Zipper<T>>` |
| `set_value` | `'a -> 'a zipper -> 'a zipper` | `fn set_value<T>(x: T, z: Zipper<T>) -> Zipper<T>` |
| `to_tree` | `'a zipper -> 'a tree` | `fn to_tree<T>(z: Zipper<T>) -> Tree<T>` |

## Key Insights

1. **Ownership makes state transitions explicit.** OCaml passes the zipper record by
   value implicitly; Rust's move semantics make the same transfer explicit — you
   hand over the old `Zipper<T>` and receive a new one. This prevents accidentally
   using a stale zipper after navigation.

2. **The `to_tree` one-liner breaks in Rust.** OCaml can write `match go_up z with
   None -> z.focus | Some z' -> to_tree z'` because the match does not consume
   `z` in a way that prevents reading `z.focus`. In Rust, `go_up(z)` moves `z`,
   so `z.focus` is inaccessible in the `None` arm. The fix is to check the trail
   *before* calling `go_up`, which is equally clear and efficient.

3. **`Vec` vs. linked list for the trail.** OCaml's natural crumb list is a linked
   list (cons-cells, O(1) prepend). Rust's `Vec` uses `push`/`pop` at the end —
   also O(1) amortised, but with better cache locality and no per-node allocation.

4. **`Box` for recursive ADTs.** OCaml trees have a uniform heap layout; Rust
   enums must have a known size, so recursive variants need `Box<Tree<T>>` to
   break the infinite-size cycle. This is the canonical Rust pattern for recursive
   data structures.

5. **`{ z with focus = … }` vs. struct update.** OCaml's record update syntax
   (`{ z with focus = Node ... }`) has a direct Rust analogue in struct update
   syntax (`Zipper { focus: …, ..z }`), but since `z` is consumed by move we
   instead destructure it manually — which is equally concise.

## When to Use Each Style

**Use idiomatic Rust (`to_tree` loop) when:** you want to avoid potential stack
overflow on deep trees, since Rust does not guarantee tail-call optimisation.

**Use recursive Rust (`to_tree_recursive`) when:** the tree depth is bounded and
you want the code to read as closely as possible to the OCaml original for
educational or porting purposes.
