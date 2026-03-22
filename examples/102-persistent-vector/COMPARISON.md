# OCaml vs Rust: Persistent Vector

## Side-by-Side Code

### OCaml

```ocaml
type 'a pvec = Nil | One of 'a | Two of 'a pvec * 'a pvec

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
    if i < ls then Two (set i v l, r)        (* right branch shared by GC *)
    else Two (l, set (i - ls) v r)           (* left branch shared by GC *)
  | Nil -> failwith "empty"
```

### Rust (idiomatic — explicit sharing with `Rc`)

```rust
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum PVec<T> {
    Nil,
    Leaf(T),
    Branch(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T: Clone> PVec<T> {
    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVec::Nil => None,
            PVec::Leaf(_) => (i == 0).then(|| PVec::Leaf(v)),
            PVec::Branch(l, r) => {
                let ls = l.size();
                if i < ls {
                    // Rc::clone(r) = pointer bump, not deep copy
                    l.set(i, v)
                        .map(|new_l| PVec::Branch(Rc::new(new_l), Rc::clone(r)))
                } else {
                    r.set(i - ls, v)
                        .map(|new_r| PVec::Branch(Rc::clone(l), Rc::new(new_r)))
                }
            }
        }
    }
}
```

### Rust (Box-based — mirrors OCaml pattern, no structural sharing)

```rust
#[derive(Debug, Clone)]
pub enum PVecBox<T> {
    Nil,
    Leaf(T),
    Branch(Box<PVecBox<T>>, Box<PVecBox<T>>),
}

impl<T: Clone> PVecBox<T> {
    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVecBox::Nil => None,
            PVecBox::Leaf(_) => (i == 0).then(|| PVecBox::Leaf(v)),
            PVecBox::Branch(l, r) => {
                let ls = l.size();
                if i < ls {
                    // r.clone() = deep copy of entire right subtree
                    l.set(i, v)
                        .map(|new_l| PVecBox::Branch(Box::new(new_l), r.clone()))
                } else {
                    r.set(i - ls, v)
                        .map(|new_r| PVecBox::Branch(l.clone(), Box::new(new_r)))
                }
            }
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| ADT definition | `type 'a pvec = Nil \| One of 'a \| Two of 'a pvec * 'a pvec` | `enum PVec<T> { Nil, Leaf(T), Branch(Rc<PVec<T>>, Rc<PVec<T>>) }` |
| Get signature | `val get : int -> 'a pvec -> 'a` | `fn get(&self, i: usize) -> Option<&T>` |
| Set signature | `val set : int -> 'a -> 'a pvec -> 'a pvec` | `fn set(&self, i: usize, v: T) -> Option<Self>` |
| Shared node | implicit (GC alias) | `Rc<PVec<T>>` |
| Error mode | `failwith "index"` (exception) | `None` (Option) |

## Key Insights

1. **GC vs explicit sharing:** OCaml's GC handles structural sharing automatically — the runtime recognises that the unchanged branch in `Two (set i v l, r)` is aliased to the original `r`. In Rust, `Rc<T>` makes this explicit: `Rc::clone(r)` is a reference-count bump, not a value copy.

2. **`Box<T>` vs `Rc<T>` for trees:** `Box<T>` expresses unique ownership — updating a `Box`-tree requires deep-cloning all untouched subtrees. `Rc<T>` expresses shared ownership — unchanged subtrees can be aliased safely across versions. Choosing `Rc` is what makes this data structure genuinely persistent.

3. **Error handling philosophy:** OCaml raises exceptions for out-of-bounds access (`failwith`). Rust makes errors part of the type: `Option<T>` forces the caller to handle the absent case at compile time, not at runtime.

4. **Clone bound propagation:** OCaml's `'a pvec` is polymorphic with no constraints — `set` works for any element type. Rust's `set` and `from_slice` require `T: Clone` explicitly because they copy values into new nodes. Operations that only traverse the tree (`get`, `size`) need no bound.

5. **Recursive types:** Both OCaml and Rust require indirection for recursive types (OCaml uses implicit heap boxing; Rust requires explicit `Box<T>` or `Rc<T>`). Without the pointer indirection, the compiler cannot determine the size of `PVec<T>` on the stack.

## When to Use Each Style

**Use `Rc`-based `PVec` when:** you need genuine persistence — multiple versions of a data structure coexisting, e.g. implementing undo history, a persistent map, or a CRDT.

**Use `Box`-based `PVecBox` when:** you want the clearest structural analogy to OCaml for learning purposes, or you only ever need one version at a time and functional updates are infrequent.

**Use `Vec::clone()` when:** you need "copy-on-write" semantics but don't care about structural sharing — simplest code, fine for small vectors or infrequent updates.
