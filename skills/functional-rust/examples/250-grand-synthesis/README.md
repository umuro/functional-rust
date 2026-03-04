# 250: Grand Synthesis

**Difficulty:** Expert  **Level:** Synthesis

All category theory concepts from this library — sum types, product types, Curry-Howard, profunctors, comonads, applicatives, limits/colimits — woven together into one validated data pipeline.

## The Problem This Solves

Every individual concept in this library solves a specific problem. But real software needs them all at once. A validation pipeline needs sum types for errors, product types for valid data, Curry-Howard for type-safe invariants, profunctors for transforming pipeline stages, comonads for context-aware processing, applicatives for accumulating all errors instead of stopping at the first, and limits/colimits to model how data flows and combines.

This capstone shows that the categories aren't separate techniques — they're facets of one coherent design philosophy. When you internalize all of them, you stop writing ad hoc validation code and start composing principled, type-safe transformations that the compiler verifies for you.

The pipeline validates raw user input (`RawInput`) through layered stages: parse → validate → enrich → format. Each stage uses the right categorical tool for its job.

## The Intuition

Think of the pipeline as a factory floor:

**Stage 1 — Sum + Product types**: The data model. `ValidationError` is a sum type (different error kinds). `ValidatedUser` is a product type (all fields present and valid). These two constructions encode all possible states of the data.

**Stage 2 — Curry-Howard**: Each validation rule is a *proof*. `fn validate_name(s: &str) -> Result<ValidatedName, ValidationError>` is a proof that "given a string, either I can prove it's a valid name, or I can prove it's invalid." The types ARE the specification.

**Stage 3 — Validated applicative**: Unlike `Result` (which stops at the first error), `Validated` *accumulates* all errors. This uses the applicative structure — combining independent validation results. Applicative = Day convolution specialized to the `Validated` functor. All fields are validated in parallel; all errors collected.

**Stage 4 — Profunctor**: The `Mapper<A, B>` profunctor adapts pipeline stages. A transformation that works on `Name` can be lifted to work on `User` via `dimap`. This is the lens structure: focus on a part, transform it, reconstruct the whole.

**Stage 5 — Comonad (Env)**: The `Env` comonad carries configuration alongside the data being processed. `extract` gets the value; `extend` runs a context-aware function over it. Configuration threading without `Rc<Config>` everywhere.

**Stage 6 — Limits/Colimits**: Products combine validations (validate name AND age AND email — all must succeed). Coproducts represent error alternatives (TooShort OR TooLong OR OutOfRange). The pipeline shape is a diagram; its categorical limit is the validated result type.

## How It Works in Rust

```rust
// SUM + PRODUCT TYPES — Data model
#[derive(Debug, Clone, PartialEq)]
enum ValidationError {        // SUM: one of these error kinds
    TooShort  { field: String, min: usize, got: usize },
    TooLong   { field: String, max: usize, got: usize },
    OutOfRange { field: String, min: i64, max: i64, got: i64 },
    // ...
}

#[derive(Debug, Clone)]
struct ValidatedUser {        // PRODUCT: ALL fields valid simultaneously
    name:  String,
    age:   u32,
    email: String,
    role:  Role,
}

// CURRY-HOWARD — Each validator is a proof
fn validate_name(s: &str) -> Result<String, ValidationError> {
    if s.len() < 2 { return Err(ValidationError::TooShort { .. }); }  // proof of invalidity
    if s.len() > 50 { return Err(ValidationError::TooLong { .. }); }
    Ok(s.trim().to_string())  // proof of validity
}

// VALIDATED APPLICATIVE — Accumulate all errors, not just first
#[derive(Debug, Clone)]
enum Validated<E, A> {
    Valid(A),
    Invalid(Vec<E>),  // collect ALL errors, not just one
}

impl<E: Clone, A> Validated<E, A> {
    // Applicative: combine two independent validations
    fn ap<B>(self, fb: Validated<E, B>) -> Validated<E, (A, B)>
    where E: Clone {
        match (self, fb) {
            (Validated::Valid(a),    Validated::Valid(b))    => Validated::Valid((a, b)),
            (Validated::Invalid(e1), Validated::Invalid(e2)) =>
                Validated::Invalid(e1.into_iter().chain(e2).collect()),  // merge errors
            (Validated::Invalid(e),  _) | (_, Validated::Invalid(e)) => Validated::Invalid(e),
        }
    }
}

// ENV COMONAD — Context-aware processing (config threading)
struct Env<E, A> { env: E, value: A }

impl<E: Clone, A> Env<E, A> {
    fn extract(self) -> A { self.value }   // counit: get the value
    fn extend<B>(self, f: impl Fn(Env<E, A>) -> B) -> Env<E, B>  // cobind
    where E: Clone, A: Clone {
        let env = self.env.clone();
        let b = f(self);
        Env { env, value: b }
    }
}

// PROFUNCTOR — Adapt pipeline stages
// dimap: transform both input and output of a processing step
let name_validator = Mapper::new(|raw: &str| validate_name(raw));
let user_name_lens = name_validator.dimap(
    |user: &RawInput| user.name.as_str(),  // extract name field
    |result| result,                        // pass through
);
```

The complete pipeline assembles these pieces: `RawInput` flows through profunctor-lifted validators, results are combined by the `Validated` applicative, configuration threads via the `Env` comonad, and the final `ValidatedUser` is the limit (product) of all successful validations.

## What This Unlocks

- **Algebraic architecture** — design systems as compositions of categorical structures. Each layer has laws; the composition inherits them. Correctness is structural, not incidental.
- **Error accumulation** — `Validated` applicative gives you all validation errors at once. Users see every problem in their form submission, not just the first field that failed.
- **Principled extensibility** — add a new field to `ValidatedUser`? Add a validator (a proof), plug it into the applicative combinator. The type system guides every step.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sum types | Polymorphic variants or `type t = ...` | `enum ValidationError { ... }` |
| Validated applicative | Module functor over `Applicative` | `enum Validated<E,A>` with `ap` method |
| Env comonad | Module `Env` with `extract`/`extend` | `struct Env<E,A>` with methods |
| Profunctor pipeline | First-class module passing | `Mapper<A,B>` with `dimap`/`first` |
| Error accumulation | `(module Applicative)` | `Vec<E>` inside `Invalid` variant |
