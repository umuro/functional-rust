# 138: Type Witnesses / GADT Encoding

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Use phantom type parameters as "witnesses" to prove to the compiler that a value has a specific type, enabling type-safe expression trees and typed heterogeneous maps.

## The Problem This Solves

Suppose you're building an expression evaluator. You have `IntLit`, `BoolLit`, `Add` (for ints), `Eq` (for ints, returns bool), and `If` (condition must be bool, branches must match). In an untyped `enum Expr`, nothing stops you from writing `Add(BoolLit(true), IntLit(5))` or `If(IntLit(1), ...)` — both are valid enum values. Type errors are only caught at eval time with panics.

What you want is an `IntExpr` that can only be constructed from int sub-expressions, and a `BoolExpr` that can only be constructed from bool sub-expressions. In Haskell/OCaml, GADTs (Generalized Algebraic Data Types) solve this natively: the constructors carry type information. Rust doesn't have GADTs, but you can simulate the key properties using phantom type parameters.

The same pattern applies to typed maps: a `TypedMap` where each key carries a type parameter, so `map.get(&age_key)` returns `Option<&i32>` and `map.get(&name_key)` returns `Option<&String>`, without any unsafe casts.

## The Intuition

A type witness is a phantom type parameter that "witnesses" — proves — some type-level fact about a value. `struct TypedExpr<T>` wraps an untyped `Expr` with a phantom `T`. The constructors enforce the right `T`: `fn int_lit(n: i32) -> TypedExpr<i32>` can only return an int expression. `fn eq(a: TypedExpr<i32>, b: TypedExpr<i32>) -> TypedExpr<bool>` takes two int expressions and produces a bool expression. The types flow through the tree.

The compiler now enforces: you can't call `add(bool_lit(true), int_lit(5))` because `add` requires two `TypedExpr<i32>` and `bool_lit` returns `TypedExpr<bool>`. Type errors in your expression language become compile-time errors in your Rust host code.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// The untyped core — for evaluation
enum Expr {
    IntLit(i32),
    BoolLit(bool),
    Add(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

// Typed wrapper — phantom T witnesses the expression's type
struct TypedExpr<T> {
    inner: Expr,
    _type: PhantomData<T>,  // T is not stored, just proves a type fact
}

// Typed constructors — the return type IS the type witness
fn int_lit(n: i32) -> TypedExpr<i32> {
    TypedExpr { inner: Expr::IntLit(n), _type: PhantomData }
}

fn bool_lit(b: bool) -> TypedExpr<bool> {
    TypedExpr { inner: Expr::BoolLit(b), _type: PhantomData }
}

// add: only accepts int expressions, produces int expression
fn add(a: TypedExpr<i32>, b: TypedExpr<i32>) -> TypedExpr<i32> {
    TypedExpr { inner: Expr::Add(Box::new(a.inner), Box::new(b.inner)), _type: PhantomData }
}

// eq: int expressions in, bool expression out — type relationship captured
fn eq(a: TypedExpr<i32>, b: TypedExpr<i32>) -> TypedExpr<bool> {
    TypedExpr { inner: Expr::Eq(Box::new(a.inner), Box::new(b.inner)), _type: PhantomData }
}

// if_then_else: condition must be bool, branches must have matching type T
fn if_then_else<T>(cond: TypedExpr<bool>, t: TypedExpr<T>, f: TypedExpr<T>) -> TypedExpr<T> {
    TypedExpr { inner: Expr::If(Box::new(cond.inner), Box::new(t.inner), Box::new(f.inner)), _type: PhantomData }
}
```

Usage:
```rust
// This compiles — valid expression
let e = if_then_else(
    eq(int_lit(1), int_lit(1)),  // bool condition ✓
    int_lit(42),                  // int branch ✓
    int_lit(0),                   // int branch ✓ (matches)
);

// This does NOT compile — type witness catches the error:
// let bad = add(bool_lit(true), int_lit(5));
// error: expected TypedExpr<i32>, found TypedExpr<bool>
```

Typed map — keys carry type information:
```rust
use std::any::Any;

struct TypedKey<T: 'static> {
    name: String,
    _type: PhantomData<T>,  // witness: this key maps to values of type T
}

struct TypedMap { entries: Vec<(String, Box<dyn Any>)> }

impl TypedMap {
    fn insert<T: 'static>(&mut self, key: &TypedKey<T>, value: T) {
        self.entries.push((key.name.clone(), Box::new(value)));
    }

    fn get<T: 'static>(&self, key: &TypedKey<T>) -> Option<&T> {
        self.entries.iter()
            .find(|(name, _)| name == &key.name)
            .and_then(|(_, val)| val.downcast_ref::<T>())  // type-safe downcast
    }
}

let age_key:  TypedKey<i32>    = TypedKey::new("age");
let name_key: TypedKey<String> = TypedKey::new("name");

map.get(&age_key)   // → Option<&i32>    — no cast needed, type is witnessed
map.get(&name_key)  // → Option<&String> — different type from same map
```

## What This Unlocks

- **Embedded DSLs** — type-safe query builders, formula languages, configuration schemas where illegal expressions are caught at the host-language type level.
- **Typed serialization keys** — configuration maps, event systems, component stores where each key statically determines the value type without runtime type tags.
- **Compile-time proof objects** — witness types can encode arbitrary type-level facts ("this list is sorted," "this value has been validated"), not just concrete types.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| GADT | Native: `type _ expr = IntLit : int -> int expr \| ...` — constructors refine the type index | Simulated: `struct TypedExpr<T>` with typed constructor functions |
| Type-safe eval | `let rec eval : type a. a expr -> a` — return type inferred from GADT index | Must eval untyped core; typed wrapper ensures only valid trees are built |
| Heterogeneous map | Module-based or `'a Hashtbl.t` with GADT keys | `Box<dyn Any>` + `downcast_ref` gated by `TypedKey<T>` phantom |
| Type flow | GADT refinement flows automatically | Phantom type propagates through typed constructor signatures |
