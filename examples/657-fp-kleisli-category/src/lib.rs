//! # Kleisli Category
//!
//! The Kleisli category of a monad M has:
//! - Objects: types
//! - Morphisms: A -> M<B> (Kleisli arrows)
//! - Composition: (>=>) Kleisli composition

/// Kleisli arrow: A -> M<B>
pub type Kleisli<A, M> = Box<dyn FnOnce(A) -> M>;

// Approach 1: Kleisli for Option
pub type KleisliOption<A, B> = Box<dyn FnOnce(A) -> Option<B>>;

/// Kleisli composition for Option: (>=>) fish operator
pub fn kleisli_compose_option<A: 'static, B: 'static, C: 'static>(
    f: impl FnOnce(A) -> Option<B> + 'static,
    g: impl FnOnce(B) -> Option<C> + 'static,
) -> impl FnOnce(A) -> Option<C> {
    move |a| f(a).and_then(g)
}

/// Identity Kleisli arrow for Option
pub fn kleisli_id_option<A>() -> impl Fn(A) -> Option<A> {
    |a| Some(a)
}

// Approach 2: Kleisli for Result
pub fn kleisli_compose_result<A, B, C, E>(
    f: impl FnOnce(A) -> Result<B, E>,
    g: impl FnOnce(B) -> Result<C, E>,
) -> impl FnOnce(A) -> Result<C, E> {
    move |a| f(a).and_then(g)
}

pub fn kleisli_id_result<A, E>() -> impl Fn(A) -> Result<A, E> {
    |a| Ok(a)
}

// Approach 3: Kleisli for Vec (list monad)
pub fn kleisli_compose_vec<A: Clone, B: Clone, C>(
    f: impl Fn(A) -> Vec<B>,
    g: impl Fn(B) -> Vec<C>,
) -> impl Fn(A) -> Vec<C> {
    move |a| f(a).into_iter().flat_map(|b| g(b.clone())).collect()
}

pub fn kleisli_id_vec<A: Clone>() -> impl Fn(A) -> Vec<A> {
    |a| vec![a]
}

/// Practical: Compose validation functions
pub fn validate_non_empty(s: String) -> Option<String> {
    if s.is_empty() { None } else { Some(s) }
}

pub fn validate_length(s: String) -> Option<String> {
    if s.len() <= 100 { Some(s) } else { None }
}

pub fn validate_no_spaces(s: String) -> Option<String> {
    if s.contains(' ') { None } else { Some(s) }
}

/// Compose all validators
pub fn validate_username(s: String) -> Option<String> {
    kleisli_compose_option(
        kleisli_compose_option(
            validate_non_empty,
            validate_length
        ),
        validate_no_spaces
    )(s)
}

/// Reader monad Kleisli composition
pub fn kleisli_compose_reader<R: Clone, A, B, C>(
    f: impl Fn(A) -> Box<dyn Fn(R) -> B>,
    g: impl Fn(B) -> Box<dyn Fn(R) -> C>,
) -> impl Fn(A) -> Box<dyn Fn(R) -> C>
where
    R: 'static,
    B: 'static,
    C: 'static,
{
    move |a| {
        let f_result = f(a);
        Box::new(move |r: R| {
            let b = f_result(r.clone());
            g(b)(r)
        })
    }
}

/// Fish operator (>=>) as a generic concept
pub trait KleisliOps<A, B, M> {
    fn fish<C, G>(self, g: G) -> Box<dyn FnOnce(A) -> M>
    where
        G: FnOnce(B) -> M + 'static;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kleisli_option_compose() {
        let f = |x: i32| if x > 0 { Some(x) } else { None };
        let g = |x: i32| Some(x * 2);
        
        let composed = kleisli_compose_option(f, g);
        
        assert_eq!(composed(5), Some(10));
        assert_eq!(composed(-1), None);
    }

    #[test]
    fn test_kleisli_option_identity() {
        let id = kleisli_id_option::<i32>();
        assert_eq!(id(42), Some(42));
    }

    #[test]
    fn test_kleisli_option_associativity() {
        let f = |x: i32| Some(x + 1);
        let g = |x: i32| Some(x * 2);
        let h = |x: i32| Some(x - 1);
        
        // (f >=> g) >=> h
        let left = kleisli_compose_option(
            kleisli_compose_option(f, g),
            h
        );
        
        // f >=> (g >=> h) - needs clones for closures
        let f2 = |x: i32| Some(x + 1);
        let right = kleisli_compose_option(
            f2,
            kleisli_compose_option(
                |x: i32| Some(x * 2),
                |x: i32| Some(x - 1)
            )
        );
        
        assert_eq!(left(5), right(5));
    }

    #[test]
    fn test_kleisli_result_compose() {
        let f = |x: i32| -> Result<i32, &str> {
            if x > 0 { Ok(x) } else { Err("negative") }
        };
        let g = |x: i32| -> Result<String, &str> {
            Ok(x.to_string())
        };
        
        let composed = kleisli_compose_result(f, g);
        
        assert_eq!(composed(5), Ok("5".to_string()));
        assert_eq!(composed(-1), Err("negative"));
    }

    #[test]
    fn test_kleisli_vec_compose() {
        let f = |x: i32| vec![x, x + 1];
        let g = |x: i32| vec![x * 10];
        
        let composed = kleisli_compose_vec(f, g);
        
        assert_eq!(composed(1), vec![10, 20]);
    }

    #[test]
    fn test_validate_username_valid() {
        assert_eq!(
            validate_username("alice123".to_string()),
            Some("alice123".to_string())
        );
    }

    #[test]
    fn test_validate_username_empty() {
        assert_eq!(validate_username("".to_string()), None);
    }

    #[test]
    fn test_validate_username_spaces() {
        assert_eq!(validate_username("alice bob".to_string()), None);
    }

    #[test]
    fn test_validate_username_too_long() {
        let long = "a".repeat(101);
        assert_eq!(validate_username(long), None);
    }
}
