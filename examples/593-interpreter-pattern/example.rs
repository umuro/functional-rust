use std::collections::HashMap;

#[derive(Debug,Clone)]
enum Expr {
    Lit(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Let { name: String, value: Box<Expr>, body: Box<Expr> },
    If  { cond: Box<Expr>, then_: Box<Expr>, else_: Box<Expr> },
}

type Env = HashMap<String, f64>;

fn eval(env: &Env, e: &Expr) -> Result<f64, String> {
    match e {
        Expr::Lit(n)           => Ok(*n),
        Expr::Var(x)           => env.get(x).copied().ok_or_else(|| format!("undefined: {}", x)),
        Expr::Add(l,r)         => Ok(eval(env,l)? + eval(env,r)?),
        Expr::Mul(l,r)         => Ok(eval(env,l)? * eval(env,r)?),
        Expr::Sub(l,r)         => Ok(eval(env,l)? - eval(env,r)?),
        Expr::Let{name,value,body} => {
            let v = eval(env,value)?;
            let mut env2 = env.clone();
            env2.insert(name.clone(), v);
            eval(&env2, body)
        }
        Expr::If{cond,then_,else_} => {
            if eval(env,cond)? != 0.0 { eval(env,then_) } else { eval(env,else_) }
        }
    }
}

// Smart constructors
fn lit(n: f64) -> Box<Expr>  { Box::new(Expr::Lit(n)) }
fn var(s: &str) -> Box<Expr> { Box::new(Expr::Var(s.into())) }
fn add(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> { Box::new(Expr::Add(l,r)) }
fn mul(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> { Box::new(Expr::Mul(l,r)) }
fn let_(name: &str, value: Box<Expr>, body: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Let { name:name.into(), value, body })
}

fn main() {
    // let x=3 in let y=4 in x*x + y*y
    let prog = let_("x", lit(3.0),
                   let_("y", lit(4.0),
                        add(mul(var("x"),var("x")), mul(var("y"),var("y")))));
    println!("result = {:.1}", eval(&HashMap::new(), &prog).unwrap());

    // if 1 then 42 else 0
    let cond = Box::new(Expr::If { cond:lit(1.0), then_:lit(42.0), else_:lit(0.0) });
    println!("if result = {:.1}", eval(&HashMap::new(), &cond).unwrap());

    // undefined variable
    println!("undef: {}", eval(&HashMap::new(), &var("z")).unwrap_err());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn let_binding() {
        let e = let_("x", lit(5.0), mul(var("x"), lit(2.0)));
        assert_eq!(eval(&HashMap::new(), &e).unwrap(), 10.0);
    }
    #[test] fn undef() {
        assert!(eval(&HashMap::new(), &var("z")).is_err());
    }
}
