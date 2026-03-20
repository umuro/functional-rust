📖 **[View on hightechmind.io →](https://hightechmind.io/rust/852-applicative-basics)**

---

# Applicative Functor Basics

## Problem Statement

Functors apply a plain function to a wrapped value: `map(f, Just(x)) = Just(f(x))`. But what if the function itself is wrapped? Applicative functors add `apply(Just(f), Just(x)) = Just(f(x))` — applying a wrapped function to a wrapped value. This enables combining multiple independent computations: parse two fields from a form, validate them independently, and combine results only if both succeed. Applicatives are strictly more powerful than functors but less powerful than monads (monads allow the second computation to depend on the first). In practice: form validation (`Validated`), parallel effects, command-line parsing (`clap`'s applicative API), and parser combinators all use applicative structure.

## Learning Outcomes

- Understand `pure(x)` = `Just(x)`: lifting a plain value into the applicative context
- Understand `apply(mf, mx)`: apply a wrapped function to a wrapped value
- Verify applicative laws: identity, composition, homomorphism, interchange
- Recognize the difference from monads: applicatives combine independent effects; monads sequence dependent effects
- Apply to: combining two `Option` values, two `Result` values without early-exit chaining

## Rust Application

```rust
impl<T> Maybe<T> {
    pub fn pure(x: T) -> Self { Maybe::Just(x) }
    pub fn apply<U, F: FnOnce(T) -> U>(self, mf: Maybe<F>) -> Maybe<U> {
        match (mf, self) {
            (Maybe::Just(f), Maybe::Just(x)) => Maybe::Just(f(x)),
            _ => Maybe::Nothing,
        }
    }
}
// Usage: combine two independent Options
let result = Maybe::pure(|x: i32| move |y: i32| x + y)
    .apply(Maybe::Just(3))  // Hmm - currying needed
```

Applicative in Rust requires currying to apply multi-argument functions wrapped in `Maybe`. The `apply` method takes `Maybe<F>` and `Maybe<T>` and produces `Maybe<U>`. Both the function and value must be `Just` — if either is `Nothing`, the result is `Nothing`. This differs from monadic `and_then`: applicative doesn't allow the second argument to depend on the first. Rust's `Option::zip` implements the applicative combining of two independent options.

## OCaml Approach

OCaml's applicative is expressed via a module signature: `module type APPLICATIVE = sig include FUNCTOR; val pure : 'a -> 'a t; val (<*>) : ('a -> 'b) t -> 'a t -> 'b t end`. The `<*>` operator applies wrapped functions. `let ( <*> ) mf mx = match mf, mx with Some f, Some x -> Some (f x) | _ -> None`. Currying in OCaml makes multi-argument applicative clean: `Some (+) <*> Some 3 <*> Some 4 = Some 7`. The `Applicative` interface underlies OCaml's Angstrom parser combinator library.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| `pure` | `Maybe::pure(x)` | `pure x` or `Some x` |
| `apply` | `.apply(mf)` method | `<*>` infix operator |
| Currying | Manual (closures returning closures) | Automatic |
| Multi-arg combine | `zip` or nested apply | `f <*> mx <*> my` |
| HKT limitation | Cannot express generic `Applicative` | Module functor can |
| Independent effects | Yes (no dependency between args) | Same |

## Exercises

1. Implement `map2(f, mx, my)` for `Maybe` using `apply` and `pure`: combine two independent Maybes with a binary function.
2. Verify the applicative identity law: `pure(id).apply(mx) == mx`.
3. Verify the homomorphism law: `pure(f).apply(pure(x)) == pure(f(x))`.
4. Implement applicative for `Result<T, E>`: both values must be `Ok`; if either is `Err`, return the first `Err`.
5. Compare applicative and monadic composition: show that `applicative(f, option1, option2)` cannot express "parse field2 differently based on field1's value."
