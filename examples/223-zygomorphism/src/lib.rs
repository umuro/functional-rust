// Example 223: Zygomorphism — Two Mutually Dependent Folds

#[derive(Debug, Clone)]
enum ExprF<A> { LitF(i64), AddF(A, A), MulF(A, A), NegF(A) }

impl<A> ExprF<A> {
    fn map<B>(self, f: impl Fn(A) -> B) -> ExprF<B> {
        match self {
            ExprF::LitF(n) => ExprF::LitF(n),
            ExprF::AddF(a, b) => ExprF::AddF(f(a), f(b)),
            ExprF::MulF(a, b) => ExprF::MulF(f(a), f(b)),
            ExprF::NegF(a) => ExprF::NegF(f(a)),
        }
    }
    fn map_ref<B>(&self, f: impl Fn(&A) -> B) -> ExprF<B> {
        match self {
            ExprF::LitF(n) => ExprF::LitF(*n),
            ExprF::AddF(a, b) => ExprF::AddF(f(a), f(b)),
            ExprF::MulF(a, b) => ExprF::MulF(f(a), f(b)),
            ExprF::NegF(a) => ExprF::NegF(f(a)),
        }
    }
    fn map_snd<X, Y, B>(&self, f: impl Fn(&(X, Y)) -> B) -> ExprF<B>
    where A: AsRef<(X, Y)> {
        self.map_ref(|a| f(a.as_ref()))
    }
}

#[derive(Debug, Clone)]
struct Fix(Box<ExprF<Fix>>);

// zygo: compute (main_result, helper_result) simultaneously
fn zygo_both<A: Clone, B: Clone>(
    helper: &dyn Fn(ExprF<B>) -> B,
    main: &dyn Fn(ExprF<(A, B)>) -> A,
    fix: &Fix,
) -> (A, B) {
    let paired: ExprF<(A, B)> = fix.0.map_ref(|child| zygo_both(helper, main, child));
    let b_layer = paired.map_ref(|(_, b)| b.clone());
    let a = main(paired.clone());
    let b = helper(b_layer);
    (a, b)
}

fn zygo<A: Clone, B: Clone>(
    helper: &dyn Fn(ExprF<B>) -> B,
    main: &dyn Fn(ExprF<(A, B)>) -> A,
    fix: &Fix,
) -> A {
    zygo_both(helper, main, fix).0
}

// Need Clone for ExprF<(A, B)>
impl<A: Clone> Clone for ExprF<A> {
    fn clone(&self) -> Self { self.map_ref(|a| a.clone()) }
}

// Approach 1: Safety check — helper evaluates, main checks bounds
fn eval_helper(e: ExprF<i64>) -> i64 {
    match e {
        ExprF::LitF(n) => n,
        ExprF::AddF(a, b) => a + b,
        ExprF::MulF(a, b) => a * b,
        ExprF::NegF(a) => -a,
    }
}

fn safe_main(e: ExprF<(bool, i64)>) -> bool {
    match e {
        ExprF::LitF(_) => true,
        ExprF::AddF((a, _), (b, _)) => a && b,
        ExprF::MulF((a, va), (b, vb)) => a && b && va.abs() < 1000 && vb.abs() < 1000,
        ExprF::NegF((a, _)) => a,
    }
}

// Approach 2: Pretty print with precedence
fn prec_helper(e: ExprF<u32>) -> u32 {
    match e {
        ExprF::LitF(_) => 100,
        ExprF::AddF(_, _) => 1,
        ExprF::MulF(_, _) => 2,
        ExprF::NegF(_) => 3,
    }
}

fn show_main(e: ExprF<(String, u32)>) -> String {
    match e {
        ExprF::LitF(n) => n.to_string(),
        ExprF::AddF((a, pa), (b, pb)) => {
            let la = if pa < 1 { format!("({a})") } else { a };
            let rb = if pb < 1 { format!("({b})") } else { b };
            format!("{la} + {rb}")
        }
        ExprF::MulF((a, pa), (b, pb)) => {
            let la = if pa < 2 { format!("({a})") } else { a };
            let rb = if pb < 2 { format!("({b})") } else { b };
            format!("{la} * {rb}")
        }
        ExprF::NegF((a, _)) => format!("-{a}"),
    }
}

fn lit(n: i64) -> Fix { Fix(Box::new(ExprF::LitF(n))) }
fn add(a: Fix, b: Fix) -> Fix { Fix(Box::new(ExprF::AddF(a, b))) }
fn mul(a: Fix, b: Fix) -> Fix { Fix(Box::new(ExprF::MulF(a, b))) }
fn neg(a: Fix) -> Fix { Fix(Box::new(ExprF::NegF(a))) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe() {
        assert!(zygo(&eval_helper, &safe_main, &add(lit(1), lit(2))));
    }

    #[test]
    fn test_unsafe() {
        assert!(!zygo(&eval_helper, &safe_main, &mul(lit(100000), lit(2))));
    }

    #[test]
    fn test_precedence() {
        let e = add(mul(lit(2), lit(3)), lit(4));
        assert_eq!(zygo(&prec_helper, &show_main, &e), "2 * 3 + 4");
    }
}
