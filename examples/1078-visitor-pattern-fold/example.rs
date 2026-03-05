#[derive(Debug, Clone)]
enum Expr {
    Lit(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

fn fold<R>(
    expr: &Expr,
    lit: &dyn Fn(f64) -> R,
    add: &dyn Fn(R, R) -> R,
    mul: &dyn Fn(R, R) -> R,
    neg: &dyn Fn(R) -> R,
) -> R {
    match expr {
        Expr::Lit(x) => lit(*x),
        Expr::Add(a, b) => add(fold(a, lit, add, mul, neg), fold(b, lit, add, mul, neg)),
        Expr::Mul(a, b) => mul(fold(a, lit, add, mul, neg), fold(b, lit, add, mul, neg)),
        Expr::Neg(a) => neg(fold(a, lit, add, mul, neg)),
    }
}

fn eval(expr: &Expr) -> f64 {
    fold(expr, &|x| x, &|a, b| a + b, &|a, b| a * b, &|x| -x)
}

fn to_string(expr: &Expr) -> String {
    fold(
        expr,
        &|x| format!("{x}"),
        &|a, b| format!("({a} + {b})"),
        &|a, b| format!("({a} * {b})"),
        &|a| format!("(-{a})"),
    )
}

fn lit(x: f64) -> Expr { Expr::Lit(x) }
fn add(a: Expr, b: Expr) -> Expr { Expr::Add(Box::new(a), Box::new(b)) }
fn mul(a: Expr, b: Expr) -> Expr { Expr::Mul(Box::new(a), Box::new(b)) }
fn neg(a: Expr) -> Expr { Expr::Neg(Box::new(a)) }

fn main() {
    let e = add(mul(lit(2.0), lit(3.0)), neg(lit(1.0)));
    println!("{} = {}", to_string(&e), eval(&e));
}

/* Output:
   ((2 * 3) + (-1)) = 5
*/
