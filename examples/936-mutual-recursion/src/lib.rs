/// Mutual Recursion with `and`
///
/// OCaml uses `let rec ... and ...` for mutually recursive functions.
/// Rust doesn't need special syntax — functions can call each other
/// freely as long as they're in scope. The compiler handles it.

/// Mutually recursive even/odd check.
/// In OCaml, these require `and` to co-define them.
/// In Rust, mutual recursion "just works" — no special syntax needed.
pub fn is_even(n: u32) -> bool {
    match n {
        0 => true,
        n => is_odd(n - 1),
    }
}

pub fn is_odd(n: u32) -> bool {
    match n {
        0 => false,
        n => is_even(n - 1),
    }
}

/// Expression tree with mutual recursion over variants.
#[derive(Debug, Clone)]
pub enum Expr {
    Lit(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn lit(n: i32) -> Self { Expr::Lit(n) }
    pub fn add(l: Expr, r: Expr) -> Self { Expr::Add(Box::new(l), Box::new(r)) }
    pub fn mul(l: Expr, r: Expr) -> Self { Expr::Mul(Box::new(l), Box::new(r)) }
}

pub fn eval_expr(e: &Expr) -> i32 {
    match e {
        Expr::Lit(n) => *n,
        Expr::Add(l, r) => eval_expr(l) + eval_expr(r),
        Expr::Mul(l, r) => eval_mul(l, r),
    }
}

fn eval_mul(l: &Expr, r: &Expr) -> i32 {
    eval_expr(l) * eval_expr(r)
}

/// Iterative is_even — avoids stack overflow for large n.
pub fn is_even_iter(n: u32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(7));
        assert!(is_even(0));
    }

    #[test]
    fn test_is_odd() {
        assert!(is_odd(7));
        assert!(!is_odd(4));
        assert!(!is_odd(0));
    }

    #[test]
    fn test_eval_expr() {
        // (2 + 3) * 4 = 20
        let e = Expr::mul(Expr::add(Expr::lit(2), Expr::lit(3)), Expr::lit(4));
        assert_eq!(eval_expr(&e), 20);
    }

    #[test]
    fn test_eval_simple() {
        assert_eq!(eval_expr(&Expr::lit(42)), 42);
        assert_eq!(eval_expr(&Expr::add(Expr::lit(1), Expr::lit(2))), 3);
    }

    #[test]
    fn test_nested_expr() {
        // (1 + 2) * (3 + 4) = 21
        let e = Expr::mul(
            Expr::add(Expr::lit(1), Expr::lit(2)),
            Expr::add(Expr::lit(3), Expr::lit(4)),
        );
        assert_eq!(eval_expr(&e), 21);
    }

    #[test]
    fn test_is_even_iter() {
        assert!(is_even_iter(100));
        assert!(!is_even_iter(101));
    }
}
