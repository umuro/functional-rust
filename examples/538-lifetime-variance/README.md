📖 **[View on hightechmind.io →](https://hightechmind.io/rust/538-lifetime-variance)**

---

# Variance: Covariant, Contravariant, Invariant

## Problem Statement

Variance describes how subtyping relationships on types propagate through type constructors. In Rust's lifetime system: if `'long: 'short` (long outlives short), does `Container<'long>` also satisfy `Container<'short>` requirements? The answer depends on whether `Container` is covariant, contravariant, or invariant in its lifetime. Getting variance wrong leads to subtle unsoundness bugs — particularly with mutable references, which must be invariant. Rust computes variance automatically based on how lifetime parameters are used, and `PhantomData` lets you declare variance for types that need it explicitly.

## Learning Outcomes

- What covariant, contravariant, and invariant mean in terms of lifetime subtyping
- Why `&'a T` is covariant (longer lifetime usable as shorter)
- Why `&'a mut T` is invariant (cannot substitute a different lifetime without unsoundness)
- How `PhantomData<&'a T>` makes a wrapper covariant in `'a`
- How `PhantomData<&'a mut T>` makes a wrapper invariant in `'a`

## Rust Application

`Covariant<'a, T>` uses `PhantomData<&'a T>` — covariant in `'a` because `&T` is covariant. `Invariant<'a, T>` uses `PhantomData<&'a mut T>` — invariant in `'a` because `&mut T` is invariant. `covariant_demo` shows `&'static str` coercing to `&'short str`. `vec_covariance` returns `Vec<&'static str>` as `Vec<&'a str>` — safe because `Vec` inherits covariance from `&T`. `invariant_example` demonstrates `Cell<T>` which is invariant in `T`, preventing lifetime substitution.

Key patterns:
- `PhantomData<&'a T>` — covariant in `'a` and `T`
- `PhantomData<&'a mut T>` — invariant in `'a`, invariant in `T`
- `PhantomData<fn(T)>` — contravariant in `T`

## OCaml Approach

OCaml's type system has variance annotations on type parameters (`+'a` for covariant, `-'a` for contravariant, no annotation for invariant). However, these apply to type parameters, not lifetimes, since OCaml has no lifetime system:

```ocaml
type +'a covariant = Cov of 'a   (* covariant in 'a *)
type -'a contravariant = Contra of ('a -> unit)  (* contravariant *)
```

## Key Differences

1. **Automatic inference**: Rust infers variance from how parameters are used (in position, out position, both); OCaml requires explicit `+`/`-` annotations or infers from usage.
2. **Lifetime variance**: Rust's variance applies to lifetime parameters `'a` directly; OCaml has no lifetimes, so variance only applies to type parameters.
3. **Soundness guarantee**: Rust's automatic invariance for `&mut T` prevents a class of memory unsoundness bugs that would be possible with covariant mutable references; OCaml's GC eliminates the corresponding risks.
4. **PhantomData**: Rust uses `PhantomData` to declare variance for types that don't directly store `T` (e.g., raw pointer wrappers); OCaml achieves the same through direct type parameter annotation.

## Exercises

1. **Covariant wrapper**: Implement `struct ReadOnly<'a, T>(PhantomData<&'a T>)` and write a function demonstrating a `ReadOnly<'static, str>` can be used where `ReadOnly<'short, str>` is expected.
2. **Invariant wrapper**: Implement `struct Mutable<'a, T>(PhantomData<&'a mut T>)` and verify in a comment that substituting `'long` for `'short` would be rejected by the compiler.
3. **Contravariant usage**: Read about `PhantomData<fn(T) -> ()>` for contravariance and implement a `struct Sink<T>(PhantomData<fn(T)>)` that is contravariant in `T`.
