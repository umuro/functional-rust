📖 **[View on hightechmind.io →](https://hightechmind.io/rust/234-propositions-types)**

---

# Propositions as Types

## Problem Statement

Extending the Curry-Howard correspondence, this example makes the type-as-proposition correspondence concrete with examples: conjunction (product types), disjunction (sum types), implication (function types), negation (function to `!`), and the universal and existential quantifiers (generic types and existential types). Seeing familiar type operations through the logical lens reveals why type system design and formal logic are the same discipline.

## Learning Outcomes

- See conjunction, disjunction, implication, and negation as concrete Rust types
- Understand universal quantification as generic type parameters: `∀T. P(T)` = `fn<T>(...)` 
- Understand existential quantification as `Box<dyn Trait>` or `impl Trait`
- Connect these to everyday programming: `Option` = "maybe A", `Result` = "A or E"

## Rust Application

Logical connectives as types:
- **Conjunction**: `struct And<A, B>(A, B)` — a value requires both A and B
- **Disjunction**: `enum Or<A, B> { Left(A), Right(B) }` — a value requires one of A or B
- **Implication**: `fn(A) -> B` — "if A is provable, then B is provable"
- **Negation**: `fn(A) -> !` — "A leads to a contradiction"
- **Universal**: `fn<T>(_: T) -> T` — holds for all types T
- **Existential**: `Box<dyn Trait>` — there exists some type satisfying Trait

## OCaml Approach

OCaml's type algebra mirrors this directly. The `type nonrec` keyword and abstract types provide the tools. Coq (the proof assistant) was originally designed with OCaml-like syntax specifically because of the Curry-Howard correspondence — proof terms in Coq are OCaml-like programs.

## Key Differences

1. **Existentials**: Rust's `Box<dyn Trait>` is a runtime existential; OCaml's first-class modules and GADTs provide more expressive existentials.
2. **Universal quantification**: Rust's generics are explicit (`fn<T>`); OCaml's polymorphism is implicit (inferred by type inference) — both express `∀T`.
3. **Negation**: Both use a function to an uninhabited type; Rust's `!` is built-in, OCaml uses `type void = |`.
4. **Proof relevance**: Rust and OCaml are "proof-relevant" (the computation matters); systems like Coq are proof-irrelevant (only inhabitation matters).

## Exercises

1. Write `double_negation_intro<A>(a: A) -> impl Fn(impl Fn(A) -> !) -> !` as a type-level proof.
2. Implement `De Morgan: (A -> !) -> (B -> !) -> Either<A, B> -> !` — De Morgan's law as a Rust function.
3. Show why `fn<A>(f: impl Fn(A) -> A) -> A` is uninhabitable (no value of any type can be returned without an `A` to start with).
