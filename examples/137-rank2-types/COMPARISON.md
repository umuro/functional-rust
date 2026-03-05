# OCaml vs Rust: Rank-2 Types

## The Core Problem

Rank-2 polymorphism lets you pass a function that must work for **any** type — not a single type chosen by the caller.
In rank-1 (normal) generics, the **caller** picks the type parameter.
In rank-2, the **callee** picks it (potentially multiple times, with different types in sequence).

---

## Side-by-Side Code

### OCaml — record-encoded rank-2

```ocaml
(* Wrap the polymorphic function in a record to get rank-2 *)
type id_fn = { id : 'a. 'a -> 'a }

let apply_id (f : id_fn) =
  let x = f.id 42 in        (* int *)
  let y = f.id "hello" in   (* string *)
  (x, y)

let () =
  let result = apply_id { id = fun x -> x } in
  assert (result = (42, "hello"))
```

### OCaml — first-class module rank-2

```ocaml
module type TRANSFORM = sig
  val transform : 'a -> 'a
end

let apply_transform (module T : TRANSFORM) =
  (T.transform 42, T.transform "hello")

module Identity : TRANSFORM = struct
  let transform x = x
end

let () =
  let (n, s) = apply_transform (module Identity) in
  assert (n = 42 && s = "hello")
```

### Rust — trait with generic method (rank-2 via static dispatch)

```rust
pub trait IdFn {
    fn apply<T>(&self, x: T) -> T;
}

pub struct Identity;

impl IdFn for Identity {
    fn apply<T>(&self, x: T) -> T { x }
}

// F: IdFn — NOT &dyn IdFn. Traits with generic methods are not dyn-compatible.
pub fn apply_id<F: IdFn>(f: &F) -> (i32, String) {
    let x = f.apply(42_i32);
    let y = f.apply("hello".to_string());
    (x, y)
}
```

### Rust — ForAll trait with Debug bound

```rust
pub trait ForAll {
    fn call<T: std::fmt::Debug>(&self, val: T) -> String;
}

pub struct ShowDebug;

impl ForAll for ShowDebug {
    fn call<T: std::fmt::Debug>(&self, val: T) -> String {
        format!("{val:?}")
    }
}

pub fn apply_forall<F: ForAll>(f: &F) -> Vec<String> {
    vec![f.call(42_i32), f.call("world"), f.call(vec![1_u8, 2, 3])]
}
```

### Rust — rank-2 over lifetimes (Higher-Ranked Trait Bounds)

```rust
// `for<'a>` is Rust's native rank-2: must work for every lifetime, not one chosen by caller.
pub fn apply_twice_hrtb<F>(f: F) -> (usize, usize)
where
    F: for<'a> Fn(&'a str) -> usize,
{
    let result1 = { let s1 = String::from("hello world"); f(&s1) };
    let result2 = { let s2 = String::from("rank-2"); f(&s2) };
    (result1, result2)
}
```

---

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Rank-2 type | `{ id : 'a. 'a -> 'a }` | `trait IdFn { fn apply<T>(&self, x: T) -> T; }` |
| Universal quantifier | `'a.` (explicit in record) | Implicit in trait's generic method parameter |
| Encoding mechanism | Record field / first-class module | Trait bound `F: IdFn` (static dispatch) |
| Dispatch style | Structural (record field, module) | Monomorphisation (zero-cost) |
| Native rank-2 syntax | `'a.` in record type | `for<'a>` in HRTB (lifetime level only) |

---

## Key Insights

1. **Rust traits with generic methods are *not* dyn-compatible.**
   Unlike OCaml records, you cannot erase a trait with generic methods into `&dyn Trait` — the compiler
   cannot build a vtable for a method whose concrete type isn't known. Rank-2 in Rust is therefore encoded
   via *static dispatch* (`F: IdFn`), not dynamic dispatch. This is actually more performant: zero overhead,
   monomorphised at compile time.

2. **OCaml uses records; Rust uses trait bounds.**
   OCaml annotates a record field with `'a.` (an explicit universal quantifier), forcing any record value
   to carry a function that works for all `'a`. Rust's equivalent is `F: IdFn` where `IdFn` has a generic
   method — the compiler enforces that `F` must handle every `T` the method might be called with.

3. **`for<'a>` is Rust's native rank-2 — but only over lifetimes.**
   Rust's higher-ranked trait bounds (`for<'a> Fn(&'a str) -> usize`) are genuine rank-2: the caller cannot
   fix `'a`; the callee uses the function with independently-scoped borrows. This mirrors Haskell's `runST`
   pattern and is the language's built-in rank-2 mechanism — though only over lifetimes, not types.

4. **First-class modules ↔ generic type parameters.**
   OCaml's `(module T : TRANSFORM)` passes a module whose `transform` is universally polymorphic.
   Rust's `<F: ForAll>(f: &F)` is structurally identical: `F` must implement `call<T>` for all `T`,
   and the concrete type is resolved at compile time by the monomorphiser.

5. **ST-monad safety: rank-2 as an escape hatch.**
   In Haskell's `runST` and OCaml's `ST` simulation, rank-2 prevents mutable state references from
   leaking out of a computation. Rust achieves the same with lifetime branding (an invariant phantom
   lifetime `'s`): the type system statically rejects any attempt to let a `&'s mut T` escape the
   closure that created it, without needing rank-2 over types at all.

---

## When to Use Each Style

**Use `F: IdFn` (generic bound / static dispatch)** when the concrete implementor is always known at
compile time and you want zero-cost monomorphisation — equivalent to OCaml's first-class modules
resolved by the compiler.

**Use `for<'a>` HRTBs** when you need a closure or function that must be valid for every possible
lifetime — the canonical Rust use case is `fn(&T) -> U` callbacks in APIs that hold borrows of
varying lifetimes.

**Use the ST-lifetime-brand pattern** when you need to guarantee that mutable state created inside
a scope cannot escape it — a safety property that rank-2 enforces at the type level in both OCaml and Rust.
