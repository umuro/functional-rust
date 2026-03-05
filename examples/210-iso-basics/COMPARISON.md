# OCaml vs Rust: Iso Basics — Lossless Bidirectional Transformations

## Side-by-Side Code

### OCaml

```ocaml
type ('s, 'a) iso = {
  get         : 's -> 'a;
  reverse_get : 'a -> 's;
}

let celsius_fahrenheit : (float, float) iso = {
  get         = (fun c -> c *. 9.0 /. 5.0 +. 32.0);
  reverse_get = (fun f -> (f -. 32.0) *. 5.0 /. 9.0);
}

(* Reverse: swap field values *)
let reverse iso = { get = iso.reverse_get; reverse_get = iso.get }

(* Compose: chain two isos *)
let compose iso1 iso2 = {
  get         = (fun s -> iso2.get (iso1.get s));
  reverse_get = (fun b -> iso1.reverse_get (iso2.reverse_get b));
}
```

### Rust (idiomatic — trait objects for type erasure)

```rust
pub struct Iso<S, A> {
    pub get: Box<dyn Fn(&S) -> A>,
    pub reverse_get: Box<dyn Fn(&A) -> S>,
}

impl<S: 'static, A: 'static> Iso<S, A> {
    pub fn new(
        get: impl Fn(&S) -> A + 'static,
        reverse_get: impl Fn(&A) -> S + 'static,
    ) -> Self {
        Iso { get: Box::new(get), reverse_get: Box::new(reverse_get) }
    }

    pub fn reverse(self) -> Iso<A, S> {
        Iso { get: self.reverse_get, reverse_get: self.get }
    }

    pub fn compose<B: 'static>(self, other: Iso<A, B>) -> Iso<S, B> {
        let get_self  = self.get;
        let rev_self  = self.reverse_get;
        let get_other = other.get;
        let rev_other = other.reverse_get;
        Iso {
            get:         Box::new(move |s| get_other(&get_self(s))),
            reverse_get: Box::new(move |b| rev_self(&rev_other(b))),
        }
    }
}

pub fn celsius_fahrenheit() -> Iso<f64, f64> {
    Iso::new(
        |c| c * 9.0 / 5.0 + 32.0,
        |f| (f - 32.0) * 5.0 / 9.0,
    )
}
```

### Rust (functional — newtype wrapper Iso)

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Celsius(pub f64);

pub fn celsius_raw() -> Iso<Celsius, f64> {
    Iso::new(
        |c: &Celsius| c.0,
        |f: &f64| Celsius(*f),
    )
}
```

## Type Signatures

| Concept              | OCaml                                    | Rust                                          |
|----------------------|------------------------------------------|-----------------------------------------------|
| Iso type             | `('s, 'a) iso` (record)                  | `Iso<S, A>` (struct with `Box<dyn Fn>` fields)|
| Forward function     | `get : 's -> 'a`                         | `Box<dyn Fn(&S) -> A>`                        |
| Backward function    | `reverse_get : 'a -> 's`                 | `Box<dyn Fn(&A) -> S>`                        |
| Reverse              | swap record fields, value semantics      | `self`-consuming method, moves ownership      |
| Compose              | function composition, pure               | closures capture moved `Box<dyn Fn>` fields   |
| Newtype unwrap       | `let Meters m = x`                       | tuple struct field access `x.0`               |
| Lifetime requirement | implicit (GC)                            | `'static` bound on type params                |

## Key Insights

1. **Record vs struct with closures:** OCaml's record `{ get; reverse_get }` maps cleanly to a Rust struct, but Rust functions stored in structs require `Box<dyn Fn(...)>` for heap-allocated, type-erased closures — or generic type parameters if monomorphisation is preferred.

2. **Ownership on `reverse`:** In Rust, `reverse` consumes `self` (moving the `Box<dyn Fn>` fields). This prevents using the original Iso after reversal, which is correct — you now hold the inverse. OCaml shares field values by default (boxed on the heap under GC), so no explicit transfer is needed.

3. **`'static` lifetime bound:** Rust closures stored in `Box<dyn Fn>` must be `'static` when the struct escapes the stack. OCaml's GC handles this transparently. This surfaces when composing Isos: each closure must own (not borrow) its captured values.

4. **Newtype Isos:** OCaml uses algebraic constructors (`Meters of float`); Rust uses tuple structs (`struct Celsius(f64)`). Both let you wrap and unwrap a primitive. The Iso makes the wrap/unwrap contract explicit and composable rather than scattered across the codebase.

5. **Roundtrip laws as tests:** Neither language enforces the Iso laws (`get ∘ reverse_get = id`) in the type system. Rust property-based tests (or manual `assert!` tests) serve as the enforcement mechanism — just as OCaml uses `assert` at the top level.

## When to Use Each Style

**Use idiomatic Rust (trait objects)** when the Iso will be passed around as a value, returned from functions, or composed dynamically at runtime — the `Box<dyn Fn>` overhead is acceptable and the API is clean.

**Use generic type parameters** (`struct Iso<S, A, F, G>`) when performance matters and the Iso is used at a fixed, known call site — this enables monomorphisation and inlining, at the cost of more complex type signatures.

**Use newtype Isos** when you want the type system to prevent mixing up unit-bearing values (e.g., `Celsius` vs `Fahrenheit`) — the Iso then documents and enforces the only safe conversion path.
