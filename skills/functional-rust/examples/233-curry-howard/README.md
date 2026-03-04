# 233: Curry-Howard Correspondence

**Difficulty:** ⭐⭐⭐  **Level:** Category Theory

Types are propositions. Programs are proofs. A function `fn(A) -> B` is a proof that "if A is true, then B is true."

## The Problem This Solves

Why does a type-safe Rust program have no undefined behavior? Why does the compiler guarantee memory safety? The answer isn't just "the borrow checker" — it's something deeper: the type system is a logic, and every well-typed program is a proof in that logic.

The Curry-Howard correspondence is the formal statement of this connection. Once you see it, the design of Rust's type system stops feeling arbitrary. `Option<T>` exists because "T might not be present" is a logical proposition with a precise type-level encoding. `Never` (`!`) exists because some computations truly never return, and that's a logical fact that deserves a type. `fn(A, B) -> C` exists because it's the type of proofs of "A AND B implies C."

This correspondence also explains why dependent types and proof assistants (like Coq, Lean, Agda) look so much like programming languages — they literally are.

## The Intuition

Every proposition in logic corresponds to a type. Every proof of a proposition corresponds to a value of that type.

| Logic | Rust |
|-------|------|
| Proposition `A` | Type `A` |
| Proof of `A` | Value of type `A` |
| `A AND B` | `(A, B)` — must have both |
| `A OR B` | `enum Either { Left(A), Right(B) }` |
| `A IMPLIES B` | `fn(A) -> B` — given a proof of A, produce a proof of B |
| `TRUE` | `()` — trivially inhabited, trivially provable |
| `FALSE` | `!` (Never) — uninhabited, unprovable |
| `NOT A` | `fn(A) -> !` — if A were true, you'd have a contradiction |

A type is "inhabited" if it has at least one value. In logic: a proposition is "provable" if it has at least one proof. An inhabited type = a provable proposition.

To prove `A AND B`, you must produce both an `A` and a `B`. That's why conjunction is a product type.  
To prove `A OR B`, you need only one: either an `A` or a `B`. That's why disjunction is a sum type.  
To prove `A IMPLIES B`, you need a function: given any proof of `A`, produce a proof of `B`. That's why implication is a function type.

## How It Works in Rust

```rust
// AND: proof requires both components
struct And<A, B>(A, B);

fn and_intro<A, B>(a: A, b: B) -> And<A, B> { And(a, b) }       // prove A∧B from A, B
fn and_elim_left<A: Clone, B>(p: &And<A, B>) -> A { p.0.clone() } // extract proof of A
fn and_elim_right<A, B: Clone>(p: &And<A, B>) -> B { p.1.clone() }

// OR: proof requires one component
enum Or<A, B> { Left(A), Right(B) }

fn or_intro_left<A, B>(a: A) -> Or<A, B>  { Or::Left(a) }  // prove A∨B from just A
fn or_intro_right<A, B>(b: B) -> Or<A, B> { Or::Right(b) }

// Elimination: given A∨B, plus f:A->C and g:B->C, prove C
fn or_elim<A, B, C>(or: Or<A, B>, f: impl Fn(A)->C, g: impl Fn(B)->C) -> C {
    match or { Or::Left(a) => f(a), Or::Right(b) => g(b) }
}

// IMPLICATION: a function from proof of A to proof of B
fn modus_ponens<A, B>(f: impl Fn(A) -> B, a: A) -> B { f(a) }

// Actual theorems as functions:
fn and_comm<A: Clone, B: Clone>(proof: And<A, B>) -> And<B, A> {
    And(proof.1, proof.0)  // proof that A∧B → B∧A
}
fn identity_proof<A>(a: A) -> A { a }  // proof that A → A (tautology)
```

The type `fn(A) -> B` is *only* inhabited if B is derivable from A. A `fn(i32) -> String` exists; a `fn(String) -> i32` also exists (parsing may fail, hence `Result`). But a `fn(()) -> !` cannot be written (there's no value of type `!` to return).

## What This Unlocks

- **Type safety = logical soundness** — every well-typed program is a proof; the compiler is the proof checker. Undefined behavior would be a logical contradiction — and logically inconsistent systems prove everything, which is useless.
- **`Option<T>` as existential quantifier** — `Option<T>` is the type of "there might be a `T`." The compiler forces you to handle `None`, just as logic forces you to handle the case where the existential is empty.
- **Designing types as propositions** — when you write a type, you're stating a proposition. When you write a function, you're proving it. Ask: "what does this type *mean* logically?" and the right type naturally emerges.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Conjunction | Tuple `(a * b)` | Struct or tuple `(A, B)` |
| Disjunction | Sum type `a \| b` | `enum Or<A, B>` / `Result<A, B>` |
| Implication | Function `a -> b` | `fn(A) -> B` |
| False / Bottom | Polymorphic `'a` (empty type) | `!` / `std::convert::Infallible` |
| Negation | `'a -> 'b` (bottom encoding) | `fn(A) -> !` (function to Never) |
