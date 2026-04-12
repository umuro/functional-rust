# OCaml vs Rust: Persistent Vector — Functional Array

## Side-by-Side Code

### OCaml
```ocaml
type 'a pvec = Nil | One of 'a | Two of 'a pvec * 'a pvec

let rec set i v = function
  | One _ -> if i = 0 then One v else failwith "index"
  | Two (l, r) ->
    let ls = size l in
    if i < ls then Two (set i v l, r)
    else Two (l, set (i - ls) v r)
  | Nil -> failwith "empty"
```

### Rust (idiomatic — with `Rc` structural sharing)
```rust
#[derive(Debug, Clone)]
pub enum PVec<T> {
    Nil,
    One(T),
    Two(Rc<PVec<T>>, Rc<PVec<T>>),
}

pub fn set(&self, i: usize, v: T) -> Option<Self> {
    match self {
        PVec::Nil => None,
        PVec::One(_) => (i == 0).then(|| PVec::One(v)),
        PVec::Two(l, r) => {
            let ls = l.size();
            if i < ls {
                l.set(i, v)
                    .map(|new_l| PVec::Two(Rc::new(new_l), Rc::clone(r)))
            } else {
                r.set(i - ls, v)
                    .map(|new_r| PVec::Two(Rc::clone(l), Rc::new(new_r)))
            }
        }
    }
}
```

### Rust (recursive — mirrors OCaml with `Box`)
```rust
#[derive(Debug, Clone)]
pub enum PVecRec<T> {
    Nil,
    One(T),
    Two(Box<PVecRec<T>>, Box<PVecRec<T>>),
}

pub fn set(&self, i: usize, v: T) -> Option<Self> {
    match self {
        PVecRec::Nil => None,
        PVecRec::One(_) => (i == 0).then(|| PVecRec::One(v)),
        PVecRec::Two(l, r) => {
            let ls = l.size();
            if i < ls {
                l.set(i, v)
                    .map(|new_l| PVecRec::Two(Box::new(new_l), r.clone()))
            } else {
                r.set(i - ls, v)
                    .map(|new_r| PVecRec::Two(l.clone(), Box::new(new_r)))
            }
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| ADT definition | `type 'a pvec = Nil \| One of 'a \| Two of 'a pvec * 'a pvec` | `enum PVec<T> { Nil, One(T), Two(Rc<PVec<T>>, Rc<PVec<T>>) }` |
| Get signature | `val get : int -> 'a pvec -> 'a` (raises) | `fn get(&self, i: usize) -> Option<&T>` |
| Set signature | `val set : int -> 'a -> 'a pvec -> 'a pvec` (raises) | `fn set(&self, i: usize, v: T) -> Option<Self>` |
| Shared subtree | implicit (GC) | `Rc::clone(r)` — O(1) ref-count bump |
| Error signal | `failwith "index"` (exception) | `None` (caller handles) |

## Key Insights

1. **Structural sharing requires explicit smart pointers in Rust.** OCaml's GC automatically shares any unchanged subtree; Rust needs `Rc<T>` (reference counting) to achieve the same. Without `Rc`, `Box` forces a deep clone of every unchanged subtree on each `set`, making the "persistence" expensive.

2. **`Rc::clone` is the idiomatic spelling.** Writing `Rc::clone(r)` instead of `r.clone()` makes it obvious that only the reference count is bumped — no heap allocation occurs. This is a critical performance distinction from cloning the contained value.

3. **`Option<T>` over panics makes callers composable.** OCaml's `failwith` tears down the call stack; Rust's `Option` lets callers chain with `?`, `.and_then()`, or `.map()`. This is more idiomatic and avoids hidden control flow.

4. **Slice patterns unify OCaml list and array matching.** OCaml matches on linked-list constructors; Rust matches on `&[T]` slice patterns. Both look like `[] | [x] | _` but Rust's operate on contiguous memory with O(1) length, making the `mid = len / 2` split more efficient.

5. **`T: Clone` bound is explicit and minimal.** OCaml's `'a` is unrestricted; Rust requires `T: Clone` only where cloning actually occurs (e.g., `from_slice` clones each element from the input slice). The `set` method works without cloning `T` itself — only the `Rc` wrapper is incremented.

## When to Use Each Style

**Use idiomatic Rust (`Rc`) when:** you want true persistence with O(log n) updates and structural sharing between versions — e.g., implementing undo/redo, version history, or any functional data structure where multiple versions coexist.

**Use recursive Rust (`Box`) when:** you only need immutable tree traversal without multiple live versions, and simplicity of ownership is more important than sharing. The tree is functionally equivalent but each `set` clones the entire path.
