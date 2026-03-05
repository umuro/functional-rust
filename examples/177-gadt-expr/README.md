📖 **[View on hightechmind.io →](https://hightechmind.io/rust/177-gadt-expr)**

---

# 177: GADT Typed Expression Evaluator

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Build a complete typed expression tree where `eval` returns exactly the right Rust type for each node — integers for arithmetic, booleans for comparisons — enforced at compile time.

## The Problem This Solves

Writing an evaluator for a mini-language hits a wall fast. If you use a flat `enum Expr` for all nodes, `eval` has to return something like `Value::Int(i64) | Value::Bool(bool)` — a runtime sum type. Now adding two expressions requires a runtime check: "are both ints?" If not, you panic or propagate an error. The type system gives you nothing; it can't tell `Add(Int(3), Bool(true))` is nonsense until someone runs it.

The deeper problem: this mismatches how the language actually works. In any typed language, `3 + 4` has type `Int` and `3 == 4` has type `Bool`. That's a compile-time fact, not a runtime discovery. If our evaluator's type system can't encode it, we're carrying a permanent tax of defensive checks and potential runtime panics.

OCaml's GADTs solve this directly: `Add : int expr * int expr -> int expr` makes the compiler verify that both operands and the result are all `int expr`. No runtime check needed — the ill-typed expression can't be constructed. Rust achieves the same through a trait-per-node approach: each node type implements `Expr` with an associated `type Value`, and generic bounds enforce that `Add<A, B>` only exists when `A: Expr<Value = i64>`.

## The Intuition

A spreadsheet formula has a type: `=SUM(...)` produces a number, `=ISBLANK(...)` produces a boolean. You can't pass a boolean column into `SUM` — the spreadsheet rejects it before computing. That's what we want from our expression tree.

The Rust solution makes each node a separate struct: `Lit`, `Add`, `Eq`, `IfExpr`. Each implements a trait `Expr` with an associated `Value` type. `Add<A, B>` is only valid when both `A` and `B` have `Value = i64`. The compiler tracks these constraints through every generic instantiation. When you call `.eval()`, the return type is exactly the associated `Value` of that node type — statically known, no boxing needed.

## How It Works in Rust

```rust
// The core trait: every expression node knows its result type
trait Expr {
    type Value;
    fn eval(&self) -> Self::Value;
}

// Integer literal — evaluates to i64
struct Lit(i64);
impl Expr for Lit {
    type Value = i64;
    fn eval(&self) -> i64 { self.0 }
}

// Boolean literal — evaluates to bool
struct BLit(bool);
impl Expr for BLit {
    type Value = bool;
    fn eval(&self) -> bool { self.0 }
}

// Add — only compiles when BOTH operands evaluate to i64
struct Add<A: Expr<Value = i64>, B: Expr<Value = i64>>(A, B);
impl<A: Expr<Value = i64>, B: Expr<Value = i64>> Expr for Add<A, B> {
    type Value = i64;
    fn eval(&self) -> i64 { self.0.eval() + self.1.eval() }
}

// Equality check — takes two i64s, returns bool
struct Eq<A: Expr<Value = i64>, B: Expr<Value = i64>>(A, B);
impl<A: Expr<Value = i64>, B: Expr<Value = i64>> Expr for Eq<A, B> {
    type Value = bool;
    fn eval(&self) -> bool { self.0.eval() == self.1.eval() }
}

// Conditional — condition must be bool, branches must match in type
struct IfExpr<C: Expr<Value = bool>, T: Expr, F: Expr<Value = T::Value>>(C, T, F);
impl<C: Expr<Value = bool>, T: Expr, F: Expr<Value = T::Value>> Expr for IfExpr<C, T, F> {
    type Value = T::Value;
    fn eval(&self) -> Self::Value {
        if self.0.eval() { self.1.eval() } else { self.2.eval() }
    }
}

// Usage — fully static, no runtime type checks:
let e = IfExpr(BLit(true), Add(Lit(1), Lit(2)), Lit(99));
let result: i64 = e.eval(); // compiler knows this is i64

// This fails at compile time — you can't add a bool to an int:
// let bad = Add(Lit(5), BLit(true));
```

## What This Unlocks

- **Compiler middle-ends and IR lowering** — represent typed IR nodes where the output type is tracked statically, eliminating casting bugs across passes.
- **Query DSLs** — a typed query builder where `select(name_col)` returns `Query<String>` and `select(age_col)` returns `Query<i64>`, preventing type errors in query composition.
- **Optimization passes** — constant folding becomes safe and natural: `Add(Lit(3), Lit(4))` can fold to `Lit(7)` without worrying about mismatched types.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Single type for all nodes | `type _ expr` unified GADT | Separate struct per node (or separate int/bool enums) |
| Type-safe eval | Return type `a` refined per GADT constructor | Associated `type Value` on `Expr` trait, constrained by bounds |
| Conditional | `If : bool expr * 'a expr * 'a expr -> 'a expr` in one constructor | `IfExpr<C, T, F>` generic struct with `F: Expr<Value = T::Value>` |
| Pair/product types | `('a * 'b) expr` naturally | `PairExpr<A, B>` with `Value = (A::Value, B::Value)` |
| Runtime flexibility | Harder — GADTs are static | Can use `Box<dyn DynExpr>` for runtime-constructed trees (with boxing cost) |
