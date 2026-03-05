// Example 053: Applicative Functor Basics
// Applicative: apply a wrapped function to a wrapped value

#[derive(Debug, PartialEq, Clone)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> Maybe<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Nothing => Maybe::Nothing,
            Maybe::Just(x) => Maybe::Just(f(x)),
        }
    }

    fn pure(x: T) -> Maybe<T> {
        Maybe::Just(x)
    }
}

// Approach 1: Apply — apply a wrapped function to a wrapped value
impl<F> Maybe<F> {
    fn apply<A, B>(self, ma: Maybe<A>) -> Maybe<B>
    where
        F: FnOnce(A) -> B,
    {
        match (self, ma) {
            (Maybe::Just(f), Maybe::Just(a)) => Maybe::Just(f(a)),
            _ => Maybe::Nothing,
        }
    }
}

// Approach 2: lift2 / lift3 as free functions
fn lift2<A, B, C, F>(f: F, a: Maybe<A>, b: Maybe<B>) -> Maybe<C>
where
    F: FnOnce(A) -> Box<dyn FnOnce(B) -> C>,
{
    match (a, b) {
        (Maybe::Just(a), Maybe::Just(b)) => Maybe::Just(f(a)(b)),
        _ => Maybe::Nothing,
    }
}

// Simpler lift2 without currying
fn lift2_simple<A, B, C, F: FnOnce(A, B) -> C>(f: F, a: Maybe<A>, b: Maybe<B>) -> Maybe<C> {
    match (a, b) {
        (Maybe::Just(a), Maybe::Just(b)) => Maybe::Just(f(a, b)),
        _ => Maybe::Nothing,
    }
}

fn lift3_simple<A, B, C, D, F: FnOnce(A, B, C) -> D>(
    f: F, a: Maybe<A>, b: Maybe<B>, c: Maybe<C>,
) -> Maybe<D> {
    match (a, b, c) {
        (Maybe::Just(a), Maybe::Just(b), Maybe::Just(c)) => Maybe::Just(f(a, b, c)),
        _ => Maybe::Nothing,
    }
}

// Approach 3: Using Option's built-in zip (Rust's applicative)
fn option_applicative_example() -> Option<(i32, i32)> {
    let a = "42".parse::<i32>().ok();
    let b = "7".parse::<i32>().ok();
    a.zip(b) // Option's built-in applicative-like combinator
}

fn parse_int(s: &str) -> Maybe<i32> {
    match s.parse::<i32>() {
        Ok(n) => Maybe::Just(n),
        Err(_) => Maybe::Nothing,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_both_just() {
        let f = Maybe::Just(|x: i32| x * 2);
        assert_eq!(f.apply(Maybe::Just(5)), Maybe::Just(10));
    }

    #[test]
    fn test_apply_nothing_function() {
        let f: Maybe<fn(i32) -> i32> = Maybe::Nothing;
        assert_eq!(f.apply(Maybe::Just(5)), Maybe::Nothing);
    }

    #[test]
    fn test_apply_nothing_value() {
        let f = Maybe::Just(|x: i32| x * 2);
        assert_eq!(f.apply(Maybe::Nothing), Maybe::Nothing);
    }

    #[test]
    fn test_lift2_both_just() {
        assert_eq!(lift2_simple(|a: i32, b: i32| a + b, Maybe::Just(10), Maybe::Just(20)), Maybe::Just(30));
    }

    #[test]
    fn test_lift2_one_nothing() {
        assert_eq!(lift2_simple(|a: i32, b: i32| a + b, Maybe::Nothing, Maybe::Just(20)), Maybe::Nothing);
    }

    #[test]
    fn test_lift3() {
        let result = lift3_simple(
            |a: &str, b: &str, c: &str| format!("{}{}{}", a, b, c),
            Maybe::Just("x"), Maybe::Just("y"), Maybe::Just("z"),
        );
        assert_eq!(result, Maybe::Just("xyz".to_string()));
    }

    #[test]
    fn test_option_zip() {
        assert_eq!(option_applicative_example(), Some((42, 7)));
    }

    #[test]
    fn test_parse_and_combine() {
        let result = lift2_simple(|a: i32, b: i32| a + b, parse_int("42"), parse_int("8"));
        assert_eq!(result, Maybe::Just(50));
        let result2 = lift2_simple(|a: i32, b: i32| a + b, parse_int("bad"), parse_int("8"));
        assert_eq!(result2, Maybe::Nothing);
    }
}
