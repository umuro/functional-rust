# 200: Tagless Final

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Write a program expression once as a trait, then run it through any interpreter — evaluator, pretty-printer, optimizer — by changing a single type parameter.

## The Problem This Solves

You're building a small expression language: integers, booleans, arithmetic, comparisons, if-then-else. Maybe it's a query language, a rules engine, or a config DSL. You need to do multiple things with expressions: evaluate them to get values, pretty-print them for debugging, maybe type-check or optimize them.

The obvious approach: build an AST enum.

```rust
enum Expr {
    Int(i64),
    Add(Box<Expr>, Box<Expr>),
    IfExpr { cond: Box<Expr>, then: Box<Expr>, else_: Box<Expr> },
    // ...
}
```

Then write `eval(e: &Expr) -> Value` and `pretty(e: &Expr) -> String` as separate functions. Clean enough. But:

- Adding a new operation (say, `Mul`) means touching the `Expr` enum AND every function that matches on it
- The AST is always heap-allocated (nested `Box`) even when you only need evaluation
- Multiple passes over the tree — one per interpretation — even when you could do it in one go
- To add a new kind of interpreter, you have to pattern-match the entire AST again from scratch

This is the **Expression Problem**: it's hard to extend both the data types and the operations at the same time without touching existing code.

Tagless Final solves it by representing expressions not as an enum but as **trait method calls**. You define a trait with methods like `int`, `add`, `if_`. A program is a generic function that calls those methods. Each interpreter is a struct that implements the trait differently — `Eval` returns actual values, `Pretty` returns strings. Adding a new interpreter is adding a new `impl`. No AST, no boxing, no pattern matching. This exists to solve exactly that pain.

## The Intuition

Imagine you're writing a math expression: `if (3 + 4) ≤ (2 × 5) then 42 else 0`.

With an AST, you'd build a tree object, then pass it to different functions.

With tagless final, you instead write a generic function that describes the expression using whatever operations are available:

```rust
fn program<E: Expr>() -> E::Repr<i64> {
    E::if_(
        E::leq(E::add(E::int(3), E::int(4)), E::mul(E::int(2), E::int(5))),
        E::int(42),
        E::int(0),
    )
}
```

This function doesn't build an object. It calls trait methods. When you call `program::<Eval>()`, Rust substitutes `Eval` for `E` and the methods compute the result directly. When you call `program::<Pretty>()`, the methods build a string instead.

The key ingredient is `type Repr<T>` — a **Generic Associated Type (GAT)**, stable since Rust 1.65. It lets each interpreter declare what type it uses to represent a value of type `T`:

- `Eval`: `Repr<T> = T` — the representation of an `i64` is an `i64`
- `Pretty`: `Repr<T> = String` — the representation of _anything_ is a `String`

This is how one generic program works for both: the return type changes depending on which `E` you plug in.

## How It Works in Rust

**Step 1: The trait defines the DSL**

```rust
pub trait Expr {
    type Repr<T>;  // GAT: each interpreter picks its own representation type

    fn int(n: i64) -> Self::Repr<i64>;
    fn bool_val(b: bool) -> Self::Repr<bool>;
    fn add(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<i64>;
    fn mul(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<i64>;
    fn leq(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<bool>;
    fn if_<T>(c: Self::Repr<bool>, t: Self::Repr<T>, e: Self::Repr<T>) -> Self::Repr<T>;
}
```

This is the entire DSL definition. No AST enum. No boxing.

**Step 2: The `Eval` interpreter**

```rust
pub struct Eval;

impl Expr for Eval {
    type Repr<T> = T;  // "represent an i64 as an i64, a bool as a bool"

    fn int(n: i64) -> i64 { n }
    fn add(a: i64, b: i64) -> i64 { a + b }
    fn leq(a: i64, b: i64) -> bool { a <= b }
    fn if_<T>(c: bool, t: T, e: T) -> T { if c { t } else { e } }
    // ... etc
}
```

**Step 3: The `Pretty` interpreter**

```rust
pub struct Pretty;

impl Expr for Pretty {
    type Repr<T> = String;  // "represent everything as a string"

    fn int(n: i64) -> String { n.to_string() }
    fn add(a: String, b: String) -> String { format!("({a} + {b})") }
    fn leq(a: String, b: String) -> String { format!("({a} <= {b})") }
    fn if_<T>(c: String, t: String, e: String) -> String {
        format!("(if {c} then {t} else {e})")
    }
    // ... etc
}
```

**Step 4: One program, two runs**

```rust
// program is written ONCE
pub fn program<E: Expr>() -> E::Repr<i64> {
    E::if_(
        E::leq(E::add(E::int(3), E::int(4)), E::mul(E::int(2), E::int(5))),
        E::int(42),
        E::int(0),
    )
}

let result  = program::<Eval>();    // => 42
let printed = program::<Pretty>();  // => "(if ((3 + 4) <= (2 * 5)) then 42 else 0)"
```

No AST traversal. No heap allocation from the pattern. At `Eval` time, Rust compiles `program::<Eval>()` to the equivalent of just computing `42`.

**What would break without GATs?**

Before Rust 1.65, you couldn't write `type Repr<T>` in a trait. You'd have to use a separate type parameter on the trait itself (`trait Expr<Repr>`) which makes `if_<T>` impossible to express correctly — the return type of `if_` depends on `T`, not on a fixed `Repr`. GATs make this work.

## What This Unlocks

- **Zero-cost multiple interpretations**: No intermediate AST built on the heap. The evaluator compiles down to direct arithmetic. The pretty-printer compiles down to string concatenation. Each interpreter is as fast as hand-written code.
- **The Expression Problem solved**: Add new interpreters (a type-checker, an optimizer, a serializer) by adding new `impl Expr for MyInterpreter`. Existing code untouched. Add new operations by extending the trait — existing impls get a compile error until updated, which is _correct_ (you want to think about how each interpreter handles the new operation).
- **Real-world appearances**: This pattern underlies many Rust async/embedded frameworks. `Future` itself is tagless final in spirit — the "interpreter" is the runtime executor. Query builders, configuration DSLs, and constraint solvers use this to separate expression from evaluation.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type abstraction | `type 'a repr` (higher-kinded, first-class) | `type Repr<T>` (GAT, stable since 1.65) |
| Interpreter as | First-class module | Zero-sized struct + `impl` |
| Dispatch | Module functor application | Generic monomorphisation |
| `bool` constructor | `bool` (valid OCaml identifier) | Must rename to `bool_val` (`bool` is a keyword) |
| Runtime cost | Zero-cost | Zero-cost — no boxing, no dynamic dispatch |
| Adding interpreters | New module satisfying module type | New struct + `impl Expr for Struct` |
