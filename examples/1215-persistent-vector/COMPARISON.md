# OCaml vs Rust: Persistent Vector

## Side-by-Side Code

### OCaml

```ocaml
type 'a pvec = Nil | One of 'a | Two of 'a pvec * 'a pvec

let push v x = match v with
  | Nil -> One x
  | _ -> Two (v, One x)

let rec pop = function
  | Nil -> None
  | One x -> Some (x, Nil)
  | Two (l, r) ->
      (match pop r with
       | None -> None
       | Some (x, Nil) -> Some (x, l)
       | Some (x, r') -> Some (x, Two (l, r')))
```

### Rust (idiomatic persistent, using `Rc`)

```rust
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PVec<T> {
    #[default]
    Nil,
    One(T),
    Two(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T: Clone> PVec<T> {
    pub fn push(&self, x: T) -> Self {
        match self {
            PVec::Nil => PVec::One(x),
            _ => PVec::Two(Rc::new(self.clone()), Rc::new(PVec::One(x))),
        }
    }

    pub fn pop(&self) -> Option<(T, Self)> {
        match self {
            PVec::Nil => None,
            PVec::One(v) => Some((v.clone(), PVec::Nil)),
            PVec::Two(l, r) => {
                let (v, new_r) = r.pop()?;
                let rest = match new_r {
                    PVec::Nil => (**l).clone(),
                    other => PVec::Two(l.clone(), Rc::new(other)),
                };
                Some((v, rest))
            }
        }
    }
}
```

### Rust (functional/recursive length and indexed access)

```rust
pub fn len(&self) -> usize {
    match self {
        PVec::Nil => 0,
        PVec::One(_) => 1,
        PVec::Two(l, r) => l.len() + r.len(),
    }
}

pub fn get(&self, i: usize) -> Option<&T> {
    match self {
        PVec::Nil => None,
        PVec::One(v) => (i == 0).then_some(v),
        PVec::Two(l, r) => {
            let left_len = l.len();
            if i < left_len { l.get(i) } else { r.get(i - left_len) }
        }
    }
}
```

## Type Signatures

| Concept              | OCaml                                           | Rust                                                 |
| -------------------- | ----------------------------------------------- | ---------------------------------------------------- |
| Vector type          | `'a pvec`                                       | `PVec<T>`                                            |
| Shared subtree       | implicit (GC managed)                           | `Rc<PVec<T>>`                                        |
| Push                 | `'a pvec -> 'a -> 'a pvec`                      | `fn push(&self, x: T) -> Self` *(requires `T: Clone`)* |
| Pop                  | `'a pvec -> ('a * 'a pvec) option`              | `fn pop(&self) -> Option<(T, Self)>`                 |
| Empty variant        | `Nil`                                           | `PVec::Nil` + `#[default]`                           |

## Key Insights

1. **Structural sharing is the whole point.**  Both languages reuse the
   untouched subtree; the difference is *how* they reuse it.  OCaml relies
   on GC; Rust relies on `Rc`'s reference count.
2. **Ownership forces `T: Clone`.**  Rust's `pop` has to return a `T`, but
   the `T` lives behind a shared `Rc`, so we cannot move it out — we must
   clone.  OCaml has no such constraint because values are conceptually
   copied by reference anyway.
3. **Enum `#[default]` is the Rust answer to OCaml's "first constructor
   is the zero value".**  One attribute replaces a hand-written `impl
   Default`.
4. **Collapsing empty `Two` nodes matters.**  Without the
   `PVec::Nil => (**l).clone()` branch in `pop`, a long sequence of pops
   leaves a spine of `Two(l, Nil)` nodes that never get cleaned up.
5. **Performance caveat.**  Both `length` and `get` are O(n) on this
   shape because nothing is rebalanced.  Production persistent vectors
   (Clojure, Scala, `im::Vector`) use a 32-ary bit-partitioned trie and
   cache lengths — this example trades that complexity for clarity.

## When to Use Each Style

**Use idiomatic Rust `Rc`-shared version when:** you want real persistence
(cheap branching history, time-travel, undo stacks) in single-threaded code.

**Use recursive pattern matching on `&self` when:** you only need read-only
traversal (`len`, `get`, `to_vec`) — no `Rc` clones happen, just borrow
descent.
