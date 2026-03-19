#![allow(clippy::all)]
// 082: Type Aliases
// Shortening complex types with type aliases

// Approach 1: Simple aliases
type Point = (f64, f64);
type Name = String;

fn distance(p1: Point, p2: Point) -> f64 {
    ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt()
}

// Approach 2: Result alias (common pattern)
#[derive(Debug, PartialEq)]
enum AppError {
    ParseError(String),
    DivByZero,
}

type AppResult<T> = Result<T, AppError>;

fn parse_int(s: &str) -> AppResult<i32> {
    s.parse()
        .map_err(|_| AppError::ParseError(format!("Not a number: {}", s)))
}

fn safe_div(a: i32, b: i32) -> AppResult<i32> {
    if b == 0 {
        Err(AppError::DivByZero)
    } else {
        Ok(a / b)
    }
}

// Approach 3: Complex type aliases
type Predicate<T> = Box<dyn Fn(&T) -> bool>;
type Transform<T> = Box<dyn Fn(T) -> T>;

fn filter_map_custom<T: Clone, U>(
    items: &[T],
    pred: &dyn Fn(&T) -> bool,
    f: &dyn Fn(T) -> U,
) -> Vec<U> {
    items
        .iter()
        .filter(|x| pred(x))
        .map(|x| f(x.clone()))
        .collect()
}

// io::Result pattern
type IoResult<T> = std::io::Result<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert!((distance((0.0, 0.0), (3.0, 4.0)) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_parse_int() {
        assert_eq!(parse_int("42"), Ok(42));
        assert!(parse_int("abc").is_err());
    }

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(10, 3), Ok(3));
        assert_eq!(safe_div(10, 0), Err(AppError::DivByZero));
    }

    #[test]
    fn test_filter_map() {
        let result = filter_map_custom(&[1, 2, 3, 4, 5, 6], &|x: &i32| x % 2 == 0, &|x: i32| x * 2);
        assert_eq!(result, vec![4, 8, 12]);
    }
}
