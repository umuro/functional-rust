#![allow(clippy::all)]
// Example 176: Introduction to GADTs
// Rust doesn't have native GADTs, but we can simulate them with
// PhantomData, traits, and sealed type-level markers.

use std::marker::PhantomData;

// === Approach 1: PhantomData + sealed traits to simulate GADTs ===

mod sealed {
    pub trait ExprType {}
    impl ExprType for i64 {}
    impl ExprType for bool {}
}

#[derive(Debug)]
enum ExprInner {
    Int(i64),
    Bool(bool),
    Add(Box<ExprInner>, Box<ExprInner>),
    If(Box<ExprInner>, Box<ExprInner>, Box<ExprInner>),
}

#[derive(Debug)]
struct Expr<T: sealed::ExprType> {
    inner: ExprInner,
    _phantom: PhantomData<T>,
}

impl Expr<i64> {
    fn int(n: i64) -> Self {
        Expr {
            inner: ExprInner::Int(n),
            _phantom: PhantomData,
        }
    }
    fn add(a: Expr<i64>, b: Expr<i64>) -> Self {
        Expr {
            inner: ExprInner::Add(Box::new(a.inner), Box::new(b.inner)),
            _phantom: PhantomData,
        }
    }
    fn eval(&self) -> i64 {
        match &self.inner {
            ExprInner::Int(n) => *n,
            ExprInner::Add(a, b) => {
                let a = Expr::<i64> {
                    inner: *a.clone(),
                    _phantom: PhantomData,
                };
                let b = Expr::<i64> {
                    inner: *b.clone(),
                    _phantom: PhantomData,
                };
                a.eval() + b.eval()
            }
            ExprInner::If(c, t, f) => {
                let c = Expr::<bool> {
                    inner: *c.clone(),
                    _phantom: PhantomData,
                };
                let t = Expr::<i64> {
                    inner: *t.clone(),
                    _phantom: PhantomData,
                };
                let f = Expr::<i64> {
                    inner: *f.clone(),
                    _phantom: PhantomData,
                };
                if c.eval() {
                    t.eval()
                } else {
                    f.eval()
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Expr<bool> {
    fn bool_val(b: bool) -> Self {
        Expr {
            inner: ExprInner::Bool(b),
            _phantom: PhantomData,
        }
    }
    fn eval(&self) -> bool {
        match &self.inner {
            ExprInner::Bool(b) => *b,
            _ => unreachable!(),
        }
    }
}

impl Clone for ExprInner {
    fn clone(&self) -> Self {
        match self {
            ExprInner::Int(n) => ExprInner::Int(*n),
            ExprInner::Bool(b) => ExprInner::Bool(*b),
            ExprInner::Add(a, b) => {
                ExprInner::Add(Box::new((**a).clone()), Box::new((**b).clone()))
            }
            ExprInner::If(a, b, c) => ExprInner::If(
                Box::new((**a).clone()),
                Box::new((**b).clone()),
                Box::new((**c).clone()),
            ),
        }
    }
}

fn if_expr(cond: Expr<bool>, then: Expr<i64>, else_: Expr<i64>) -> Expr<i64> {
    Expr {
        inner: ExprInner::If(
            Box::new(cond.inner),
            Box::new(then.inner),
            Box::new(else_.inner),
        ),
        _phantom: PhantomData,
    }
}

// === Approach 2: Enum-based with trait for evaluation ===

trait Eval {
    type Output;
    fn eval(&self) -> Self::Output;
}

struct IntLit(i64);
struct BoolLit(bool);
struct AddExpr(Box<dyn Eval<Output = i64>>, Box<dyn Eval<Output = i64>>);

impl Eval for IntLit {
    type Output = i64;
    fn eval(&self) -> i64 {
        self.0
    }
}

impl Eval for BoolLit {
    type Output = bool;
    fn eval(&self) -> bool {
        self.0
    }
}

impl Eval for AddExpr {
    type Output = i64;
    fn eval(&self) -> i64 {
        self.0.eval() + self.1.eval()
    }
}

// === Approach 3: Type-safe heterogeneous list with tuples ===

// Rust's type system naturally supports this via nested tuples
// (42, ("hello", (true, ())))  is the HList equivalent

trait HList {}
impl HList for () {}
impl<H, T: HList> HList for (H, T) {}

trait Head {
    type Item;
    fn head(&self) -> &Self::Item;
}

impl<H, T: HList> Head for (H, T) {
    type Item = H;
    fn head(&self) -> &H {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phantom_gadt_int() {
        assert_eq!(Expr::int(42).eval(), 42);
    }

    #[test]
    fn test_phantom_gadt_add() {
        assert_eq!(Expr::add(Expr::int(1), Expr::int(2)).eval(), 3);
    }

    #[test]
    fn test_phantom_gadt_bool() {
        assert_eq!(Expr::bool_val(true).eval(), true);
    }

    #[test]
    fn test_phantom_gadt_if() {
        let e = if_expr(Expr::bool_val(true), Expr::int(10), Expr::int(20));
        assert_eq!(e.eval(), 10);
        let e2 = if_expr(Expr::bool_val(false), Expr::int(10), Expr::int(20));
        assert_eq!(e2.eval(), 20);
    }

    #[test]
    fn test_trait_eval() {
        assert_eq!(IntLit(5).eval(), 5);
        assert_eq!(BoolLit(false).eval(), false);
        assert_eq!(AddExpr(Box::new(IntLit(3)), Box::new(IntLit(4))).eval(), 7);
    }

    #[test]
    fn test_hlist() {
        let hlist = (42, ("hello", (true, ())));
        assert_eq!(*hlist.head(), 42);
        assert_eq!(*hlist.1.head(), "hello");
        assert_eq!(*hlist.1 .1.head(), true);
    }
}
