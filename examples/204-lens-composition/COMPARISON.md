# OCaml vs Rust: Lens Composition — Zoom Into Nested Structs

## Side-by-Side Code

### OCaml

```ocaml
type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

let compose outer inner = {
  get = (fun s -> inner.get (outer.get s));
  set = (fun b s ->
    let a = outer.get s in
    outer.set (inner.set b a) s);
}

let ( |>> ) = compose
```

### Rust (idiomatic — boxed closures)

```rust
type GetFn<S, A> = Box<dyn Fn(&S) -> A>;
type SetFn<S, A> = Box<dyn Fn(A, &S) -> S>;

pub struct Lens<S, A> {
    pub get: GetFn<S, A>,
    pub set: SetFn<S, A>,
}

impl<S: 'static, A: Clone + 'static> Lens<S, A> {
    pub fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B> {
        use std::rc::Rc;
        let outer_get = Rc::new(self.get);
        let outer_get2 = Rc::clone(&outer_get);
        let outer_set = self.set;
        let inner_get = inner.get;
        let inner_set = inner.set;
        Lens {
            get: Box::new(move |s| inner_get(&outer_get(s))),
            set: Box::new(move |b, s| {
                let a = outer_get2(s);
                outer_set(inner_set(b, &a), s)
            }),
        }
    }
}
```

### Rust (functional / recursive usage — three-level chain)

```rust
// Three lenses snapped together into one:
let person_street_number = person_address_lens()
    .compose(address_street_lens())
    .compose(street_number_lens());

// Read three levels deep in one call:
let n = (person_street_number.get)(&alice);

// Update three levels deep in one call:
let updated = (person_street_number.set)(99, &alice);
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Lens type | `('s, 'a) lens` | `Lens<S, A>` |
| Get function | `get : 's -> 'a` | `get: Box<dyn Fn(&S) -> A>` |
| Set function | `set : 'a -> 's -> 's` | `set: Box<dyn Fn(A, &S) -> S>` |
| Compose result | `('s, 'b) lens` | `Lens<S, B>` |
| Ownership of S | Immutable value | Immutable reference `&S` |

## Key Insights

1. **Records vs structs with boxed closures**: OCaml's `{ get; set }` record maps directly to a Rust `struct` — but OCaml functions are first-class values while Rust closures require `Box<dyn Fn(...)>` to be stored in a struct field.

2. **Sharing the outer `get` across two closures**: Both the composed `get` and `set` need to call `outer_get`. In OCaml this is free (closures share the environment by reference); in Rust we need `Rc` to give two owning closures a shared handle to the same boxed function.

3. **Immutable update syntax**: OCaml's `{ p with address = a }` and Rust's `Person { address: a, ..p.clone() }` are both *functional update* — neither mutates in place. Rust requires `Clone` because `..p` in a struct literal moves or copies each field.

4. **Composition is associative**: `(A |>> B) |>> C` equals `A |>> (B |>> C)` in both languages. This algebraic property means you can chain any number of lenses in any grouping and always get the same result — the test `test_composition_is_associative` verifies this explicitly.

5. **Type erasure cost**: OCaml infers the most general polymorphic type for a lens at zero runtime cost. Rust's `Box<dyn Fn(...)>` performs type erasure via a vtable, paying a small indirection cost but allowing lenses of different concrete types to be stored uniformly.

## When to Use Each Style

**Use idiomatic Rust (boxed closures) when:** you want dynamically constructed lenses at runtime, or you need to store lenses of varying concrete types in a collection.
**Use recursive Rust (trait-based lenses) when:** you have a performance-critical hot path — a trait-object-free approach avoids heap allocation and virtual dispatch at the cost of more complex generic bounds.
