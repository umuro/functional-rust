# OCaml vs Rust: Van Laarhoven Lenses (Example 212)

## The Core Idea

A Van Laarhoven lens is a single polymorphic function that unifies `get`, `set`,
and `modify` by being abstract over the functor `f`:

```
type Lens s a = ∀f. Functor f ⇒ (a → f a) → s → f s
```

Plugging in `f = Identity` gives you `over` (modify).
Plugging in `f = Const r` gives you `view` (read).
The same function expression handles every operation.

---

## Side-by-Side Code

### OCaml (module-based encoding)

```ocaml
(* OCaml can express the polymorphism through first-class modules *)
module type FUNCTOR = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end

module Identity = struct
  type 'a t = 'a
  let map f x = f x
  let run x = x
end

module Const (M : sig type t end) = struct
  type 'a t = M.t
  let map _f x = x
  let run x = x
end

(* A concrete VL lens for a record field *)
let person_age_lens (module F : FUNCTOR) f p =
  F.map (fun age -> { p with age }) (f p.age)

(* view: plug in Const *)
let view lens s =
  let module C = Const(struct type t = _ end) in
  C.run (lens (module C) (fun a -> C.map (fun _ -> a) (C.run ())) s)

(* Composition = function composition *)
let compose outer inner (module F : FUNCTOR) f s =
  outer (module F) (inner (module F) f) s
```

### Rust (trait-based encoding with two specialisations)

```rust
use std::rc::Rc;

// Type aliases make the functor applications readable
type IdentityApp<S, A> = Rc<dyn Fn(Rc<dyn Fn(A) -> A>) -> Rc<dyn Fn(S) -> S>>;
type ConstApp<S, A>    = Rc<dyn Fn(&S) -> A>;

pub struct VLLens<S: 'static, A: 'static> {
    apply_identity: IdentityApp<S, A>,  // for over/set
    apply_const:    ConstApp<S, A>,     // for view
}

impl<S: 'static, A: 'static> VLLens<S, A> {
    pub fn view(&self, s: &S) -> A {
        (self.apply_const)(s)
    }

    pub fn over(&self, s: S, f: impl Fn(A) -> A + 'static) -> S {
        let modifier = (self.apply_identity)(Rc::new(f));
        modifier(s)
    }
}
```

### Rust (composition — mirrors Haskell's `.`)

```rust
impl<S: 'static, A: 'static> VLLens<S, A> {
    pub fn compose<B: 'static>(self, inner: VLLens<A, B>) -> VLLens<S, B> {
        let outer_id = self.apply_identity;
        let inner_id = inner.apply_identity;
        let outer_c  = self.apply_const;
        let inner_c  = inner.apply_const;

        VLLens {
            // This IS function composition: outer(inner(f))
            apply_identity: Rc::new(move |f| {
                let inner_lifted = (inner_id)(f);   // (B→B) → (A→A)
                (outer_id)(inner_lifted)             // (A→A) → (S→S)
            }),
            apply_const: Rc::new(move |s| {
                let a = (outer_c)(s);               // S → A
                (inner_c)(&a)                       // A → B
            }),
        }
    }
}
```

---

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Lens type | `∀f. Functor f ⇒ (a → f a) → s → f s` | `struct VLLens<S, A>` with two `Rc<dyn Fn>` fields |
| Identity functor | `module Identity : FUNCTOR with type 'a t = 'a` | `struct Identity<A>(A)` |
| Const functor | `module Const(M) : FUNCTOR with type 'a t = M.t` | `struct Const<R, B>(R, PhantomData<B>)` |
| Composition | `fun f s -> outer (inner f) s` — plain `(.)` | `compose` method; body is identical in structure |
| Rank-2 poly | Native: `∀f.` in the type | Not native; split into two fields |

---

## Key Insights

1. **OCaml's first-class modules are the key difference.**
   OCaml can pass `(module F : FUNCTOR)` as a value at runtime, so a single
   function body handles every functor. Rust has no equivalent; we must split the
   single polymorphic function into two monomorphised fields.

2. **Composition is function composition in both languages.**
   Whether you write `outer . inner` in Haskell/OCaml or
   `Rc::new(|f| outer_id(inner_id(f)))` in Rust, the structure is identical:
   the outer lens's `apply` receives the inner lens's `apply` as its argument.
   No special `compose_lens` combinator is needed — or exists — in either encoding.

3. **The functor determines the operation.**
   Identity carries the modified value forward; Const ignores updates and
   propagates a read value. This duality is what lets one lens type expression
   unify `get`, `set`, and `modify` with no branching on the operation.

4. **`'static` bounds are the Rust cost of erasing the functor.**
   Because `apply_identity` stores a `dyn Fn` behind `Rc`, the closures passed
   to `over` must be `'static`. OCaml has no such restriction because functors
   are passed as arguments, not stored in trait objects.

5. **This is why Haskell's `lens` library composes with `(.)`.**
   Every operator in that library (`^.`, `%~`, `.~`, `^..`) is just a different
   functor plugged into the same lens value. Understanding the VL encoding unlocks
   the entire optics hierarchy: Prism, Traversal, Iso, and Affine all follow the
   same pattern with different functor constraints.

---

## When to Use Each Style

**Use idiomatic Rust `VLLens` when:** you want free lens composition with the
same call-site ergonomics as `view`/`over`/`set`, and you can tolerate `'static`
bounds on modifier closures. This is the closest Rust gets to the Haskell `lens`
experience.

**Use the simpler `Lens` struct (Example 205) when:** you only need one or two
levels of composition and prefer avoiding `Rc<dyn Fn>` overhead.  The standard
`{ get, set }` pair composes via explicit wiring but incurs no dynamic dispatch
on the lens operations themselves.
