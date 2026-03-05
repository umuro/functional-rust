#[derive(Debug, Clone)]
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Var(String),
}
use Expr::*;

fn eval(env: &[(&str, f64)], e: &Expr) -> f64 {
    match e {
        Num(n)       => *n,
        Add(l, r)    => eval(env, l) + eval(env, r),
        Mul(l, r)    => eval(env, l) * eval(env, r),
        Neg(e)       => -eval(env, e),
        Var(s)       => env.iter().find(|&&(k,_)| k == s).map(|&(_,v)| v).unwrap_or(0.0),
    }
}

fn show(e: &Expr) -> String {
    match e {
        Num(n)       => format!("{}", n),
        Add(l, r)    => format!("({}+{})", show(l), show(r)),
        Mul(l, r)    => format!("({}*{})", show(l), show(r)),
        Neg(e)       => format!("(-{})", show(e)),
        Var(s)       => s.clone(),
    }
}

fn main() {
    let env = [("x",3.0),("y",4.0)];
    let e = Add(Box::new(Mul(Box::new(Var("x".into())), Box::new(Var("x".into())))),
                Box::new(Mul(Box::new(Var("y".into())), Box::new(Var("y".into())))));
    println!("{} = {:.1}", show(&e), eval(&env, &e));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn add() { assert_eq!(eval(&[], &Add(Box::new(Num(2.0)), Box::new(Num(3.0)))), 5.0); }
    #[test] fn neg() { assert_eq!(eval(&[], &Neg(Box::new(Num(7.0)))), -7.0); }
}
