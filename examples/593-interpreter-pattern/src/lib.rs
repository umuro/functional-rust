#![allow(clippy::all)]
//! # Interpreter Pattern
//!
//! Build and evaluate an abstract syntax tree for a simple expression language.

use std::collections::HashMap;

/// Expression AST for a mini-language.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Lit(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Let {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then_: Box<Expr>,
        else_: Box<Expr>,
    },
}

/// Environment mapping variable names to values.
pub type Env = HashMap<String, f64>;

/// Evaluate an expression in an environment.
pub fn eval(env: &Env, e: &Expr) -> Result<f64, String> {
    match e {
        Expr::Lit(n) => Ok(*n),
        Expr::Var(x) => env
            .get(x)
            .copied()
            .ok_or_else(|| format!("undefined variable: {}", x)),
        Expr::Add(l, r) => Ok(eval(env, l)? + eval(env, r)?),
        Expr::Sub(l, r) => Ok(eval(env, l)? - eval(env, r)?),
        Expr::Mul(l, r) => Ok(eval(env, l)? * eval(env, r)?),
        Expr::Div(l, r) => {
            let divisor = eval(env, r)?;
            if divisor == 0.0 {
                Err("division by zero".into())
            } else {
                Ok(eval(env, l)? / divisor)
            }
        }
        Expr::Let { name, value, body } => {
            let v = eval(env, value)?;
            let mut env2 = env.clone();
            env2.insert(name.clone(), v);
            eval(&env2, body)
        }
        Expr::If { cond, then_, else_ } => {
            if eval(env, cond)? != 0.0 {
                eval(env, then_)
            } else {
                eval(env, else_)
            }
        }
    }
}

// Smart constructors for building expressions
pub fn lit(n: f64) -> Box<Expr> {
    Box::new(Expr::Lit(n))
}

pub fn var(s: &str) -> Box<Expr> {
    Box::new(Expr::Var(s.into()))
}

pub fn add(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Add(l, r))
}

pub fn sub(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Sub(l, r))
}

pub fn mul(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Mul(l, r))
}

pub fn div(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Div(l, r))
}

pub fn let_(name: &str, value: Box<Expr>, body: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Let {
        name: name.into(),
        value,
        body,
    })
}

pub fn if_(cond: Box<Expr>, then_: Box<Expr>, else_: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::If { cond, then_, else_ })
}

/// Pretty-print an expression.
pub fn pretty(e: &Expr) -> String {
    match e {
        Expr::Lit(n) => format!("{}", n),
        Expr::Var(x) => x.clone(),
        Expr::Add(l, r) => format!("({} + {})", pretty(l), pretty(r)),
        Expr::Sub(l, r) => format!("({} - {})", pretty(l), pretty(r)),
        Expr::Mul(l, r) => format!("({} * {})", pretty(l), pretty(r)),
        Expr::Div(l, r) => format!("({} / {})", pretty(l), pretty(r)),
        Expr::Let { name, value, body } => {
            format!("let {} = {} in {}", name, pretty(value), pretty(body))
        }
        Expr::If { cond, then_, else_ } => {
            format!(
                "if {} then {} else {}",
                pretty(cond),
                pretty(then_),
                pretty(else_)
            )
        }
    }
}

/// Count AST nodes.
pub fn count_nodes(e: &Expr) -> usize {
    match e {
        Expr::Lit(_) | Expr::Var(_) => 1,
        Expr::Add(l, r) | Expr::Sub(l, r) | Expr::Mul(l, r) | Expr::Div(l, r) => {
            1 + count_nodes(l) + count_nodes(r)
        }
        Expr::Let { value, body, .. } => 1 + count_nodes(value) + count_nodes(body),
        Expr::If { cond, then_, else_ } => {
            1 + count_nodes(cond) + count_nodes(then_) + count_nodes(else_)
        }
    }
}

/// Collect all variable names used in an expression.
pub fn free_vars(e: &Expr) -> Vec<String> {
    fn go(e: &Expr, bound: &[String]) -> Vec<String> {
        match e {
            Expr::Lit(_) => vec![],
            Expr::Var(x) => {
                if bound.contains(x) {
                    vec![]
                } else {
                    vec![x.clone()]
                }
            }
            Expr::Add(l, r) | Expr::Sub(l, r) | Expr::Mul(l, r) | Expr::Div(l, r) => {
                let mut vars = go(l, bound);
                vars.extend(go(r, bound));
                vars
            }
            Expr::Let { name, value, body } => {
                let mut vars = go(value, bound);
                let mut bound2 = bound.to_vec();
                bound2.push(name.clone());
                vars.extend(go(body, &bound2));
                vars
            }
            Expr::If { cond, then_, else_ } => {
                let mut vars = go(cond, bound);
                vars.extend(go(then_, bound));
                vars.extend(go(else_, bound));
                vars
            }
        }
    }
    let vars = go(e, &[]);
    // Remove duplicates
    let mut unique = vec![];
    for v in vars {
        if !unique.contains(&v) {
            unique.push(v);
        }
    }
    unique
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_literal() {
        assert_eq!(eval(&HashMap::new(), &Expr::Lit(42.0)).unwrap(), 42.0);
    }

    #[test]
    fn test_eval_add() {
        let e = add(lit(2.0), lit(3.0));
        assert_eq!(eval(&HashMap::new(), &e).unwrap(), 5.0);
    }

    #[test]
    fn test_eval_let_binding() {
        // let x = 5 in x * 2
        let e = let_("x", lit(5.0), mul(var("x"), lit(2.0)));
        assert_eq!(eval(&HashMap::new(), &e).unwrap(), 10.0);
    }

    #[test]
    fn test_eval_nested_let() {
        // let x = 3 in let y = 4 in x*x + y*y = 9 + 16 = 25
        let e = let_(
            "x",
            lit(3.0),
            let_(
                "y",
                lit(4.0),
                add(mul(var("x"), var("x")), mul(var("y"), var("y"))),
            ),
        );
        assert_eq!(eval(&HashMap::new(), &e).unwrap(), 25.0);
    }

    #[test]
    fn test_eval_if_true() {
        let e = if_(lit(1.0), lit(42.0), lit(0.0));
        assert_eq!(eval(&HashMap::new(), &e).unwrap(), 42.0);
    }

    #[test]
    fn test_eval_if_false() {
        let e = if_(lit(0.0), lit(42.0), lit(100.0));
        assert_eq!(eval(&HashMap::new(), &e).unwrap(), 100.0);
    }

    #[test]
    fn test_eval_undefined_var() {
        let e = var("z");
        assert!(eval(&HashMap::new(), &e).is_err());
    }

    #[test]
    fn test_eval_div_zero() {
        let e = div(lit(10.0), lit(0.0));
        assert!(eval(&HashMap::new(), &e).is_err());
    }

    #[test]
    fn test_pretty() {
        let e = add(lit(2.0), mul(lit(3.0), lit(4.0)));
        assert_eq!(pretty(&e), "(2 + (3 * 4))");
    }

    #[test]
    fn test_count_nodes() {
        let e = add(lit(1.0), lit(2.0));
        assert_eq!(count_nodes(&e), 3);
    }

    #[test]
    fn test_free_vars() {
        // let x = 1 in x + y -> y is free
        let e = let_("x", lit(1.0), add(var("x"), var("y")));
        assert_eq!(free_vars(&e), vec!["y".to_string()]);
    }
}
