# OCaml vs Rust: Persistent Vector (Functional Array)

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
    if i < ls then Two (set i v l, r)
    else Two (l, set (i - ls) v r)
  | Nil -> failwith "empty"
```

### Rust (idiomatic — method API)
```rust
#[derive(Debug, Clone)]
pub enum PVec<T> {
    Nil,
    One(T),
    Two(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T> PVec<T> {
    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            PVec::Nil => None,
            PVec::One(x) => (i == 0).then_some(x),
            PVec::Two(l, r) => {
                let ls = l.size();
                if i < ls { l.get(i) } else { r.get(i - ls) }
            }
        }
    }

    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVec::Nil => None,
            PVec::One(_) => (i == 0).then(|| PVec::One(v)),
            PVec::Two(l, r) => {
                let ls = l.size();
                if i < ls {
                    let new_l = l.set(i, v)?;
                    Some(PVec::Two(Rc::new(new_l), Rc::clone(r)))
                } else {
                    let new_r = r.set(i - ls, v)?;
                    Some(PVec::Two(Rc::clone(l), Rc::new(new_r)))
                }
            }
        }
    }
}
```

### Rust (functional style — free functions mirroring OCaml argument order)
```rust
pub fn pvec_get<T>(i: usize, v: &PVec<T>) -> Option<&T> {
    match v {
        PVec::Nil => None,
        PVec::One(x) => (i == 0).then_some(x),
        PVec::Two(l, r) => {
            let ls = pvec_size(l);
            if i < ls { pvec_get(i, l) } else { pvec_get(i - ls, r) }
        }
    }
}

pub fn pvec_set<T>(i: usize, val: T, v: &PVec<T>) -> Option<PVec<T>> {
    match v {
        PVec::Nil => None,
        PVec::One(_) => (i == 0).then(|| PVec::One(val)),
        PVec::Two(l, r) => {
            let ls = pvec_size(l);
            if i < ls {
                let new_l = pvec_set(i, val, l)?;
                Some(PVec::Two(Rc::new(new_l), Rc::clone(r)))
            } else {
                let new_r = pvec_set(i - ls, val, r)?;
                Some(PVec::Two(Rc::clone(l), Rc::new(new_r)))
            }
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| ADT definition | `type 'a pvec = Nil \| One of 'a \| Two of 'a pvec * 'a pvec` | `enum PVec<T> { Nil, One(T), Two(Rc<PVec<T>>, Rc<PVec<T>>) }` |
| Get signature | `val get : int -> 'a pvec -> 'a` | `fn get(&self, i: usize) -> Option<&T>` |
| Set signature | `val set : int -> 'a -> 'a pvec -> 'a pvec` | `fn set(&self, i: usize, v: T) -> Option<Self>` |
| Error handling | `failwith "index"` (exception) | `None` (Option) |
| Shared subtree | Implicit via GC | `Rc::clone(r)` — explicit, O(1) |

## Key Insights

1. **Structural sharing is explicit in Rust.** OCaml's GC silently shares unchanged subtrees; Rust requires `Rc::clone(r)` at every sharing point, making the O(log n) allocation budget visible in the code itself.

2. **`Rc` replaces the GC for this pattern.** Reference counting gives the same persistent-structure semantics as a tracing GC: a subtree lives as long as any version references it, and is freed deterministically when the last reference drops.

3. **`Option` replaces exceptions.** OCaml's `failwith "index"` raises an exception that crosses stack frames; Rust's `Option` with the `?` operator propagates the failure value up the call chain without unwinding — composable, type-safe, zero-cost.

4. **`usize` vs `int`.** OCaml uses signed `int` for indices (so negative indices are representable, though not useful here); Rust uses `usize` — unsigned, platform-width — making the precondition `i >= 0` enforced by the type system with no runtime check needed.

5. **Two API styles coexist.** Rust idiom prefers `v.get(i)` (method syntax); the OCaml convention is `get i v` (index before collection). Both styles are provided: the method API for ergonomics and the free-function API for a direct structural translation from OCaml.

## When to Use Each Style

**Use idiomatic Rust (method API) when:** writing new Rust code, composing with other Rust types, or when method chaining (`v.set(2, 99)?.set(3, 42)`) improves readability.

**Use functional free-function style when:** translating OCaml algorithms directly, keeping argument order identical to the source for comparison, or teaching the OCaml→Rust translation explicitly.
