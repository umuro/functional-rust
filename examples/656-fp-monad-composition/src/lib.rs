//! # Monad Composition
//!
//! Unlike functors, monads don't compose directly. 
//! We need monad transformers to stack monadic effects.

/// OptionT monad transformer: adds Option to another monad M
pub struct OptionT<M, A> {
    run: M,
    _phantom: std::marker::PhantomData<A>,
}

// Approach 1: OptionT over Result
pub type OptionResult<A, E> = Result<Option<A>, E>;

pub fn pure_option_result<A, E>(a: A) -> OptionResult<A, E> {
    Ok(Some(a))
}

pub fn bind_option_result<A, B, E, F>(
    ma: OptionResult<A, E>,
    f: F,
) -> OptionResult<B, E>
where
    F: FnOnce(A) -> OptionResult<B, E>,
{
    match ma {
        Err(e) => Err(e),
        Ok(None) => Ok(None),
        Ok(Some(a)) => f(a),
    }
}

// Approach 2: ResultT over Option
pub type ResultOption<A, E> = Option<Result<A, E>>;

pub fn pure_result_option<A, E>(a: A) -> ResultOption<A, E> {
    Some(Ok(a))
}

pub fn bind_result_option<A, B, E, F>(
    ma: ResultOption<A, E>,
    f: F,
) -> ResultOption<B, E>
where
    F: FnOnce(A) -> ResultOption<B, E>,
{
    match ma {
        None => None,
        Some(Err(e)) => Some(Err(e)),
        Some(Ok(a)) => f(a),
    }
}

// Approach 3: StateT transformer
pub struct StateT<S, M, A> {
    run: Box<dyn FnOnce(S) -> (A, S, M)>,
}

// Practical: Combined Result and Vec (validation)
pub fn sequence_results<A, E: Clone>(results: Vec<Result<A, E>>) -> Result<Vec<A>, Vec<E>> {
    let mut successes = Vec::new();
    let mut errors = Vec::new();
    
    for r in results {
        match r {
            Ok(a) => successes.push(a),
            Err(e) => errors.push(e),
        }
    }
    
    if errors.is_empty() {
        Ok(successes)
    } else {
        Err(errors)
    }
}

/// EitherT: Either monad transformer
pub enum EitherT<L, M, R> {
    Left(L),
    Right(M, std::marker::PhantomData<R>),
}

/// Flatten nested Options
pub fn join_option<A>(opt: Option<Option<A>>) -> Option<A> {
    opt.flatten()
}

/// Flatten nested Results (requires same error type)
pub fn join_result<A, E>(res: Result<Result<A, E>, E>) -> Result<A, E> {
    res?
}

/// Lift Option into OptionResult
pub fn lift_option<A, E>(opt: Option<A>) -> OptionResult<A, E> {
    Ok(opt)
}

/// Lift Result into OptionResult
pub fn lift_result<A, E>(res: Result<A, E>) -> OptionResult<A, E> {
    res.map(Some)
}

// Practical example: parsing with multiple potential failures
pub fn parse_int_option(s: &str) -> Option<i32> {
    s.parse().ok()
}

pub fn parse_int_result(s: &str) -> Result<i32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

pub fn parse_and_validate(s: &str, min: i32, max: i32) -> OptionResult<i32, String> {
    let n = match parse_int_result(s) {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    
    if n >= min && n <= max {
        Ok(Some(n))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_result_pure() {
        let x: OptionResult<i32, &str> = pure_option_result(42);
        assert_eq!(x, Ok(Some(42)));
    }

    #[test]
    fn test_option_result_bind() {
        let x: OptionResult<i32, &str> = Ok(Some(21));
        let result = bind_option_result(x, |n| Ok(Some(n * 2)));
        assert_eq!(result, Ok(Some(42)));
    }

    #[test]
    fn test_option_result_none() {
        let x: OptionResult<i32, &str> = Ok(None);
        let result = bind_option_result(x, |n| Ok(Some(n * 2)));
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn test_option_result_error() {
        let x: OptionResult<i32, &str> = Err("error");
        let result = bind_option_result(x, |n| Ok(Some(n * 2)));
        assert_eq!(result, Err("error"));
    }

    #[test]
    fn test_result_option_bind() {
        let x: ResultOption<i32, &str> = Some(Ok(21));
        let result = bind_result_option(x, |n| Some(Ok(n * 2)));
        assert_eq!(result, Some(Ok(42)));
    }

    #[test]
    fn test_sequence_results_success() {
        let results: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
        assert_eq!(sequence_results(results), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_sequence_results_errors() {
        let results: Vec<Result<i32, &str>> = vec![Ok(1), Err("e1"), Err("e2")];
        assert_eq!(sequence_results(results), Err(vec!["e1", "e2"]));
    }

    #[test]
    fn test_join_option() {
        assert_eq!(join_option(Some(Some(42))), Some(42));
        assert_eq!(join_option(Some(None::<i32>)), None);
        assert_eq!(join_option(None::<Option<i32>>), None);
    }

    #[test]
    fn test_join_result() {
        let nested: Result<Result<i32, &str>, &str> = Ok(Ok(42));
        assert_eq!(join_result(nested), Ok(42));
    }

    #[test]
    fn test_parse_and_validate() {
        assert_eq!(parse_and_validate("50", 0, 100), Ok(Some(50)));
        assert_eq!(parse_and_validate("150", 0, 100), Ok(None));
        assert!(parse_and_validate("abc", 0, 100).is_err());
    }

    #[test]
    fn test_lift_option() {
        assert_eq!(lift_option::<_, &str>(Some(42)), Ok(Some(42)));
        assert_eq!(lift_option::<i32, &str>(None), Ok(None));
    }

    #[test]
    fn test_lift_result() {
        let ok: Result<i32, &str> = Ok(42);
        let err: Result<i32, &str> = Err("e");
        assert_eq!(lift_result(ok), Ok(Some(42)));
        assert_eq!(lift_result(err), Err("e"));
    }
}
