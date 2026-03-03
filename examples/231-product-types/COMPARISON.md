# OCaml vs Rust: Product Types

## Side-by-Side Code

### OCaml

```ocaml
type point2d = { x: float; y: float }

let swap (a, b) = (b, a)
let fst (a, _) = a
let snd (_, b) = b
let pair a b = (a, b)

let uncurry f (a, b) = f a b
let curry f a b = f (a, b)

let distance p q =
  let dx = p.x -. q.x and dy = p.y -. q.y in
  sqrt (dx *. dx +. dy *. dy)
```

### Rust (idiomatic — method on struct)

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d { pub x: f64, pub y: f64 }

impl Point2d {
    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub fn swap<A, B>(pair: (A, B)) -> (B, A) { (pair.1, pair.0) }
pub fn fst<A, B>(pair: (A, B)) -> A { pair.0 }
pub fn snd<A, B>(pair: (A, B)) -> B { pair.1 }
```

### Rust (functional — curry/uncurry with Rc)

```rust
use std::rc::Rc;

pub fn uncurry<A, B, C>(f: impl Fn(A, B) -> C) -> impl Fn((A, B)) -> C {
    move |(a, b)| f(a, b)
}

pub fn curry<A: Clone + 'static, B: 'static, C: 'static>(
    f: impl Fn((A, B)) -> C + 'static,
) -> impl Fn(A) -> Box<dyn Fn(B) -> C> {
    let f = Rc::new(f);
    move |a: A| {
        let f = Rc::clone(&f);
        let a = a.clone();
        Box::new(move |b: B| f((a.clone(), b)))
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Record type | `type point2d = { x: float; y: float }` | `struct Point2d { x: f64, y: f64 }` |
| Tuple swap | `val swap : ('a * 'b) -> ('b * 'a)` | `fn swap<A,B>(pair: (A,B)) -> (B,A)` |
| Projection | `val fst : ('a * 'b) -> 'a` | `fn fst<A,B>(pair: (A,B)) -> A` |
| Uncurry | `val uncurry : ('a -> 'b -> 'c) -> 'a * 'b -> 'c` | `fn uncurry<A,B,C>(f: impl Fn(A,B)->C) -> impl Fn((A,B))->C` |
| Curry | `val curry : ('a * 'b -> 'c) -> 'a -> 'b -> 'c` | `fn curry<A,B,C>(f: impl Fn((A,B))->C) -> impl Fn(A)->Box<dyn Fn(B)->C>` |
| Distance | `val distance : point2d -> point2d -> float` | `fn distance(p: &Point2d, q: &Point2d) -> f64` |

## Key Insights

1. **Structs vs records:** OCaml records and Rust structs are syntactically similar, but Rust structs are *nominal* — two structs with identical fields are different types. OCaml records are also nominal, but the structural feel differs because field access doesn't require `self`.

2. **Tuple ownership:** OCaml tuples are garbage-collected values; projecting with `fst`/`snd` doesn't affect the original. Rust tuples are moved — calling `fst((a, b))` *consumes* the tuple. For `Copy` types (like integers) this is invisible; for heap types it matters.

3. **No automatic currying:** Every OCaml function is automatically curried (`f a b` is `(f a) b`). Rust has no such mechanism; multi-argument functions take all arguments at once. Simulating currying requires closures, and sharing a closure across repeated calls requires reference counting (`Rc`) or a `Clone` bound.

4. **Method syntax:** OCaml groups functions in modules (`module Point2d = struct ... end`). Rust groups methods in `impl` blocks directly on the type, which is generally more discoverable and is the idiomatic style for domain types with associated behaviour.

5. **`'static` lifetime bounds:** `Box<dyn Fn(B) -> C>` is implicitly `+ 'static`, which forces `A: 'static` and the captured `f: 'static`. In OCaml all values have indefinite lifetime (GC), so this constraint has no analogue.

## When to Use Each Style

**Use the idiomatic Rust (method) style when:** you own a domain type and want behaviour collocated with data — i.e., almost always for structs like `Point2d`.

**Use the functional (free-function) style when:** implementing generic combinators (`swap`, `uncurry`, `curry`) that operate on any pair type and have no natural "owner" type to attach to.
