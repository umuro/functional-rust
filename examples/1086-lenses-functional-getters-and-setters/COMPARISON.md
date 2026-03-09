# OCaml vs Rust: Lenses — Functional Getters and Setters

## Side-by-Side Code

### OCaml
```ocaml
type ('s, 'a) lens = {
  get: 's -> 'a;
  set: 'a -> 's -> 's;
}

let compose outer inner = {
  get = (fun s -> inner.get (outer.get s));
  set = (fun a s -> outer.set (inner.set a (outer.get s)) s);
}

let over lens f s = lens.set (f (lens.get s)) s

type address = { street: string; city: string }
type person = { name: string; addr: address }

let addr_lens = { get = (fun p -> p.addr); set = (fun a p -> { p with addr = a }) }
let city_lens = { get = (fun a -> a.city); set = (fun c a -> { a with city = c }) }
let person_city = compose addr_lens city_lens
```

### Rust (idiomatic — struct with boxed closures)
```rust
pub struct Lens<S, A> {
    getter: Box<dyn Fn(&S) -> A>,
    setter: Box<dyn Fn(A, &S) -> S>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    pub fn new(
        getter: impl Fn(&S) -> A + 'static,
        setter: impl Fn(A, &S) -> S + 'static,
    ) -> Self {
        Lens { getter: Box::new(getter), setter: Box::new(setter) }
    }

    pub fn get(&self, s: &S) -> A { (self.getter)(s) }
    pub fn set(&self, a: A, s: &S) -> S { (self.setter)(a, s) }
    pub fn over(&self, f: impl Fn(A) -> A, s: &S) -> S {
        self.set(f(self.get(s)), s)
    }

    pub fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where A: Clone {
        use std::rc::Rc;
        let og = Rc::new(self.getter);
        let os = Rc::new(self.setter);
        let ig = Rc::new(inner.getter);
        let is_ = Rc::new(inner.setter);

        let (og1, ig1) = (Rc::clone(&og), Rc::clone(&ig));
        let (og2, os2, is2) = (Rc::clone(&og), Rc::clone(&os), Rc::clone(&is_));

        Lens::new(
            move |s: &S| ig1(&og1(s)),
            move |b, s: &S| { let a = og2(s); os2(is2(b, &a), s) },
        )
    }
}
```

### Rust (functional/recursive style — lens combinators)
```rust
fn addr_lens() -> Lens<Person, Address> {
    Lens::new(
        |p: &Person| p.addr.clone(),
        |a, p| Person { name: p.name.clone(), addr: a },
    )
}

fn city_lens() -> Lens<Address, String> {
    Lens::new(
        |a: &Address| a.city.clone(),
        |c, a| Address { street: a.street.clone(), city: c },
    )
}

fn person_city_lens() -> Lens<Person, String> {
    addr_lens().compose(city_lens())
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Lens type | `('s, 'a) lens` (record) | `Lens<S, A>` (struct with `Box<dyn Fn>`) |
| Getter | `get: 's -> 'a` | `Fn(&S) -> A` |
| Setter | `set: 'a -> 's -> 's` | `Fn(A, &S) -> S` |
| Compose | `('s, 'a) lens -> ('a, 'b) lens -> ('s, 'b) lens` | `Lens<S,A>.compose(Lens<A,B>) -> Lens<S,B>` |
| Over | `('s, 'a) lens -> ('a -> 'a) -> 's -> 's` | `lens.over(Fn(A)->A, &S) -> S` |
| Record update | `{ p with addr = a }` | `Person { name: p.name.clone(), addr: a }` |

## Key Insights

1. **First-class functions as fields:** OCaml records can hold functions directly with no type-erasure cost. Rust needs `Box<dyn Fn>` (heap allocation + vtable dispatch) to store closures with different captured environments in the same struct.
2. **Composition and ownership:** In OCaml, composing lenses just creates a new record — closures are GC'd values. In Rust, `compose` must consume both lenses and wrap their internals in `Rc` so the composed getter and setter can both reference the original functions.
3. **Clone at the boundary:** OCaml's GC means `get` returns a value that shares the original's memory. Rust's getter returns an *owned* value, so struct fields must be `.clone()`d — this is the price of no-GC ownership.
4. **Immutable update ergonomics:** OCaml's `{ record with field = value }` is built-in syntax. Rust has no equivalent — every "functional update" must list all fields explicitly (or derive a builder). This makes lenses even more valuable in Rust for hiding update boilerplate.
5. **Lens laws hold identically:** The three lens laws (get-set, set-get, set-set) are algebraic properties independent of language. Both OCaml and Rust lenses must satisfy them to be correct, and our tests verify all three.

## When to Use Each Style

**Use the struct-with-closures approach when:** you need composable, reusable lenses that can be stored, passed around, and combined at runtime — especially for deeply nested configuration or domain objects.
**Use direct field access when:** the nesting is shallow (one level), performance is critical (avoid the clone + vtable overhead), or the code only reads/writes a single field.
