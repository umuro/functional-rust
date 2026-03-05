// Example 057: Monad Laws
// Law 1 (Left Identity):  return a >>= f  ≡  f(a)
// Law 2 (Right Identity): m >>= return    ≡  m
// Law 3 (Associativity):  (m >>= f) >>= g ≡ m >>= (|x| f(x) >>= g)

// Approach 1: Verify for Option
fn double(x: i32) -> Option<i32> { Some(x * 2) }
fn inc(x: i32) -> Option<i32> { Some(x + 1) }
fn safe_div10(x: i32) -> Option<i32> { if x == 0 { None } else { Some(10 / x) } }

fn verify_left_identity<A: Clone, B: PartialEq + std::fmt::Debug>(
    a: A, f: fn(A) -> Option<B>
) -> bool {
    Some(a.clone()).and_then(f) == f(a)
}

fn verify_right_identity<A: Clone + PartialEq + std::fmt::Debug>(m: Option<A>) -> bool {
    m.clone().and_then(Some) == m
}

fn verify_associativity<A: Clone + PartialEq + std::fmt::Debug>(
    m: Option<A>,
    f: fn(A) -> Option<A>,
    g: fn(A) -> Option<A>,
) -> bool {
    let left = m.clone().and_then(f).and_then(g);
    let right = m.and_then(|x| f(x).and_then(g));
    left == right
}

// Approach 2: Verify for Result
fn verify_result_left_identity<A: Clone, B: PartialEq + std::fmt::Debug>(
    a: A, f: fn(A) -> Result<B, String>
) -> bool {
    Ok::<A, String>(a.clone()).and_then(f) == f(a)
}

fn verify_result_right_identity<A: Clone + PartialEq + std::fmt::Debug>(
    m: Result<A, String>
) -> bool {
    m.clone().and_then(Ok) == m
}

// Approach 3: Verify for Vec (List monad)
fn vec_bind<A, B>(xs: Vec<A>, f: fn(&A) -> Vec<B>) -> Vec<B> {
    xs.iter().flat_map(f).collect()
}

fn verify_vec_left_identity<A: Clone + PartialEq + std::fmt::Debug, B: PartialEq + std::fmt::Debug>(
    a: A, f: fn(&A) -> Vec<B>
) -> bool {
    vec_bind(vec![a.clone()], f) == f(&a)
}


#[cfg(test)]
mod tests {
    use super::*;

    // Option monad laws
    #[test]
    fn test_option_left_identity() {
        assert!(verify_left_identity(5, double));
        assert!(verify_left_identity(0, safe_div10));
    }

    #[test]
    fn test_option_right_identity() {
        assert!(verify_right_identity(Some(42)));
        assert!(verify_right_identity(None::<i32>));
    }

    #[test]
    fn test_option_associativity() {
        assert!(verify_associativity(Some(5), double, inc));
        assert!(verify_associativity(None, double, inc));
        assert!(verify_associativity(Some(0), safe_div10, double));
    }

    // Result monad laws
    #[test]
    fn test_result_left_identity() {
        assert!(verify_result_left_identity(5, |x| Ok(x * 2)));
    }

    #[test]
    fn test_result_right_identity() {
        assert!(verify_result_right_identity(Ok::<i32, String>(42)));
        assert!(verify_result_right_identity(Err::<i32, String>("oops".into())));
    }

    // Vec (list) monad laws
    #[test]
    fn test_vec_left_identity() {
        let expand = |x: &i32| vec![*x, x * 10];
        assert!(verify_vec_left_identity(3, expand));
    }

    #[test]
    fn test_vec_associativity() {
        let xs = vec![1, 2];
        let f = |x: &i32| vec![*x, x * 10];
        let g = |x: &i32| vec![-x, *x];
        let left: Vec<i32> = vec_bind(vec_bind(xs.clone(), f), g);
        let right: Vec<i32> = xs.iter().flat_map(|x| {
            let fx = f(x);
            fx.iter().flat_map(g).collect::<Vec<_>>()
        }).collect();
        assert_eq!(left, right);
    }
}
