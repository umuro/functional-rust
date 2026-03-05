#[derive(Debug,Clone)]
enum Expr { Lit(f64), Add(Box<Expr>,Box<Expr>), Sub(Box<Expr>,Box<Expr>),
            Mul(Box<Expr>,Box<Expr>), Div(Box<Expr>,Box<Expr>) }

// Visitor 1: evaluate
fn eval(e: &Expr) -> f64 {
    match e {
        Expr::Lit(n)      => *n,
        Expr::Add(l,r)    => eval(l) + eval(r),
        Expr::Sub(l,r)    => eval(l) - eval(r),
        Expr::Mul(l,r)    => eval(l) * eval(r),
        Expr::Div(l,r)    => eval(l) / eval(r),
    }
}

// Visitor 2: count operations
fn count_ops(e: &Expr) -> usize {
    match e {
        Expr::Lit(_)      => 0,
        Expr::Add(l,r)|Expr::Sub(l,r)|Expr::Mul(l,r)|Expr::Div(l,r)
                          => 1 + count_ops(l) + count_ops(r),
    }
}

// Visitor 3: pretty print
fn pretty(e: &Expr) -> String {
    match e {
        Expr::Lit(n)      => format!("{}", n),
        Expr::Add(l,r)    => format!("({}+{})", pretty(l), pretty(r)),
        Expr::Sub(l,r)    => format!("({}-{})", pretty(l), pretty(r)),
        Expr::Mul(l,r)    => format!("({}*{})", pretty(l), pretty(r)),
        Expr::Div(l,r)    => format!("({}/{})", pretty(l), pretty(r)),
    }
}

// Visitor 4: collect all literals
fn collect_lits(e: &Expr) -> Vec<f64> {
    match e {
        Expr::Lit(n)      => vec![*n],
        Expr::Add(l,r)|Expr::Sub(l,r)|Expr::Mul(l,r)|Expr::Div(l,r) => {
            let mut v = collect_lits(l); v.extend(collect_lits(r)); v
        }
    }
}

fn main() {
    use Expr::*;
    let e = Add(Box::new(Mul(Box::new(Lit(3.0)), Box::new(Lit(4.0)))),
                Box::new(Sub(Box::new(Lit(10.0)),Box::new(Lit(2.0)))));
    println!("{} = {:.1} (ops={}, lits={:?})", pretty(&e), eval(&e), count_ops(&e), collect_lits(&e));
}

#[cfg(test)]
mod tests {
    use super::*;
    use Expr::*;
    #[test] fn eval_add()  { assert_eq!(eval(&Add(Box::new(Lit(2.0)),Box::new(Lit(3.0)))), 5.0); }
    #[test] fn ops_count() { let e = Add(Box::new(Lit(1.0)),Box::new(Lit(2.0))); assert_eq!(count_ops(&e),1); }
}
