//! # Visitor Pattern via Match
//!
//! Implement multiple traversal operations over a recursive data structure
//! using pattern matching instead of the traditional OOP visitor pattern.

/// Expression AST for arithmetic operations.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Lit(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Helper to create a literal.
    pub fn lit(n: f64) -> Box<Self> {
        Box::new(Expr::Lit(n))
    }

    /// Helper to create an addition.
    pub fn add(l: Box<Expr>, r: Box<Expr>) -> Box<Self> {
        Box::new(Expr::Add(l, r))
    }

    /// Helper to create a subtraction.
    pub fn sub(l: Box<Expr>, r: Box<Expr>) -> Box<Self> {
        Box::new(Expr::Sub(l, r))
    }

    /// Helper to create a multiplication.
    pub fn mul(l: Box<Expr>, r: Box<Expr>) -> Box<Self> {
        Box::new(Expr::Mul(l, r))
    }

    /// Helper to create a division.
    pub fn div(l: Box<Expr>, r: Box<Expr>) -> Box<Self> {
        Box::new(Expr::Div(l, r))
    }
}

/// Visitor 1: Evaluate expression to a number.
pub fn eval(e: &Expr) -> f64 {
    match e {
        Expr::Lit(n) => *n,
        Expr::Add(l, r) => eval(l) + eval(r),
        Expr::Sub(l, r) => eval(l) - eval(r),
        Expr::Mul(l, r) => eval(l) * eval(r),
        Expr::Div(l, r) => eval(l) / eval(r),
    }
}

/// Visitor 2: Count number of operations.
pub fn count_ops(e: &Expr) -> usize {
    match e {
        Expr::Lit(_) => 0,
        Expr::Add(l, r) | Expr::Sub(l, r) | Expr::Mul(l, r) | Expr::Div(l, r) => {
            1 + count_ops(l) + count_ops(r)
        }
    }
}

/// Visitor 3: Pretty print the expression.
pub fn pretty(e: &Expr) -> String {
    match e {
        Expr::Lit(n) => format!("{}", n),
        Expr::Add(l, r) => format!("({} + {})", pretty(l), pretty(r)),
        Expr::Sub(l, r) => format!("({} - {})", pretty(l), pretty(r)),
        Expr::Mul(l, r) => format!("({} * {})", pretty(l), pretty(r)),
        Expr::Div(l, r) => format!("({} / {})", pretty(l), pretty(r)),
    }
}

/// Visitor 4: Collect all literal values.
pub fn collect_lits(e: &Expr) -> Vec<f64> {
    match e {
        Expr::Lit(n) => vec![*n],
        Expr::Add(l, r) | Expr::Sub(l, r) | Expr::Mul(l, r) | Expr::Div(l, r) => {
            let mut v = collect_lits(l);
            v.extend(collect_lits(r));
            v
        }
    }
}

/// Visitor 5: Calculate tree depth.
pub fn depth(e: &Expr) -> usize {
    match e {
        Expr::Lit(_) => 1,
        Expr::Add(l, r) | Expr::Sub(l, r) | Expr::Mul(l, r) | Expr::Div(l, r) => {
            1 + depth(l).max(depth(r))
        }
    }
}

/// Visitor 6: Simplify constant expressions (constant folding).
pub fn simplify(e: &Expr) -> Box<Expr> {
    match e {
        Expr::Lit(n) => Expr::lit(*n),
        Expr::Add(l, r) => {
            let l = simplify(l);
            let r = simplify(r);
            if let (Expr::Lit(a), Expr::Lit(b)) = (l.as_ref(), r.as_ref()) {
                Expr::lit(a + b)
            } else {
                Expr::add(l, r)
            }
        }
        Expr::Sub(l, r) => {
            let l = simplify(l);
            let r = simplify(r);
            if let (Expr::Lit(a), Expr::Lit(b)) = (l.as_ref(), r.as_ref()) {
                Expr::lit(a - b)
            } else {
                Expr::sub(l, r)
            }
        }
        Expr::Mul(l, r) => {
            let l = simplify(l);
            let r = simplify(r);
            if let (Expr::Lit(a), Expr::Lit(b)) = (l.as_ref(), r.as_ref()) {
                Expr::lit(a * b)
            } else {
                Expr::mul(l, r)
            }
        }
        Expr::Div(l, r) => {
            let l = simplify(l);
            let r = simplify(r);
            if let (Expr::Lit(a), Expr::Lit(b)) = (l.as_ref(), r.as_ref()) {
                Expr::lit(a / b)
            } else {
                Expr::div(l, r)
            }
        }
    }
}

/// Visitor 7: Check if expression contains division.
pub fn has_division(e: &Expr) -> bool {
    match e {
        Expr::Lit(_) => false,
        Expr::Div(_, _) => true,
        Expr::Add(l, r) | Expr::Sub(l, r) | Expr::Mul(l, r) => has_division(l) || has_division(r),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_expr() -> Box<Expr> {
        // (3 * 4) + (10 - 2) = 12 + 8 = 20
        Expr::add(
            Expr::mul(Expr::lit(3.0), Expr::lit(4.0)),
            Expr::sub(Expr::lit(10.0), Expr::lit(2.0)),
        )
    }

    #[test]
    fn test_eval() {
        let e = sample_expr();
        assert_eq!(eval(&e), 20.0);
    }

    #[test]
    fn test_eval_simple() {
        assert_eq!(eval(&Expr::Lit(42.0)), 42.0);
        assert_eq!(eval(&Expr::Add(Expr::lit(2.0), Expr::lit(3.0))), 5.0);
        assert_eq!(eval(&Expr::Mul(Expr::lit(4.0), Expr::lit(5.0))), 20.0);
    }

    #[test]
    fn test_count_ops() {
        let e = sample_expr();
        assert_eq!(count_ops(&e), 3); // add, mul, sub
    }

    #[test]
    fn test_count_ops_lit() {
        assert_eq!(count_ops(&Expr::Lit(1.0)), 0);
    }

    #[test]
    fn test_pretty() {
        let e = Expr::add(Expr::lit(2.0), Expr::lit(3.0));
        assert_eq!(pretty(&e), "(2 + 3)");
    }

    #[test]
    fn test_collect_lits() {
        let e = sample_expr();
        let lits = collect_lits(&e);
        assert_eq!(lits, vec![3.0, 4.0, 10.0, 2.0]);
    }

    #[test]
    fn test_depth() {
        let e = sample_expr();
        assert_eq!(depth(&e), 3); // add -> mul/sub -> lit
    }

    #[test]
    fn test_depth_lit() {
        assert_eq!(depth(&Expr::Lit(1.0)), 1);
    }

    #[test]
    fn test_simplify() {
        let e = sample_expr();
        let simplified = simplify(&e);
        assert_eq!(*simplified, Expr::Lit(20.0));
    }

    #[test]
    fn test_has_division() {
        let e = sample_expr();
        assert!(!has_division(&e));

        let e_div = Expr::div(Expr::lit(10.0), Expr::lit(2.0));
        assert!(has_division(&e_div));
    }
}
