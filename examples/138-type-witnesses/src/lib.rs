#![allow(clippy::all)]
//! Example 138: Type Witnesses / GADT Encoding
//!
//! Simulates OCaml's GADTs in Rust using phantom type parameters.
//! A phantom `T` on `TypedExpr<T>` *witnesses* the expression's result type
//! at compile time — the compiler rejects ill-typed trees before runtime.
//!
//! Two demonstrations:
//!   1. A typed expression tree (GADT-style via smart constructors + PhantomData)
//!   2. A typed heterogeneous map (each key witnesses its value's type)

use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;

// ── Approach 1: GADT-style typed expression tree ───────────────────────────
//
// OCaml: `type _ expr = IntLit : int -> int expr | Add : int expr * int expr -> int expr | ...`
// Rust:  wrap a `RawExpr` in `TypedExpr<T>`; smart constructors set T correctly.
//
// The `unreachable!` branches in eval are dead code by construction —
// the phantom-type invariant guarantees only valid combinations reach `eval`.

/// Untyped inner AST — private; never exposed directly.
enum RawExpr {
    IntLit(i32),
    BoolLit(bool),
    Add(Box<RawExpr>, Box<RawExpr>),
    Eq(Box<RawExpr>, Box<RawExpr>),
    If(Box<RawExpr>, Box<RawExpr>, Box<RawExpr>),
}

/// A typed expression: `T` is the *witness* for the result type.
///
/// Users can only construct values through the smart constructors below,
/// which enforce the correct `T` — exactly what OCaml GADTs guarantee via
/// constructor type indices.
pub struct TypedExpr<T> {
    raw: RawExpr,
    _marker: PhantomData<T>,
}

// ── Smart constructors (the only way to build `TypedExpr<T>`) ──────────────

/// An integer literal expression.
pub fn int_lit(n: i32) -> TypedExpr<i32> {
    TypedExpr {
        raw: RawExpr::IntLit(n),
        _marker: PhantomData,
    }
}

/// A boolean literal expression.
pub fn bool_lit(b: bool) -> TypedExpr<bool> {
    TypedExpr {
        raw: RawExpr::BoolLit(b),
        _marker: PhantomData,
    }
}

/// Addition of two integer expressions — both arguments *must* be `i32`.
pub fn add(a: TypedExpr<i32>, b: TypedExpr<i32>) -> TypedExpr<i32> {
    TypedExpr {
        raw: RawExpr::Add(Box::new(a.raw), Box::new(b.raw)),
        _marker: PhantomData,
    }
}

/// Equality test on two integer expressions — result is `bool`.
pub fn eq_expr(a: TypedExpr<i32>, b: TypedExpr<i32>) -> TypedExpr<bool> {
    TypedExpr {
        raw: RawExpr::Eq(Box::new(a.raw), Box::new(b.raw)),
        _marker: PhantomData,
    }
}

/// Conditional: condition must be `bool`, branches must share the same type `T`.
pub fn if_expr<T>(
    cond: TypedExpr<bool>,
    then_branch: TypedExpr<T>,
    else_branch: TypedExpr<T>,
) -> TypedExpr<T> {
    TypedExpr {
        raw: RawExpr::If(
            Box::new(cond.raw),
            Box::new(then_branch.raw),
            Box::new(else_branch.raw),
        ),
        _marker: PhantomData,
    }
}

// ── Evaluation ─────────────────────────────────────────────────────────────
//
// We provide concrete `eval` impls for each supported result type rather than
// a generic trait-bounded impl, so the internal `RawExpr` type stays private
// and we avoid the `private_bounds` lint.

fn eval_i32(raw: &RawExpr) -> i32 {
    match raw {
        RawExpr::IntLit(n) => *n,
        RawExpr::Add(a, b) => eval_i32(a) + eval_i32(b),
        RawExpr::If(cond, t, f) => {
            if eval_bool(cond) {
                eval_i32(t)
            } else {
                eval_i32(f)
            }
        }
        _ => unreachable!("type witness invariant: not an i32 expression"),
    }
}

fn eval_bool(raw: &RawExpr) -> bool {
    match raw {
        RawExpr::BoolLit(b) => *b,
        RawExpr::Eq(a, b) => eval_i32(a) == eval_i32(b),
        RawExpr::If(cond, t, f) => {
            if eval_bool(cond) {
                eval_bool(t)
            } else {
                eval_bool(f)
            }
        }
        _ => unreachable!("type witness invariant: not a bool expression"),
    }
}

impl TypedExpr<i32> {
    /// Evaluate, returning the witnessed `i32`.
    pub fn eval(&self) -> i32 {
        eval_i32(&self.raw)
    }
}

impl TypedExpr<bool> {
    /// Evaluate, returning the witnessed `bool`.
    pub fn eval(&self) -> bool {
        eval_bool(&self.raw)
    }
}

// ── Approach 2: Typed heterogeneous map ────────────────────────────────────
//
// A `TypedKey<T>` witnesses that the value stored under this key has type `T`.
// The caller sees `Option<&T>` from `get` — no visible downcasting.

/// A key whose phantom `T` witnesses the type of its associated value.
pub struct TypedKey<T: 'static> {
    name: &'static str,
    _marker: PhantomData<T>,
}

impl<T: 'static> TypedKey<T> {
    /// Create a new key with a unique string name.
    /// Keys with different names are always distinct, even for the same `T`.
    pub const fn new(name: &'static str) -> Self {
        TypedKey {
            name,
            _marker: PhantomData,
        }
    }
}

/// A heterogeneous map: each value may have a different type,
/// determined at compile time by the `TypedKey<T>` used to access it.
#[derive(Default)]
pub struct TypedMap {
    inner: HashMap<&'static str, Box<dyn Any>>,
}

impl TypedMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a value; the key's phantom type ensures `value: T`.
    pub fn insert<T: Any>(&mut self, key: &TypedKey<T>, value: T) {
        self.inner.insert(key.name, Box::new(value));
    }

    /// Retrieve a reference; returns `Option<&T>` — type determined by the key.
    pub fn get<T: Any>(&self, key: &TypedKey<T>) -> Option<&T> {
        self.inner.get(key.name)?.downcast_ref::<T>()
    }

    /// Remove and return the value, if present.
    pub fn remove<T: Any>(&mut self, key: &TypedKey<T>) -> Option<T> {
        self.inner
            .remove(key.name)
            .and_then(|v| v.downcast::<T>().ok())
            .map(|b| *b)
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Expression tree ────────────────────────────────────────────────────

    #[test]
    fn test_int_literal() {
        assert_eq!(int_lit(42).eval(), 42);
        assert_eq!(int_lit(0).eval(), 0);
        assert_eq!(int_lit(-7).eval(), -7);
    }

    #[test]
    fn test_bool_literal() {
        assert!(bool_lit(true).eval());
        assert!(!bool_lit(false).eval());
    }

    #[test]
    fn test_add() {
        assert_eq!(add(int_lit(3), int_lit(4)).eval(), 7);
        assert_eq!(add(int_lit(0), int_lit(0)).eval(), 0);
        // Nested: (1 + 2) + (3 + 4)
        assert_eq!(
            add(add(int_lit(1), int_lit(2)), add(int_lit(3), int_lit(4))).eval(),
            10
        );
    }

    #[test]
    fn test_eq_expr() {
        assert!(eq_expr(int_lit(5), int_lit(5)).eval());
        assert!(!eq_expr(int_lit(5), int_lit(6)).eval());
        assert!(eq_expr(add(int_lit(1), int_lit(2)), int_lit(3)).eval());
    }

    #[test]
    fn test_if_int_branches() {
        assert_eq!(if_expr(bool_lit(true), int_lit(10), int_lit(20)).eval(), 10);
        assert_eq!(
            if_expr(bool_lit(false), int_lit(10), int_lit(20)).eval(),
            20
        );
    }

    #[test]
    fn test_if_bool_branches() {
        assert!(!if_expr(bool_lit(true), bool_lit(false), bool_lit(true)).eval());
        assert!(if_expr(bool_lit(false), bool_lit(false), bool_lit(true)).eval());
    }

    #[test]
    fn test_complex_expression() {
        // if (1 + 2 == 3) then 100 else 0  →  100
        let cond = eq_expr(add(int_lit(1), int_lit(2)), int_lit(3));
        let expr = if_expr(cond, int_lit(100), int_lit(0));
        assert_eq!(expr.eval(), 100);
    }

    // ── Typed map ──────────────────────────────────────────────────────────

    #[test]
    fn test_typed_map_insert_and_get() {
        static AGE: TypedKey<i32> = TypedKey::new("age");
        static NAME: TypedKey<String> = TypedKey::new("name");

        let mut map = TypedMap::new();
        map.insert(&AGE, 30);
        map.insert(&NAME, "Alice".to_string());

        assert_eq!(map.get(&AGE), Some(&30));
        assert_eq!(map.get(&NAME), Some(&"Alice".to_string()));
    }

    #[test]
    fn test_typed_map_missing_key() {
        static KEY: TypedKey<i32> = TypedKey::new("missing_key_138");
        let map = TypedMap::new();
        assert_eq!(map.get(&KEY), None);
    }

    #[test]
    fn test_typed_map_overwrite() {
        static KEY: TypedKey<i32> = TypedKey::new("counter_138");
        let mut map = TypedMap::new();
        map.insert(&KEY, 1);
        map.insert(&KEY, 2);
        assert_eq!(map.get(&KEY), Some(&2));
    }

    #[test]
    fn test_typed_map_remove() {
        static KEY: TypedKey<String> = TypedKey::new("greeting_138");
        let mut map = TypedMap::new();
        map.insert(&KEY, "hello".to_string());
        assert_eq!(map.remove(&KEY), Some("hello".to_string()));
        assert_eq!(map.get(&KEY), None);
    }
}
