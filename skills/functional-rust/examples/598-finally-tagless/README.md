# 598: Finally Tagless Style

**Difficulty:** 4  **Level:** Advanced

Define a language as a trait, then run the same expression through any number of interpreters — evaluator, pretty-printer, operation counter — without changing the expression.

## The Problem This Solves

Say you have arithmetic expressions and you want to do three things with them: evaluate them to a number, print them as a human-readable string, and count how many operations they contain. The classic approach: build an `enum Expr { Lit(i64), Add(Box<Expr>, Box<Expr>), ... }` then write three separate functions that pattern-match on it.

This works — until you want to add a new kind of expression. Add `Sub` to the enum, and suddenly _every_ match expression in your codebase is incomplete. The compiler will catch it, but you still have to touch every interpreter. In a small project that's fine. In a large codebase with many passes (evaluation, optimization, type-checking, code generation, pretty-printing) it becomes a maintenance burden.

The other direction is worse: add a new _interpretation_ of existing expressions. Maybe you want to count expressions instead of evaluating them. With an AST enum, you write a new function from scratch, re-matching on every case. There's no way to share structure.

The "finally tagless" style (the name comes from a paper by Kiselyov et al.) flips the model. Instead of a data type with constructors, you define a **trait with methods**. The expression is a generic function that calls those methods. Each interpreter is a struct that implements the trait with a different concrete type for the result. Adding an interpreter? New `impl`. Adding an expression form? New method in the trait (with a compile error for every impl that needs updating — which is the right behavior). This pattern exists to solve exactly that pain.

## The Intuition

Think of a universal remote control. You press "volume up." The remote doesn't know if you're controlling a TV, a soundbar, or an amp. It just calls `volume_up()` on whatever device is connected. The _command_ is defined once; the _behavior_ depends on the device.

Tagless final works the same way. The "remote control" is the generic function — it calls `lit()`, `add()`, `neg()` without knowing what they return. Each "device" is an interpreter:

```rust
trait Expr<R> {
    fn lit(n: i64) -> R;     // "make a literal number"
    fn add(l: R, r: R) -> R; // "add two things"
    fn mul(l: R, r: R) -> R; // "multiply two things"
    fn neg(x: R) -> R;       // "negate something"
}
```

The type parameter `R` is what the interpreter produces. Different interpreters pick different `R`:

- `EvalInterp`: `R = i64` — the result is an actual number
- `PrintInterp`: `R = String` — the result is a readable string
- `CountInterp`: `R = usize` — the result is a count of operations

The program is written _once_ using the trait methods:

```rust
// 3*4 + (-2)
fn program<R, E: Expr<R>>() -> R {
    E::add(
        E::mul(E::lit(3), E::lit(4)),
        E::neg(E::lit(2)),
    )
}
```

When you call `program::<i64, EvalInterp>()`, Rust substitutes `i64` for `R` and `EvalInterp` for `E`. The whole thing compiles to direct arithmetic. No AST is ever built.

## How It Works in Rust

**Step 1: Define the language as a trait**

```rust
trait Expr<R> {
    fn lit(n: i64) -> R;
    fn add(l: R, r: R) -> R;
    fn mul(l: R, r: R) -> R;
    fn neg(x: R) -> R;
}
```

This is your entire language definition. No `enum`, no `Box`, no heap.

**Step 2: Three interpreters**

```rust
// Interpreter 1: evaluate to i64
struct EvalInterp;
impl Expr<i64> for EvalInterp {
    fn lit(n: i64)         -> i64 { n }
    fn add(l: i64, r: i64) -> i64 { l + r }
    fn mul(l: i64, r: i64) -> i64 { l * r }
    fn neg(x: i64)         -> i64 { -x }
}

// Interpreter 2: pretty-print to String
struct PrintInterp;
impl Expr<String> for PrintInterp {
    fn lit(n: i64)             -> String { format!("{}", n) }
    fn add(l: String, r: String) -> String { format!("({}+{})", l, r) }
    fn mul(l: String, r: String) -> String { format!("({}*{})", l, r) }
    fn neg(x: String)            -> String { format!("(-{})", x) }
}

// Interpreter 3: count operations (literals don't count)
struct CountInterp;
impl Expr<usize> for CountInterp {
    fn lit(_: i64)             -> usize { 0 }          // literals aren't "operations"
    fn add(l: usize, r: usize) -> usize { 1 + l + r }  // add itself is 1 op
    fn mul(l: usize, r: usize) -> usize { 1 + l + r }
    fn neg(x: usize)           -> usize { 1 + x }
}
```

**Step 3: One expression, three runs**

```rust
fn program<R, E: Expr<R>>() -> R {
    E::add(
        E::mul(E::lit(3), E::lit(4)),  // 3*4 = 12
        E::neg(E::lit(2)),             // -2
    )
}
// 3*4 + (-2) = 10

println!("{}", program::<i64,    EvalInterp>());   // => 10
println!("{}", program::<String, PrintInterp>());  // => "((3*4)+(-2))"
println!("{}", program::<usize,  CountInterp>());  // => 3 (mul, add, neg)
```

**Adding a fourth interpreter**: Create a new struct, `impl Expr<YourType> for YourStruct`, done. The program function doesn't change. The other interpreters don't change.

**Adding a new operation** (say, `sub`): Add `fn sub(l: R, r: R) -> R` to the trait. Every existing `impl` breaks at compile time — which is exactly right. You want each interpreter to consciously handle subtraction.

## What This Unlocks

- **Open extension**: New interpreters without touching existing code. Add a type-checker, an optimizer, a serializer — each is a new struct + impl, nothing else modified.
- **Zero-cost abstraction**: Rust monomorphises — `program::<i64, EvalInterp>()` compiles to `10i64` with no overhead. No virtual dispatch, no heap allocation from the pattern.
- **Real-world appearances**: The Rust `async`/`await` system is tagless final in spirit — `Future` defines the "trait" and different executors (Tokio, async-std, smol) are the interpreters. Compile-time DSLs in embedded Rust, query builders, and constraint solvers use this pattern to separate language definition from execution.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Language definition | Module type with `type 'a repr` (HKT) | Trait with type parameter `R` |
| Interpreter | Module satisfying the module type | Struct implementing `Expr<R>` |
| Multiple interpreters | Different modules | Different `impl Expr<R>` for different `R` |
| Extension | New module | New struct + impl |
| Type safety | Phantom types in HKT | Generics — compiler enforces it |
| Runtime cost | Zero — direct calls | Zero — monomorphised by rustc |
