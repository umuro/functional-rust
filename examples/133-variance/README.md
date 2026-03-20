📖 **[View on hightechmind.io →](https://hightechmind.io/rust/133-variance)**

---

# Variance — Covariance, Contravariance, Invariance

## Problem Statement

Variance determines when a generic type `F<T>` can be substituted for `F<U>` given a subtype or lifetime relationship between `T` and `U`. Getting variance wrong leads to subtle memory safety bugs: if `&mut Vec<Dog>` were covariant in `Dog`, you could assign a `Cat` into a `Vec<Dog>` through it. Rust's borrow checker enforces correct variance automatically for most types, but `PhantomData` lets you declare the correct variance for raw-pointer wrappers where the compiler cannot infer it.

## Learning Outcomes

- Understand covariance, contravariance, and invariance in terms of lifetime subtyping
- Learn why `&'a T` is covariant in `'a` and `T`, but `&'a mut T` is invariant in `T`
- See how `PhantomData<T>` vs `PhantomData<fn(T)>` vs `PhantomData<*mut T>` declare different variances
- Understand why function arguments are contravariant while return types are covariant

## Rust Application

`Producer<T>` uses `PhantomData<T>` (covariant): a `Producer<&'long str>` can be used as a `Producer<&'short str>` because `'long` outlives `'short`. `Consumer<T>` uses `PhantomData<fn(T)>` (contravariant): a `Consumer<Animal>` can be used as a `Consumer<Dog>` because anything that handles any animal also handles dogs. `Invariant<T>` uses `PhantomData<*mut T>`: neither direction of substitution is safe for mutable access.

## OCaml Approach

OCaml's type system handles variance through type annotations on type parameters. A parameter declared as `+'a` is covariant, `-'a` is contravariant, and `'a` is invariant. For example, `type +'a producer = { produce: unit -> 'a }` marks `'a` as covariant. OCaml's variance annotations are checked by the compiler and documented in module signatures, providing similar guarantees to Rust's automatic variance inference.

## Key Differences

1. **Annotation style**: OCaml uses `+'a` / `-'a` in type declarations; Rust infers variance from field types and `PhantomData` usage.
2. **Mutable references**: OCaml's `ref` type is invariant like Rust's `&mut T`; both prevent unsound substitution through write access.
3. **Raw pointers**: Rust requires careful `PhantomData` choices for raw-pointer wrappers; OCaml has fewer raw-pointer types to worry about.
4. **Lifetime subtyping**: Rust's variance is primarily about lifetime subtyping; OCaml's is primarily about type subtyping (since it lacks explicit lifetimes).

## Exercises

1. Verify that `Producer<&'static str>` can be assigned to a variable of type `Producer<&'short str>` (covariance in action).
2. Implement a `Cell<T>` wrapper using `PhantomData<*mut T>` to make it invariant and explain why `get` and `set` together require invariance.
3. Write a test demonstrating that `Consumer<Animal>` can be used where `Consumer<Dog>` is expected, given `Animal` is a broader type.
