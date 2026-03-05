// Finally tagless: the language is a trait
trait Expr<R> {
    fn lit(n: i64)     -> R;
    fn add(l: R, r: R) -> R;
    fn mul(l: R, r: R) -> R;
    fn neg(x: R)       -> R;
}

// Interpreter 1: evaluate
struct EvalInterp;
impl Expr<i64> for EvalInterp {
    fn lit(n: i64)       -> i64 { n }
    fn add(l: i64, r: i64) -> i64 { l + r }
    fn mul(l: i64, r: i64) -> i64 { l * r }
    fn neg(x: i64)         -> i64 { -x }
}

// Interpreter 2: pretty print
struct PrintInterp;
impl Expr<String> for PrintInterp {
    fn lit(n: i64)           -> String { format!("{}", n) }
    fn add(l: String, r: String) -> String { format!("({}+{})", l, r) }
    fn mul(l: String, r: String) -> String { format!("({}*{})", l, r) }
    fn neg(x: String)            -> String { format!("(-{})", x) }
}

// Interpreter 3: count operations
struct CountInterp;
impl Expr<usize> for CountInterp {
    fn lit(_: i64)           -> usize { 0 }
    fn add(l: usize, r: usize) -> usize { 1 + l + r }
    fn mul(l: usize, r: usize) -> usize { 1 + l + r }
    fn neg(x: usize)           -> usize { 1 + x }
}

// Same expression, multiple interpretations: 3*4 + (-2)
fn program<R, E: Expr<R>>() -> R {
    E::add(
        E::mul(E::lit(3), E::lit(4)),
        E::neg(E::lit(2)),
    )
}

fn main() {
    println!("eval  = {}", program::<i64,    EvalInterp>());
    println!("print = {}", program::<String, PrintInterp>());
    println!("ops   = {}", program::<usize,  CountInterp>());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn eval()  { assert_eq!(program::<i64,EvalInterp>(), 10); }
    #[test] fn print() { assert!(program::<String,PrintInterp>().contains("+")); }
    #[test] fn count() { assert_eq!(program::<usize,CountInterp>(), 3); }
}
