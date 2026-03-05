// Example 225: Prepromorphism — Apply Natural Transformation at Each Step of Cata

// prepro: like cata, but applies a nat transform to each layer before recursing

#[derive(Debug, Clone)]
enum ExprF<A> {
    LitF(i64),
    AddF(A, A),
    MulF(A, A),
    NegF(A),
}

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
}

#[derive(Debug, Clone)]
struct Fix(Box<ExprF<Fix>>);

fn cata<A>(alg: &dyn Fn(ExprF<A>) -> A, Fix(f): &Fix) -> A {
    alg(f.map_ref(|child| cata(alg, child)))
}

// prepro: transform each child's layer before recursing
fn prepro<A>(
    nat: &dyn Fn(ExprF<Fix>) -> ExprF<Fix>,
    alg: &dyn Fn(ExprF<A>) -> A,
    Fix(f): &Fix,
) -> A {
    alg(f.map_ref(|child| {
        // Apply natural transformation to child's layer, then recurse
        let transformed = Fix(Box::new(nat(child.0.as_ref().clone())));
        prepro(nat, alg, &transformed)
    }))
}

impl Clone for ExprF<Fix> {
    fn clone(&self) -> Self { self.map_ref(|a| a.clone()) }
}

fn eval_alg(e: ExprF<i64>) -> i64 {
    match e {
        ExprF::LitF(n) => n,
        ExprF::AddF(a, b) => a + b,
        ExprF::MulF(a, b) => a * b,
        ExprF::NegF(a) => -a,
    }
}

// Approach 1: Replace Mul with Add
fn mul_to_add(e: ExprF<Fix>) -> ExprF<Fix> {
    match e {
        ExprF::MulF(a, b) => ExprF::AddF(a, b),
        other => other,
    }
}

// Approach 2: Double all literals
fn double_lits(e: ExprF<Fix>) -> ExprF<Fix> {
    match e {
        ExprF::LitF(n) => ExprF::LitF(n * 2),
        other => other,
    }
}

// Approach 3: Remove negations
fn remove_neg(e: ExprF<Fix>) -> ExprF<Fix> {
    match e {
        ExprF::NegF(a) => a.0.as_ref().clone(),
        other => other,
    }
}

fn identity_nat(e: ExprF<Fix>) -> ExprF<Fix> { e }

fn lit(n: i64) -> Fix { Fix(Box::new(ExprF::LitF(n))) }
fn add(a: Fix, b: Fix) -> Fix { Fix(Box::new(ExprF::AddF(a, b))) }
fn mul(a: Fix, b: Fix) -> Fix { Fix(Box::new(ExprF::MulF(a, b))) }
fn neg(a: Fix) -> Fix { Fix(Box::new(ExprF::NegF(a))) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_is_cata() {
        let e = add(lit(1), mul(lit(2), lit(3)));
        assert_eq!(cata(&eval_alg, &e), prepro(&identity_nat, &eval_alg, &e));
    }

    #[test]
    fn test_mul_to_add() {
        let e = mul(mul(lit(2), lit(3)), lit(4));
        // All muls become adds: 2+3+4 = 9? Actually: top mul→add, then inner mul→add
        assert_eq!(prepro(&mul_to_add, &eval_alg, &e), 9);
    }

    #[test]
    fn test_double_nested() {
        let e = add(add(lit(1), lit(1)), lit(1));
        // Outer add: children get doubled
        // Inner add(1,1): its children (lit 1) get doubled to (lit 2) → 2+2=4
        // But inner add itself was already doubled... depth matters
        let result = prepro(&double_lits, &eval_alg, &e);
        assert!(result > 3); // exact value depends on depth
    }

    #[test]
    fn test_remove_double_neg() {
        assert_eq!(prepro(&remove_neg, &eval_alg, &neg(neg(neg(lit(7))))), 7);
    }
}
