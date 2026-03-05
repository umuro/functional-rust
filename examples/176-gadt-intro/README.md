📖 **[View on hightechmind.io →](https://hightechmind.io/rust/176-gadt-intro)**

---

# 176: Introduction to GADTs

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Simulate Generalized Algebraic Data Types in Rust using phantom types, so different enum variants can carry provably different return types.

## The Problem This Solves

In a normal Rust enum, every variant collapses to the same type. If you build an expression tree with `enum Expr { Int(i64), Bool(bool), Add(Box<Expr>, Box<Expr>) }`, the `eval` function must return `Box<dyn Any>` — you've thrown away type information and now need runtime casts. This means `Add(Bool(true), Int(5))` is representable, and its wrongness is only caught at runtime.

Generalized Algebraic Data Types (GADTs) solve this by letting each constructor carry its own type index. In OCaml, `Int : int -> int expr` is a constructor whose type says "I produce an `int expr`". The type system can then prove that `eval : 'a expr -> 'a` — call `eval` on an `int expr` and you get an `int`; call it on a `bool expr` and you get a `bool`. No casts, no `Any`.

The payoff is an expression tree where the type of the result is part of the node's type. You literally cannot write `Add(Bool(true), Int(5))` — the compiler rejects it before your program ever runs.

## The Intuition

Think of a vending machine with typed buttons. A standard enum is like a machine where all buttons return a bag of "something" — you have to reach in and hope it's what you wanted. A GADT is like a machine where button A returns a `Drink` and button B returns a `Snack`, guaranteed by the machine's own type. The *return type* of each button is built into the button's type signature.

In Rust we don't have native GADTs, but we can get close with **phantom types**: `Expr<T>` is a struct that carries a `PhantomData<T>`. The type `T` is never stored — it takes zero bytes — but the compiler tracks it statically. Smart constructors enforce the rules: `Expr::<i64>::int(5)` only compiles when building an integer node. `Expr::<bool>::bool_val(true)` only compiles for booleans. The type parameter flows through the tree and prevents mixing.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// The phantom type T says "what this expression evaluates to"
struct Expr<T> {
    inner: ExprInner,       // actual data, untyped
    _phantom: PhantomData<T>, // type tag — zero bytes
}

// Only specific impl blocks exist:
impl Expr<i64> {
    fn int(n: i64) -> Self { /* only callable for i64 */ }
    fn add(a: Expr<i64>, b: Expr<i64>) -> Expr<i64> { /* enforces both i64 */ }
    fn eval(&self) -> i64 { /* return type matches phantom */ }
}

impl Expr<bool> {
    fn bool_val(b: bool) -> Self { /* only callable for bool */ }
    fn eval(&self) -> bool { /* return type matches phantom */ }
}

// This compiles:
let e: Expr<i64> = Expr::add(Expr::int(3), Expr::int(4));
let result: i64 = e.eval(); // statically known: returns i64

// This does NOT compile — types mismatch:
// Expr::add(Expr::int(3), Expr::bool_val(true));
//                         ^^^^^^^^^^^^^^^^^ expected Expr<i64>, found Expr<bool>
```

The key insight: `PhantomData<T>` lets you brand a struct with a type without storing any value of that type. Different `impl` blocks on different `Expr<T>` specializations enforce GADT-like rules.

## What This Unlocks

- **Type-safe ASTs** for compilers, query builders, template engines — where you need to know the output type of each node statically.
- **Typed serialization/deserialization** — a schema type parameterized by its decoded value (`Schema<User>` decodes to `User`).
- **Embedded DSLs** where invalid programs are impossible to represent — the host language's type system rejects nonsense programs at the DSL level.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| GADT syntax | `type _ expr = Int : int -> int expr` | `struct Expr<T>` + `PhantomData<T>` + separate `impl` blocks |
| Type refinement in match | Automatic per branch — compiler knows `a = int` inside `Int` branch | No refinement; must use `unreachable!()` for provably-impossible arms |
| Enforcing constructor rules | Built into the type system via GADT constructor return types | Enforced by only exposing smart constructors (no direct struct construction) |
| Runtime overhead | None | None — `PhantomData` is zero-sized |
| Exhaustiveness | Full GADT checking | Pattern matching on inner untyped enum; type safety at construction time only |
