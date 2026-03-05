// Example 059: Kleisli Composition
// Kleisli arrow: A -> Option<B>, composed to build pipelines

// Approach 1: Kleisli composition function
fn kleisli<A, B, C>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
) -> impl Fn(A) -> Option<C> {
    move |a| f(a).and_then(|b| g(b))
}

fn parse_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn check_positive(n: i32) -> Option<i32> {
    if n > 0 { Some(n) } else { None }
}

fn safe_half(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}

// Approach 2: Kleisli for Result
fn kleisli_result<A, B, C, E>(
    f: impl Fn(A) -> Result<B, E>,
    g: impl Fn(B) -> Result<C, E>,
) -> impl Fn(A) -> Result<C, E> {
    move |a| f(a).and_then(|b| g(b))
}

fn parse_r(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| "parse failed".into())
}

fn positive_r(n: i32) -> Result<i32, String> {
    if n > 0 { Ok(n) } else { Err("not positive".into()) }
}

fn even_r(n: i32) -> Result<i32, String> {
    if n % 2 == 0 { Ok(n) } else { Err("not even".into()) }
}

// Approach 3: Dynamic pipeline from Vec of Kleisli arrows
fn pipeline(steps: &[fn(i32) -> Option<i32>], x: i32) -> Option<i32> {
    steps.iter().fold(Some(x), |acc, step| acc.and_then(step))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kleisli_success() {
        let validate = kleisli(kleisli(parse_int, check_positive), safe_half);
        assert_eq!(validate("42"), Some(21));
    }

    #[test]
    fn test_kleisli_parse_fail() {
        let validate = kleisli(kleisli(parse_int, check_positive), safe_half);
        assert_eq!(validate("bad"), None);
    }

    #[test]
    fn test_kleisli_not_positive() {
        let validate = kleisli(kleisli(parse_int, check_positive), safe_half);
        assert_eq!(validate("0"), None);
    }

    #[test]
    fn test_kleisli_not_even() {
        let validate = kleisli(kleisli(parse_int, check_positive), safe_half);
        assert_eq!(validate("7"), None);
    }

    #[test]
    fn test_kleisli_result_success() {
        let v = kleisli_result(kleisli_result(parse_r, positive_r), even_r);
        assert_eq!(v("42"), Ok(42));
    }

    #[test]
    fn test_kleisli_result_errors() {
        let v = kleisli_result(kleisli_result(parse_r, positive_r), even_r);
        assert_eq!(v("bad"), Err("parse failed".to_string()));
        assert_eq!(v("-1"), Err("not positive".to_string()));
        assert_eq!(v("7"), Err("not even".to_string()));
    }

    #[test]
    fn test_pipeline() {
        let steps: Vec<fn(i32) -> Option<i32>> = vec![check_positive, safe_half];
        assert_eq!(pipeline(&steps, 50), Some(25));
        assert_eq!(pipeline(&steps, -1), None);
    }
}
