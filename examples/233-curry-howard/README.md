📖 **[View on hightechmind.io →](https://hightechmind.io/rust/233-curry-howard)**

---

# Curry-Howard Correspondence

## Problem Statement

The Curry-Howard correspondence is the profound observation that propositions in logic correspond to types in programming, and proofs correspond to programs. A function `A -> B` is a proof that "if A then B." A pair `(A, B)` is a proof of "A and B." A sum type `Either<A, B>` is a proof of "A or B." An impossible type (like `fn() -> !`) corresponds to a false proposition. Understanding this correspondence connects type system design to formal logic.

## Learning Outcomes

- Understand the correspondence: types are propositions, values are proofs
- Learn how function types correspond to implications, product types to conjunctions, sum types to disjunctions
- See how Rust's `!` (never) type corresponds to the proposition "false" (which has no proof)
- Appreciate how type-driven development and theorem proving are the same activity

## Rust Application

`fn and_intro<A, B>(a: A, b: B) -> (A, B) { (a, b) }` is the proof of "A ∧ B given A and B." `fn or_intro_left<A, B>(a: A) -> Either<A, B>` is the proof of "A ∨ B given A." `fn modus_ponens<A, B>(f: impl Fn(A) -> B, a: A) -> B { f(a) }` is the proof of "B given A → B and A." Functions of type `fn(A) -> B` are proofs of `A ⊢ B`. The `!` (never) type has no inhabitants — it is the type of false propositions.

## OCaml Approach

OCaml's type system supports the same correspondence:
```ocaml
let and_intro a b = (a, b)
let or_intro_left a = Either.Left a
let modus_ponens f a = f a
type void = |  (* no constructors — empty type, corresponds to False *)
```
OCaml's module system makes the correspondence explicit in type theory research tools (Coq, which started as a theorem prover for OCaml-like languages, directly implements this).

## Key Differences

1. **Never type**: Rust's `!` is built into the language as "never/bottom"; OCaml uses an empty variant type `type void = |` — both represent the false proposition.
2. **Dependent types**: The full Curry-Howard correspondence requires dependent types (Coq, Agda, Idris); Rust and OCaml only partially implement it.
3. **Proof relevance**: In pure type theory, only whether a proof exists matters; in Rust/OCaml, the specific program matters too — proofs are computations.
4. **Practical use**: The correspondence guides API design: a function returning `Option<T>` is a proof that "maybe T"; `Result<T, E>` is proof of "T or E".

## Exercises

1. Write the proof of `(A → B) → (B → C) → A → C` (function composition) as a Rust function.
2. Implement `contraposition: (A -> B) -> (B -> Void) -> (A -> Void)` — the contrapositive as a type-level proof.
3. Demonstrate that a function `fn absurd<T>(n: !) -> T` has no body to write — the `!` type has no inhabitants.
