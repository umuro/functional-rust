// Base functor for a simple Nat type
#[derive(Debug)]
enum NatF<A> { Zero, Succ(A) }

// Fix: wraps the recursive layer in a Box
struct Fix<F>(Box<F>);

type Nat = Fix<NatF<Fix<NatF<()>>>>;  // Only 2 levels for demo; true Fix requires GAT/recursive types

// More practical: define recursion schemes over concrete types

// Expression base functor (non-recursive)
#[derive(Debug,Clone)]
enum ExprF<R> {
    Lit(i64),
    Add(R, R),
    Mul(R, R),
    Neg(R),
}

// Recursive expression using Fix-like structure
#[derive(Debug,Clone)]
struct Expr(Box<ExprF<Expr>>);

impl Expr {
    fn lit(n: i64) -> Self { Expr(Box::new(ExprF::Lit(n))) }
    fn add(l: Self, r: Self) -> Self { Expr(Box::new(ExprF::Add(l, r))) }
    fn mul(l: Self, r: Self) -> Self { Expr(Box::new(ExprF::Mul(l, r))) }
    fn neg(x: Self) -> Self { Expr(Box::new(ExprF::Neg(x))) }
}

// Catamorphism (fold) over the expression
fn cata<R>(expr: Expr, alg: &impl Fn(ExprF<R>) -> R) -> R {
    match *expr.0 {
        ExprF::Lit(n)    => alg(ExprF::Lit(n)),
        ExprF::Add(l,r)  => { let l=cata(l,alg); let r=cata(r,alg); alg(ExprF::Add(l,r)) }
        ExprF::Mul(l,r)  => { let l=cata(l,alg); let r=cata(r,alg); alg(ExprF::Mul(l,r)) }
        ExprF::Neg(x)    => { let x=cata(x,alg); alg(ExprF::Neg(x)) }
    }
}

// Two algebras
fn eval_alg(e: ExprF<i64>) -> i64 {
    match e { ExprF::Lit(n)=>n, ExprF::Add(l,r)=>l+r, ExprF::Mul(l,r)=>l*r, ExprF::Neg(x)=>-x }
}

fn print_alg(e: ExprF<String>) -> String {
    match e {
        ExprF::Lit(n)    => format!("{}", n),
        ExprF::Add(l,r)  => format!("({}+{})",l,r),
        ExprF::Mul(l,r)  => format!("({}*{})",l,r),
        ExprF::Neg(x)    => format!("(-{})",x),
    }
}

fn main() {
    // (3*4) + (-2)
    let e = Expr::add(
        Expr::mul(Expr::lit(3), Expr::lit(4)),
        Expr::neg(Expr::lit(2)),
    );
    let e2 = e.clone();
    println!("eval  = {}", cata(e,  &eval_alg));
    println!("print = {}", cata(e2, &print_alg));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn eval_lit() { let e=Expr::lit(42); assert_eq!(cata(e,&eval_alg),42); }
    #[test] fn eval_add() { let e=Expr::add(Expr::lit(2),Expr::lit(3)); assert_eq!(cata(e,&eval_alg),5); }
}
