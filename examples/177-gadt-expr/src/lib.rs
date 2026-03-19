// Example 177: GADT Typed Expression Evaluator
// Only well-typed expressions can be constructed

use std::fmt;

// === Approach 1: Trait-based typed expression tree ===
// Each node type is a separate struct; the trait ensures type safety

trait Expr: fmt::Debug {
    type Value;
    fn eval(&self) -> Self::Value;
    fn to_expr_string(&self) -> String;
}

#[derive(Debug)]
struct Lit(i64);

#[derive(Debug)]
struct BLit(bool);

#[derive(Debug)]
struct Add<A: Expr<Value = i64>, B: Expr<Value = i64>>(A, B);

#[derive(Debug)]
struct Mul<A: Expr<Value = i64>, B: Expr<Value = i64>>(A, B);

#[derive(Debug)]
struct Eq<A: Expr<Value = i64>, B: Expr<Value = i64>>(A, B);

#[derive(Debug)]
struct And<A: Expr<Value = bool>, B: Expr<Value = bool>>(A, B);

#[derive(Debug)]
struct Not<A: Expr<Value = bool>>(A);

#[derive(Debug)]
struct IfExpr<C: Expr<Value = bool>, T: Expr, F: Expr<Value = T::Value>>(C, T, F);

impl Expr for Lit {
    type Value = i64;
    fn eval(&self) -> i64 {
        self.0
    }
    fn to_expr_string(&self) -> String {
        self.0.to_string()
    }
}

impl Expr for BLit {
    type Value = bool;
    fn eval(&self) -> bool {
        self.0
    }
    fn to_expr_string(&self) -> String {
        self.0.to_string()
    }
}

impl<A: Expr<Value = i64>, B: Expr<Value = i64>> Expr for Add<A, B> {
    type Value = i64;
    fn eval(&self) -> i64 {
        self.0.eval() + self.1.eval()
    }
    fn to_expr_string(&self) -> String {
        format!(
            "({} + {})",
            self.0.to_expr_string(),
            self.1.to_expr_string()
        )
    }
}

impl<A: Expr<Value = i64>, B: Expr<Value = i64>> Expr for Mul<A, B> {
    type Value = i64;
    fn eval(&self) -> i64 {
        self.0.eval() * self.1.eval()
    }
    fn to_expr_string(&self) -> String {
        format!(
            "({} * {})",
            self.0.to_expr_string(),
            self.1.to_expr_string()
        )
    }
}

impl<A: Expr<Value = i64>, B: Expr<Value = i64>> Expr for Eq<A, B> {
    type Value = bool;
    fn eval(&self) -> bool {
        self.0.eval() == self.1.eval()
    }
    fn to_expr_string(&self) -> String {
        format!(
            "({} = {})",
            self.0.to_expr_string(),
            self.1.to_expr_string()
        )
    }
}

impl<A: Expr<Value = bool>, B: Expr<Value = bool>> Expr for And<A, B> {
    type Value = bool;
    fn eval(&self) -> bool {
        self.0.eval() && self.1.eval()
    }
    fn to_expr_string(&self) -> String {
        format!(
            "({} && {})",
            self.0.to_expr_string(),
            self.1.to_expr_string()
        )
    }
}

impl<A: Expr<Value = bool>> Expr for Not<A> {
    type Value = bool;
    fn eval(&self) -> bool {
        !self.0.eval()
    }
    fn to_expr_string(&self) -> String {
        format!("not({})", self.0.to_expr_string())
    }
}

impl<C: Expr<Value = bool>, T: Expr, F: Expr<Value = T::Value>> Expr for IfExpr<C, T, F> {
    type Value = T::Value;
    fn eval(&self) -> T::Value {
        if self.0.eval() {
            self.1.eval()
        } else {
            self.2.eval()
        }
    }
    fn to_expr_string(&self) -> String {
        format!(
            "if {} then {} else {}",
            self.0.to_expr_string(),
            self.1.to_expr_string(),
            self.2.to_expr_string()
        )
    }
}

// === Approach 2: Boxed dynamic dispatch for runtime-built trees ===

trait DynExprI64: fmt::Debug {
    fn eval(&self) -> i64;
}

struct DynLit(i64);
impl fmt::Debug for DynLit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl DynExprI64 for DynLit {
    fn eval(&self) -> i64 {
        self.0
    }
}

struct DynAdd(Box<dyn DynExprI64>, Box<dyn DynExprI64>);
impl fmt::Debug for DynAdd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?} + {:?})", self.0, self.1)
    }
}
impl DynExprI64 for DynAdd {
    fn eval(&self) -> i64 {
        self.0.eval() + self.1.eval()
    }
}

// === Approach 3: Enum-based with optimization pass ===

#[derive(Debug, Clone)]
enum IntExpr {
    Lit(i64),
    Add(Box<IntExpr>, Box<IntExpr>),
    Mul(Box<IntExpr>, Box<IntExpr>),
    IfB(Box<BoolExpr>, Box<IntExpr>, Box<IntExpr>),
}

#[derive(Debug, Clone)]
enum BoolExpr {
    Lit(bool),
    Eq(Box<IntExpr>, Box<IntExpr>),
    And(Box<BoolExpr>, Box<BoolExpr>),
    Not(Box<BoolExpr>),
}

impl IntExpr {
    fn eval(&self) -> i64 {
        match self {
            IntExpr::Lit(n) => *n,
            IntExpr::Add(a, b) => a.eval() + b.eval(),
            IntExpr::Mul(a, b) => a.eval() * b.eval(),
            IntExpr::IfB(c, t, f) => {
                if c.eval() {
                    t.eval()
                } else {
                    f.eval()
                }
            }
        }
    }

    fn optimize(self) -> Self {
        match self {
            IntExpr::Add(a, b) => {
                let a = a.optimize();
                let b = b.optimize();
                match (&a, &b) {
                    (IntExpr::Lit(0), _) => b,
                    (_, IntExpr::Lit(0)) => a,
                    (IntExpr::Lit(x), IntExpr::Lit(y)) => IntExpr::Lit(x + y),
                    _ => IntExpr::Add(Box::new(a), Box::new(b)),
                }
            }
            IntExpr::Mul(a, b) => {
                let a = a.optimize();
                let b = b.optimize();
                match (&a, &b) {
                    (IntExpr::Lit(0), _) | (_, IntExpr::Lit(0)) => IntExpr::Lit(0),
                    (IntExpr::Lit(1), _) => b,
                    (_, IntExpr::Lit(1)) => a,
                    (IntExpr::Lit(x), IntExpr::Lit(y)) => IntExpr::Lit(x * y),
                    _ => IntExpr::Mul(Box::new(a), Box::new(b)),
                }
            }
            other => other,
        }
    }
}

impl BoolExpr {
    fn eval(&self) -> bool {
        match self {
            BoolExpr::Lit(b) => *b,
            BoolExpr::Eq(a, b) => a.eval() == b.eval(),
            BoolExpr::And(a, b) => a.eval() && b.eval(),
            BoolExpr::Not(a) => !a.eval(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_eval() {
        assert_eq!(Lit(42).eval(), 42);
        assert_eq!(Add(Lit(1), Lit(2)).eval(), 3);
        assert_eq!(Mul(Lit(3), Lit(4)).eval(), 12);
        assert_eq!(Eq(Lit(1), Lit(1)).eval(), true);
        assert_eq!(Eq(Lit(1), Lit(2)).eval(), false);
        assert_eq!(And(BLit(true), BLit(true)).eval(), true);
        assert_eq!(Not(BLit(true)).eval(), false);
    }

    #[test]
    fn test_if_expr() {
        assert_eq!(IfExpr(BLit(true), Lit(10), Lit(20)).eval(), 10);
        assert_eq!(IfExpr(BLit(false), Lit(10), Lit(20)).eval(), 20);
    }

    #[test]
    fn test_pretty_print() {
        assert_eq!(Add(Lit(1), Lit(2)).to_expr_string(), "(1 + 2)");
        assert_eq!(Not(BLit(true)).to_expr_string(), "not(true)");
    }

    #[test]
    fn test_dynamic() {
        let d = DynAdd(Box::new(DynLit(10)), Box::new(DynLit(32)));
        assert_eq!(d.eval(), 42);
    }

    #[test]
    fn test_optimize() {
        let e = IntExpr::Add(Box::new(IntExpr::Lit(0)), Box::new(IntExpr::Lit(5)));
        assert_eq!(e.optimize().eval(), 5);

        let e = IntExpr::Mul(Box::new(IntExpr::Lit(0)), Box::new(IntExpr::Lit(999)));
        assert_eq!(e.optimize().eval(), 0);
    }
}
