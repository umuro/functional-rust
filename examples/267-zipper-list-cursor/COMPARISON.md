# OCaml vs Rust: Zipper List Cursor

## Side-by-Side Code

### OCaml

```ocaml
type 'a zipper = { left: 'a list; focus: 'a; right: 'a list }

let of_list = function
  | [] -> failwith "empty"
  | h :: t -> { left = []; focus = h; right = t }

let go_right z = match z.right with
  | [] -> None
  | h :: t -> Some { left = z.focus :: z.left; focus = h; right = t }

let go_left z = match z.left with
  | [] -> None
  | h :: t -> Some { left = t; focus = h; right = z.focus :: z.right }

let update f z = { z with focus = f z.focus }
let to_list z = List.rev z.left @ [z.focus] @ z.right
```

### Rust (idiomatic — method API)

```rust
impl<T> Zipper<T> {
    pub fn from_slice(slice: &[T]) -> Option<Self> where T: Clone {
        let mut iter = slice.iter().cloned();
        let focus = iter.next()?;
        Some(Zipper { left: vec![], focus, right: iter.collect() })
    }

    pub fn move_right(self) -> Option<Self> {
        let mut right = self.right;
        if right.is_empty() { return None; }
        let new_focus = right.remove(0);
        let mut left = self.left;
        left.insert(0, self.focus);
        Some(Zipper { left, focus: new_focus, right })
    }

    pub fn map_focus<F: FnOnce(T) -> T>(self, f: F) -> Self {
        Zipper { focus: f(self.focus), ..self }
    }

    pub fn into_vec(self) -> Vec<T> {
        let mut v: Vec<T> = self.left.into_iter().rev().collect();
        v.push(self.focus);
        v.extend(self.right);
        v
    }
}
```

### Rust (functional/free-function — mirrors OCaml style)

```rust
pub fn go_right<T>(z: Zipper<T>) -> Option<Zipper<T>> {
    let mut right = z.right;
    if right.is_empty() { return None; }
    let new_focus = right.remove(0);
    let mut left = z.left;
    left.insert(0, z.focus);
    Some(Zipper { left, focus: new_focus, right })
}

pub fn go_left<T>(z: Zipper<T>) -> Option<Zipper<T>> {
    let mut left = z.left;
    if left.is_empty() { return None; }
    let new_focus = left.remove(0);
    let mut right = z.right;
    right.insert(0, z.focus);
    Some(Zipper { left, focus: new_focus, right })
}

pub fn update<T, F: FnOnce(T) -> T>(z: Zipper<T>, f: F) -> Zipper<T> {
    Zipper { focus: f(z.focus), ..z }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Zipper type | `type 'a zipper = { left: 'a list; focus: 'a; right: 'a list }` | `struct Zipper<T> { left: Vec<T>, focus: T, right: Vec<T> }` |
| Construction | `of_list : 'a list -> 'a zipper` | `of_slice<T: Clone>(&[T]) -> Option<Zipper<T>>` |
| Navigation | `go_right : 'a zipper -> 'a zipper option` | `go_right<T>(Zipper<T>) -> Option<Zipper<T>>` |
| Update | `update : ('a -> 'a) -> 'a zipper -> 'a zipper` | `update<T, F: FnOnce(T)->T>(Zipper<T>, F) -> Zipper<T>` |
| Flatten | `to_list : 'a zipper -> 'a list` | `to_vec<T>(Zipper<T>) -> Vec<T>` |
| Optional value | `'a option` | `Option<T>` |

## Key Insights

1. **Record update syntax is nearly identical.** OCaml's `{ z with focus = f z.focus }` maps directly to Rust's struct update `Zipper { focus: f(z.focus), ..z }`. This is one of the closest syntactic parallels between the two languages.

2. **Ownership makes sharing explicit.** In OCaml the GC transparently shares the spine of the old zipper when constructing the new one. In Rust, `go_right` consumes the old `Zipper<T>` by value; the compiler ensures there is only ever one live zipper, enforcing the "one focused location" invariant at the type level.

3. **`Vec<T>` replaces persistent linked lists.** OCaml's `'a list` is a persistent singly-linked list; prepending is O(1). Rust's `Vec<T>` is a growable array; `insert(0, …)` is O(n). For a pedagogical example this is fine, but a production zipper would use `VecDeque` or a real linked structure for O(1) ends.

4. **`FnOnce` captures OCaml's higher-order style.** The `update` function takes `F: FnOnce(T) -> T`, which is the Rust equivalent of OCaml's `('a -> 'a)` argument — a single-use, potentially capturing closure.

5. **Dual API.** Rust idiom allows providing both a free-function API (`go_right(z)`) that feels like OCaml and a method API (`z.move_right()`) that feels like Rust. Both share the same implementation; the methods are thin wrappers.

## When to Use Each Style

**Use idiomatic Rust (method API) when:** integrating the zipper into a larger system where call sites prefer `z.move_right()?.map_focus(|x| x + 1).into_vec()` chaining.

**Use recursive/functional Rust (free functions) when:** you want the OCaml-to-Rust mapping to be maximally transparent for teaching or porting an existing functional algorithm step by step.
