# OCaml vs Rust: Lenses

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

let addr_lens = { get = (fun p -> p.addr); set = (fun a p -> { p with addr = a }) }
let city_lens = { get = (fun a -> a.city); set = (fun c a -> { a with city = c }) }
let person_city = compose addr_lens city_lens
```

### Rust (idiomatic)
```rust
type Getter<S, A> = Box<dyn Fn(&S) -> &A>;
type Setter<S, A> = Box<dyn Fn(A, &S) -> S>;

struct Lens<S, A> {
    get_fn: Getter<S, A>,
    set_fn: Setter<S, A>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    fn new(get: impl Fn(&S) -> &A + 'static, set: impl Fn(A, &S) -> S + 'static) -> Self {
        Lens { get_fn: Box::new(get), set_fn: Box::new(set) }
    }

    fn get<'s>(&self, whole: &'s S) -> &'s A { (self.get_fn)(whole) }
    fn set(&self, value: A, whole: &S) -> S { (self.set_fn)(value, whole) }
}

fn over<S: 'static, A: Clone + 'static>(lens: &Lens<S, A>, f: impl FnOnce(A) -> A, whole: &S) -> S {
    let current = lens.get(whole).clone();
    lens.set(f(current), whole)
}
```

### Rust (composed lens via Arc)
```rust
fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
where S: Clone, A: Clone {
    let inner = Arc::new(inner);
    let outer_get = Arc::new(self.get_fn);
    let outer_set = Arc::new(self.set_fn);
    // ... share Arcs between get and set closures
    Lens::new(
        move |s| { /* get outer, then get inner */ },
        move |b, s| { /* get outer, set inner, set outer */ },
    )
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Lens type | `('s, 'a) lens` | `Lens<S, A>` |
| Get | `'s -> 'a` | `Fn(&S) -> &A` |
| Set | `'a -> 's -> 's` | `Fn(A, &S) -> S` |
| Compose | `('s, 'a) lens -> ('a, 'b) lens -> ('s, 'b) lens` | `Lens<S, A> -> Lens<A, B> -> Lens<S, B>` |
| Over | `('s, 'a) lens -> ('a -> 'a) -> 's -> 's` | `&Lens<S, A>, FnOnce(A) -> A, &S -> S` |
| Record update | `{ p with field = v }` | Manual struct construction + Clone |

## Key Insights

1. **Closures as values:** OCaml's record-of-closures maps directly to Rust's struct-of-boxed-closures, but Rust requires explicit `'static` bounds for boxed trait objects. This is the fundamental cost of no GC — you must prove closure lifetimes to the compiler.

2. **Composition needs sharing:** OCaml's `compose` creates two new closures that capture the inner lens. In Rust, the inner lens must be wrapped in `Arc` because both the composed getter and setter need access to it, and closures can't share ownership without reference counting.

3. **Borrowing vs copying in get:** OCaml's `get` returns a value (copied by the GC). Rust's `get` returns a `&A` reference into the original structure, avoiding allocation. This is more efficient but makes composition harder — the composed getter must thread lifetimes through two levels of indirection.

4. **Set requires Clone:** OCaml's `{ p with addr = a }` is syntactic sugar that cheaply creates a new record. Rust has no equivalent — you must manually construct a new struct and `Clone` every unchanged field. This makes the `Clone` bound mandatory on the set path.

5. **Lens laws hold in both:** The three lens laws (get-set, set-get, set-set) are preserved identically. Functional purity means the same equational reasoning applies regardless of language. The tests verify all three laws.

## When to Use Each Style

**Use idiomatic Rust lenses when:** you have deeply nested structures that need frequent functional updates, especially in state management for UI frameworks or game engines where immutability prevents bugs.

**Use OCaml-style lenses when:** you're writing OCaml or want to understand the theoretical foundation. OCaml's GC and structural equality make lenses almost zero-cost to define and compose, which is why they're more common in ML-family languages.
