# OCaml vs Rust: Variance

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml annotates variance explicitly on type parameters *)
type +'a producer = { produce : unit -> 'a }   (* covariant:     +'a *)
type -'a consumer = { consume : 'a -> unit }   (* contravariant: -'a *)
type  'a invariant_ref = { mutable contents : 'a } (* invariant: no annotation *)

let int_prod : int producer = { produce = fun () -> 42 }
(* A producer of int can be widened to a producer of a supertype *)

let print_consumer : string consumer = { consume = print_endline }
(* A consumer of string can be narrowed: anything consuming 'a can consume a subtype *)
```

### Rust (idiomatic — PhantomData)
```rust
use std::marker::PhantomData;

// Covariant in T: PhantomData<T> mirrors &T
pub struct Producer<T> {
    func: fn() -> T,
    _marker: PhantomData<T>,
}

// Contravariant in T: PhantomData<fn(T)>
pub struct Consumer<T> {
    func: fn(T),
    _marker: PhantomData<fn(T)>,
}

// Invariant in T: PhantomData<fn(T) -> T> (both producer and consumer)
pub struct Invariant<T> {
    value: T,
    _marker: PhantomData<fn(T) -> T>,
}
```

### Rust (lifetime variance — no PhantomData needed)
```rust
// &'a str is covariant in 'a: a long-lived reference is valid where a short one is needed
fn use_short<'short>(s: &'short str) -> usize { s.len() }

fn demo<'long>(long: &'long str) {
    use_short(long);   // 'long shrinks to 'short — covariance in action
}

// &'a mut T is invariant in T — the compiler refuses widening or narrowing:
// fn push_cat(v: &mut Vec<Animal>) { v.push(Cat); }
// let mut dogs: Vec<Dog> = vec![Dog];
// push_cat(&mut dogs);  // ← compile error: invariance saves soundness
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Covariant producer | `type +'a producer` | `PhantomData<T>` or `PhantomData<&'a T>` |
| Contravariant consumer | `type -'a consumer` | `PhantomData<fn(T)>` |
| Invariant cell | `type 'a ref` (no annotation) | `PhantomData<fn(T) -> T>` or `*mut T` |
| Shared reference | `'a` (inferred covariant) | `&'a T` — covariant in both `'a` and `T` |
| Mutable reference | N/A (OCaml refs are invariant) | `&'a mut T` — invariant in `T`, covariant in `'a` |

## Key Insights

1. **Annotation vs inference**: OCaml requires explicit `+'a` / `-'a` annotations on type parameters; Rust *infers* variance automatically from how the type parameter is used in struct fields.

2. **PhantomData as a declaration**: When a Rust struct holds no real `T` (e.g. raw pointers, zero-sized wrappers), you use `PhantomData<T>`, `PhantomData<fn(T)>`, or `PhantomData<fn(T)->T>` to tell the compiler what variance you intend.

3. **Mutable references are invariant in their target type**: `&mut Vec<Dog>` cannot widen to `&mut Vec<Animal>` — if it could, a function expecting `&mut Vec<Animal>` could push a `Cat` and corrupt the `Vec<Dog>`.  OCaml mutable records have the same constraint.

4. **Lifetimes are the primary arena**: Unlike OCaml (which lacks lifetimes), Rust's variance rules are most visible through `'a` — `&'a T` is covariant so a long-lived reference can fill a short-lived slot, enabling ergonomic lifetime shortening without unsafe code.

5. **Soundness is enforced at compile time**: Both languages use variance to uphold type safety, but Rust enforces it through borrow-checker constraints rather than runtime checks, giving zero-cost guarantees.

## When to Use Each Style

**Use covariant (`PhantomData<T>`)** when your wrapper only *produces* or *returns* `T` values — like iterators, generators, or read-only handles.

**Use contravariant (`PhantomData<fn(T)>`)** when your wrapper only *consumes* `T` — like callbacks, sinks, or write-only handles.

**Use invariant (`PhantomData<fn(T) -> T>` or `*mut T`)** when your wrapper both reads and writes `T`, or when you're wrapping raw pointers that could alias.

**Use phantom lifetimes (`PhantomData<&'a ()>`)** to tie a handle's lifetime to borrowed data without actually storing a reference, so the borrow checker enforces correct lifetimes on the API boundary.
