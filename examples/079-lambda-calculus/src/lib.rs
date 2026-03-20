#![allow(clippy::all)]
/// Simple Lambda Calculus Interpreter
///
/// Ownership insight: The expression tree uses Box for recursive types.
/// Environments clone values because closures capture their environment.

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Int(i64),
    Var(String),
    Lam(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    VInt(i64),
    VClosure(String, Box<Expr>, Env),
}

type Env = Vec<(String, Value)>;

/// Evaluate an expression in an environment
/// Ownership: env is cloned when creating closures (capturing environment)
pub fn eval(env: &Env, expr: &Expr) -> Result<Value, String> {
    match expr {
        Expr::Int(n) => Ok(Value::VInt(*n)),
        Expr::Var(x) => env
            .iter()
            .rev()
            .find(|(k, _)| k == x)
            .map(|(_, v)| v.clone())
            .ok_or_else(|| format!("unbound variable: {}", x)),
        Expr::Lam(x, body) => Ok(Value::VClosure(x.clone(), body.clone(), env.clone())),
        Expr::App(f, arg) => {
            let fv = eval(env, f)?;
            let av = eval(env, arg)?;
            match fv {
                Value::VClosure(x, body, mut cenv) => {
                    cenv.push((x, av));
                    eval(&cenv, &body)
                }
                _ => Err("not a function".into()),
            }
        }
        Expr::Add(a, b) => match (eval(env, a)?, eval(env, b)?) {
            (Value::VInt(x), Value::VInt(y)) => Ok(Value::VInt(x + y)),
            _ => Err("type error in add".into()),
        },
    }
}

/// Version 2: Using Rc for shared expression trees (avoids deep clones)
/// In production, you'd use Rc<Expr> to share subtrees.

#[cfg(test)]
mod tests {
    use super::*;

    fn int(n: i64) -> Expr {
        Expr::Int(n)
    }
    fn var(s: &str) -> Expr {
        Expr::Var(s.into())
    }
    fn lam(s: &str, body: Expr) -> Expr {
        Expr::Lam(s.into(), Box::new(body))
    }
    fn app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    fn add(a: Expr, b: Expr) -> Expr {
        Expr::Add(Box::new(a), Box::new(b))
    }

    #[test]
    fn test_integer() {
        assert_eq!(eval(&vec![], &int(42)), Ok(Value::VInt(42)));
    }

    #[test]
    fn test_identity() {
        // (\x -> x) 42
        let e = app(lam("x", var("x")), int(42));
        assert_eq!(eval(&vec![], &e), Ok(Value::VInt(42)));
    }

    #[test]
    fn test_add() {
        // (\x -> x + 1) 41
        let e = app(lam("x", add(var("x"), int(1))), int(41));
        assert_eq!(eval(&vec![], &e), Ok(Value::VInt(42)));
    }

    #[test]
    fn test_nested_lambda() {
        // (\x -> \y -> x + y) 10 32
        let e = app(
            app(lam("x", lam("y", add(var("x"), var("y")))), int(10)),
            int(32),
        );
        assert_eq!(eval(&vec![], &e), Ok(Value::VInt(42)));
    }

    #[test]
    fn test_unbound_var() {
        assert!(eval(&vec![], &var("x")).is_err());
    }
}
