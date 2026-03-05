// Example 215: Recursion Schemes — Separating What From How

// === Approach 1: Direct — recursion entangled with logic ===

#[derive(Debug, Clone)]
enum Expr {
    Lit(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn lit(n: i64) -> Self { Expr::Lit(n) }
    fn add(a: Expr, b: Expr) -> Self { Expr::Add(Box::new(a), Box::new(b)) }
    fn mul(a: Expr, b: Expr) -> Self { Expr::Mul(Box::new(a), Box::new(b)) }
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Lit(n)    => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

fn show(e: &Expr) -> String {
    match e {
        Expr::Lit(n)    => n.to_string(),
        Expr::Add(a, b) => format!("({} + {})", show(a), show(b)),
        Expr::Mul(a, b) => format!("({} * {})", show(a), show(b)),
    }
}

// === Approach 2: Catamorphism — recursion factored out ===

// Base functor: recursive positions replaced by type variable A
enum ExprF<A> {
    Lit(i64),
    Add(A, A),
    Mul(A, A),
}

impl<A> ExprF<A> {
    fn map<B, F: Fn(A) -> B>(self, f: F) -> ExprF<B> {
        match self {
            ExprF::Lit(n)    => ExprF::Lit(n),
            ExprF::Add(a, b) => ExprF::Add(f(a), f(b)),
            ExprF::Mul(a, b) => ExprF::Mul(f(a), f(b)),
        }
    }
}

fn project(e: Expr) -> ExprF<Box<Expr>> {
    match e {
        Expr::Lit(n)    => ExprF::Lit(n),
        Expr::Add(a, b) => ExprF::Add(a, b),
        Expr::Mul(a, b) => ExprF::Mul(a, b),
    }
}

// The ONE place recursion lives
fn cata<A, F: Fn(ExprF<A>) -> A>(e: Expr, alg: &F) -> A {
    alg(project(e).map(|child| cata(*child, alg)))
}

// Algebras: pure logic, no recursion
fn eval_cata(e: Expr) -> i64 {
    cata(e, &|node| match node {
        ExprF::Lit(n)    => n,
        ExprF::Add(a, b) => a + b,
        ExprF::Mul(a, b) => a * b,
    })
}

fn show_cata(e: Expr) -> String {
    cata(e, &|node| match node {
        ExprF::Lit(n)    => n.to_string(),
        ExprF::Add(a, b) => format!("({a} + {b})"),
        ExprF::Mul(a, b) => format!("({a} * {b})"),
    })
}

fn count_nodes(e: Expr) -> usize {
    cata(e, &|node| match node {
        ExprF::Lit(_)    => 1,
        ExprF::Add(a, b) | ExprF::Mul(a, b) => 1 + a + b,
    })
}

fn main() {
    // (2 + 3) * 4
    let sample = || Expr::make_mul(Expr::make_add(Expr::lit(2), Expr::lit(3)), Expr::lit(4));

    println!("=== Direct recursion ===");
    println!("eval((2+3)*4)  = {}", eval(&sample()));
    println!("show((2+3)*4)  = {}", show(&sample()));

    println!("\n=== Catamorphism ===");
    println!("eval_cata      = {}", eval_cata(sample()));
    println!("show_cata      = {}", show_cata(sample()));
    println!("count_nodes    = {}", count_nodes(sample()));
}

/* Output:
   === Direct recursion ===
   eval((2+3)*4)  = 20
   show((2+3)*4)  = ((2 + 3) * 4)

   === Catamorphism ===
   eval_cata      = 20
   show_cata      = ((2 + 3) * 4)
   count_nodes    = 5
*/
