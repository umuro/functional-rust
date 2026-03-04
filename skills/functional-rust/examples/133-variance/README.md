# 133: Variance

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Understand how Rust's type system decides when one generic type can substitute for another — and how to control it with `PhantomData`.

## The Problem This Solves

You have a function that expects a `&'short str` — a string reference valid for a short time. Can you pass a `&'long str` — a reference valid longer? Intuitively yes: something that lives longer is safe to use where something shorter is needed. But what about `&mut Vec<Dog>` where `&mut Vec<Animal>` is expected? That turns out to be *unsafe* — if the function pushes a `Cat` into `Animal`, your `Vec<Dog>` now contains a Cat.

These are variance rules: when can you substitute a type for another in a generic context? The compiler infers variance automatically from how type parameters appear — but when you're working with raw pointers, `PhantomData`, or designing APIs, you may need to control it explicitly. Getting it wrong silently breaks soundness.

For API design, variance determines whether a `Producer<Dog>` can be used as a `Producer<Animal>` (covariant — yes, a producer of specific things can produce general things) or whether a `Consumer<Animal>` can be used as a `Consumer<Dog>` (contravariant — yes, something that consumes any animal can consume a dog). Understanding this lets you design correct, flexible APIs.

## The Intuition

**Covariance**: if `Dog` is a subtype of `Animal`, then `Producer<Dog>` is a subtype of `Producer<Animal>`. Producers can be specialized. Think: a function that *returns* something. It's safe to get back a `Dog` when you expected an `Animal`.

**Contravariance**: `Consumer<Animal>` is a subtype of `Consumer<Dog>`. Consumers can be generalized. Think: a function that *accepts* something. A function accepting any `Animal` is safe to use where a function accepting a `Dog` is expected — it handles everything dogs do and more.

**Invariance**: neither direction is safe. Mutation creates invariance — if you can read *and* write, you need the exact type.

In Rust, variance is primarily about lifetimes. `&'long T` is covariant in `'long` — a longer lifetime can be used where a shorter one is expected. `&mut T` is invariant in `T` — you can't use `&mut SubType` where `&mut SuperType` is expected, because the function could write a `SuperType` value that breaks the `SubType` guarantee.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// Variance is controlled by how T appears in PhantomData:

struct Covariant<T> {
    _phantom: PhantomData<T>,        // covariant in T (same as &T)
}

struct Contravariant<T> {
    _phantom: PhantomData<fn(T)>,    // contravariant in T (same as fn(T) -> ())
}

struct Invariant<T> {
    _phantom: PhantomData<fn(T) -> T>,  // invariant in T
}
```

Practical example — contravariant predicate (can `contramap`):
```rust
struct Predicate<T> {
    check: Box<dyn Fn(&T) -> bool>,
}

impl<T: 'static> Predicate<T> {
    // Contramap: given a way to turn U into T, a predicate on T becomes a predicate on U
    // This is the "contravariant functor" — adapt a predicate to a different input type
    fn contramap<U: 'static>(self, f: impl Fn(&U) -> T + 'static) -> Predicate<U> {
        Predicate {
            check: Box::new(move |u| (self.check)(&f(u))),
        }
    }
}

// Usage: adapt an int predicate to work on strings
let is_positive = Predicate::new(|x: &i32| *x > 0);

// A string "has positive length" if its length is positive
// We contramap: provide a way to extract the length (i32) from a String
let has_chars = is_positive.contramap(|s: &String| s.len() as i32);
// has_chars is a Predicate<String>
```

Practical example — covariant producer (can `map`):
```rust
struct Lazy<T> {
    produce: Box<dyn Fn() -> T>,
}

impl<T: 'static> Lazy<T> {
    // Map: transform the output — covariant functor
    fn map<U: 'static>(self, f: impl Fn(T) -> U + 'static) -> Lazy<U> {
        Lazy { produce: Box::new(move || f((self.produce)())) }
    }
}

let int_lazy = Lazy::new(|| 42);
let str_lazy = int_lazy.map(|x| format!("value: {}", x));  // Lazy<String>
```

## What This Unlocks

- **Correct PhantomData usage** — when writing `unsafe` code with raw pointers, choosing the right PhantomData variance prevents the compiler from accepting unsound substitutions.
- **Functor/contramap patterns** — covariant types naturally support `map`; contravariant types support `contramap`; understanding variance tells you which one applies.
- **Lifetime correctness** — variance rules explain why `&mut Vec<&'a str>` is invariant in `'a` and why the borrow checker occasionally needs a nudge via explicit lifetime annotations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Variance annotation | Explicit: `type +'a producer`, `type -'a consumer` in type declarations | Inferred from structure; controlled via `PhantomData<T>`, `PhantomData<fn(T)>` |
| Covariant type | `type +'a producer = { produce : unit -> 'a }` | `PhantomData<T>` or `PhantomData<&'a T>` |
| Contravariant type | `type -'a consumer = { consume : 'a -> unit }` | `PhantomData<fn(T)>` |
| Contramap | `let contramap f pred = fun x -> pred (f x)` | `fn contramap<U>(self, f: impl Fn(&U) -> T) -> Predicate<U>` |
