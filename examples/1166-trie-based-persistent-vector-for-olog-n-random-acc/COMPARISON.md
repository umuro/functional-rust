# OCaml vs Rust: Trie-based Persistent Vector

## Side-by-Side Code

### OCaml
```ocaml
type 'a pvec = Nil | One of 'a | Two of 'a pvec * 'a pvec

let rec size = function
  | Nil -> 0 | One _ -> 1
  | Two (l, r) -> size l + size r

let rec get i = function
  | One x -> if i = 0 then x else failwith "index"
  | Two (l, r) ->
    let ls = size l in
    if i < ls then get i l else get (i - ls) r
  | Nil -> failwith "empty"

let rec set i v = function
  | One _ -> if i = 0 then One v else failwith "index"
  | Two (l, r) ->
    let ls = size l in
    if i < ls then Two (set i v l, r)
    else Two (l, set (i - ls) v r)
  | Nil -> failwith "empty"
```

### Rust (idiomatic — Rc for structural sharing)
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum PVec<T> {
    Nil,
    One(T),
    Two(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T: Clone> PVec<T> {
    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVec::Two(l, r) => {
                let ls = l.size();
                if i < ls {
                    // Clone only the path; share the unchanged subtree via Rc::clone.
                    l.set(i, v).map(|new_l| PVec::Two(Rc::new(new_l), Rc::clone(r)))
                } else {
                    r.set(i - ls, v).map(|new_r| PVec::Two(Rc::clone(l), Rc::new(new_r)))
                }
            }
            // ...
        }
    }
}
```

### Rust (functional/recursive — Box, no sharing)
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum PVecBox<T> {
    Nil,
    One(T),
    Two(Box<PVecBox<T>>, Box<PVecBox<T>>),
}
// set() must deep-clone unchanged subtrees — no structural sharing.
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type 'a pvec = Nil \| One of 'a \| Two of 'a pvec * 'a pvec` | `enum PVec<T> { Nil, One(T), Two(Rc<PVec<T>>, Rc<PVec<T>>) }` |
| Structural sharing | GC (automatic) | `Rc<T>` (reference counting) |
| set | returns new tree, shares old branches | returns `Option<PVec<T>>`, shares via `Rc::clone` |
| get | raises exception on out-of-bounds | returns `Option<&T>` |
| Recursive type | automatic | requires `Rc<PVec<T>>` or `Box<PVecBox<T>>` |

## Key Insights

1. **Memory management**: OCaml's GC automatically keeps shared nodes alive as long as any version references them; Rust uses `Rc<T>` (reference counting) for the same effect — but with explicit `Rc::clone` and deterministic drop.
2. **Structural sharing**: When `set` updates index `i`, only the O(log n) nodes on the path from root to leaf are re-allocated; unchanged subtrees are shared via `Rc::clone` (just increments a counter).
3. **Persistence invariant enforced at compile time**: Rust's type system prevents mutating a shared `Rc` value — you cannot obtain a mutable reference to a shared node, so the persistence invariant cannot be violated even accidentally.
4. **Box vs. Rc**: `Box<T>` gives unique ownership (no sharing, must deep-clone on update); `Rc<T>` allows shared ownership (structural sharing, O(log n) updates). OCaml uses the same node for both via GC.
5. **Error handling**: OCaml raises `failwith "index"` for out-of-bounds; Rust returns `Option<&T>` or `Option<Self>`, making the caller handle the error at compile time.

## When to Use Each Style

**Use `Rc`-based PVec when:** you need genuine structural sharing for time-travel debugging, undo/redo stacks, or persistent data structure semantics in production.
**Use `Box`-based PVecBox when:** teaching the OCaml parallel without the complexity of reference counting, or in cases where versions are never shared simultaneously.
