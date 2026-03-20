📖 **[View on hightechmind.io →](https://hightechmind.io/rust/740-phantom-variance-control)**

---

# 740-phantom-variance-control — Phantom Variance Control

## Problem Statement

Variance describes how subtyping relationships propagate through generic type constructors. In Rust, lifetimes and types can be covariant, contravariant, or invariant in a type parameter. Getting variance wrong leads to unsound code: a `Cell<&'static str>` being treated as `Cell<&'short str>` would allow writing a short-lived reference into a long-lived cell. `PhantomData` is the tool for explicitly setting variance when the compiler cannot infer it correctly from the struct fields, especially when raw pointers are involved.

## Learning Outcomes

- Understand covariance, contravariance, and invariance in Rust's type system
- Use `PhantomData<T>` for covariance (acts like owning a `T`)
- Use `PhantomData<fn(T)>` for contravariance (acts like a function consuming `T`)
- Use `PhantomData<Cell<T>>` or `PhantomData<*mut T>` for invariance
- Know when raw pointer fields make variance inference go wrong

## Rust Application

The example demonstrates three phantom data uses: `Covariant<T>` holds `PhantomData<T>`, meaning it's covariant in `T` (a `Covariant<&'long str>` can be used where `Covariant<&'short str>` is expected). `Contravariant<T>` holds `PhantomData<fn(T)>`, making it contravariant. `Invariant<T>` holds `PhantomData<Cell<T>>` or `*mut T`, preventing any automatic subtyping. These distinctions matter for generic collections holding raw pointers.

## OCaml Approach

OCaml uses variance annotations directly on type parameters: `type +'a t` (covariant), `type -'a t` (contravariant), and `type 'a t` (invariant by default). The compiler verifies that the annotation matches the actual usage. OCaml's explicit variance annotations are more readable than Rust's `PhantomData` trick. The `Base` library's `Container` types use precise variance annotations to enable safe covariant use.

## Key Differences

1. **Syntax**: OCaml uses `+'a` / `-'a` variance annotations directly; Rust encodes variance through the type of the `PhantomData` field.
2. **Clarity**: OCaml's annotations are self-documenting; Rust's `PhantomData<fn(T)>` for contravariance is a non-obvious idiom requiring documentation.
3. **Inference**: Rust infers variance automatically for structs without raw pointers; OCaml requires explicit annotations on abstract types.
4. **Soundness**: Both enforce variance at the type-checker level with no runtime cost.

## Exercises

1. Write a `Writer<T>` type that is contravariant in `T` — it accepts a `T` but produces nothing — and verify that `Writer<&'short str>` can be used as `Writer<&'static str>`.
2. Implement a `Buffer<T>` type that must be invariant in `T` because it both reads and writes `T`. Use `PhantomData<Cell<T>>` and explain why invariance is required.
3. Create a `Producer<T>` (covariant) and `Consumer<T>` (contravariant) and combine them into a `Channel<T>` that is invariant, demonstrating variance composition.
