#![allow(clippy::all)]
// Example 224: Mutumorphism — Genuinely Mutual Recursion

// mutu: two folds that depend on EACH OTHER

#[derive(Debug)]
enum NatF<A> {
    ZeroF,
    SuccF(A),
}

impl<A> NatF<A> {
    fn map_ref<B>(&self, f: impl Fn(&A) -> B) -> NatF<B> {
        match self {
            NatF::ZeroF => NatF::ZeroF,
            NatF::SuccF(a) => NatF::SuccF(f(a)),
        }
    }
}

#[derive(Debug, Clone)]
struct FixNat(Box<NatF<FixNat>>);

fn zero() -> FixNat {
    FixNat(Box::new(NatF::ZeroF))
}
fn succ(n: FixNat) -> FixNat {
    FixNat(Box::new(NatF::SuccF(n)))
}
fn nat(n: u32) -> FixNat {
    (0..n).fold(zero(), |acc, _| succ(acc))
}

fn mutu<A: Clone, B: Clone>(
    alg_a: &dyn Fn(NatF<(A, B)>) -> A,
    alg_b: &dyn Fn(NatF<(A, B)>) -> B,
    fix: &FixNat,
) -> (A, B) {
    let paired = fix.0.map_ref(|child| mutu(alg_a, alg_b, child));
    (alg_a(paired.clone()), alg_b(paired))
}

impl<A: Clone> Clone for NatF<A> {
    fn clone(&self) -> Self {
        self.map_ref(|a| a.clone())
    }
}

// Approach 1: isEven / isOdd
fn is_even_alg(n: NatF<(bool, bool)>) -> bool {
    match n {
        NatF::ZeroF => true,
        NatF::SuccF((_even, odd)) => odd,
    }
}

fn is_odd_alg(n: NatF<(bool, bool)>) -> bool {
    match n {
        NatF::ZeroF => false,
        NatF::SuccF((even, _odd)) => even,
    }
}

fn is_even(n: u32) -> bool {
    mutu(&is_even_alg, &is_odd_alg, &nat(n)).0
}
fn is_odd(n: u32) -> bool {
    mutu(&is_even_alg, &is_odd_alg, &nat(n)).1
}

// Approach 2: Typed expression evaluation — value AND type simultaneously
#[derive(Debug, PartialEq)]
enum ExprF<A> {
    IntLit(i64),
    BoolLit(bool),
    Add(A, A),
    Eq(A, A),
    If(A, A, A),
}

impl<A> ExprF<A> {
    fn map_ref<B>(&self, f: impl Fn(&A) -> B) -> ExprF<B> {
        match self {
            ExprF::IntLit(n) => ExprF::IntLit(*n),
            ExprF::BoolLit(b) => ExprF::BoolLit(*b),
            ExprF::Add(a, b) => ExprF::Add(f(a), f(b)),
            ExprF::Eq(a, b) => ExprF::Eq(f(a), f(b)),
            ExprF::If(c, t, e) => ExprF::If(f(c), f(t), f(e)),
        }
    }
}

impl<A: Clone> Clone for ExprF<A> {
    fn clone(&self) -> Self {
        self.map_ref(|a| a.clone())
    }
}

#[derive(Debug, Clone)]
struct FixExpr(Box<ExprF<FixExpr>>);

#[derive(Debug, Clone, PartialEq)]
enum Value {
    VInt(i64),
    VBool(bool),
    VError,
}

#[derive(Debug, Clone, PartialEq)]
enum Typ {
    TInt,
    TBool,
    TError,
}

fn mutu_expr<A: Clone, B: Clone>(
    alg_a: &dyn Fn(ExprF<(A, B)>) -> A,
    alg_b: &dyn Fn(ExprF<(A, B)>) -> B,
    fix: &FixExpr,
) -> (A, B) {
    let paired = fix.0.map_ref(|child| mutu_expr(alg_a, alg_b, child));
    (alg_a(paired.clone()), alg_b(paired))
}

fn val_alg(e: ExprF<(Value, Typ)>) -> Value {
    match e {
        ExprF::IntLit(n) => Value::VInt(n),
        ExprF::BoolLit(b) => Value::VBool(b),
        ExprF::Add((Value::VInt(a), _), (Value::VInt(b), _)) => Value::VInt(a + b),
        ExprF::Eq((Value::VInt(a), _), (Value::VInt(b), _)) => Value::VBool(a == b),
        ExprF::If((Value::VBool(true), _), (v, _), _) => v,
        ExprF::If((Value::VBool(false), _), _, (v, _)) => v,
        _ => Value::VError,
    }
}

fn typ_alg(e: ExprF<(Value, Typ)>) -> Typ {
    match e {
        ExprF::IntLit(_) => Typ::TInt,
        ExprF::BoolLit(_) => Typ::TBool,
        ExprF::Add((_, Typ::TInt), (_, Typ::TInt)) => Typ::TInt,
        ExprF::Eq((_, Typ::TInt), (_, Typ::TInt)) => Typ::TBool,
        ExprF::If((_, Typ::TBool), (_, t1), (_, t2)) if t1 == t2 => t1,
        _ => Typ::TError,
    }
}

fn int_lit(n: i64) -> FixExpr {
    FixExpr(Box::new(ExprF::IntLit(n)))
}
fn bool_lit(b: bool) -> FixExpr {
    FixExpr(Box::new(ExprF::BoolLit(b)))
}
fn add_e(a: FixExpr, b: FixExpr) -> FixExpr {
    FixExpr(Box::new(ExprF::Add(a, b)))
}
fn eq_e(a: FixExpr, b: FixExpr) -> FixExpr {
    FixExpr(Box::new(ExprF::Eq(a, b)))
}
fn if_e(c: FixExpr, t: FixExpr, e: FixExpr) -> FixExpr {
    FixExpr(Box::new(ExprF::If(c, t, e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_odd() {
        for i in 0..10 {
            assert_eq!(is_even(i), i % 2 == 0);
        }
    }

    #[test]
    fn test_type_check_ok() {
        let e = eq_e(int_lit(1), int_lit(2));
        let (v, t) = mutu_expr(&val_alg, &typ_alg, &e);
        assert_eq!(v, Value::VBool(false));
        assert_eq!(t, Typ::TBool);
    }

    #[test]
    fn test_type_error() {
        let e = eq_e(int_lit(1), bool_lit(true));
        assert_eq!(mutu_expr(&val_alg, &typ_alg, &e).1, Typ::TError);
    }
}
