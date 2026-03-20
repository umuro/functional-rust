#![allow(clippy::all)]
// Example 082: Type Aliases
// type keyword in both languages — aliases vs newtypes

// === Approach 1: Simple type aliases ===
type UserId = u64;
type Name = String;
type Age = u32;

struct User {
    id: UserId,
    name: Name,
    age: Age,
}

fn create_user(id: UserId, name: Name, age: Age) -> User {
    User { id, name, age }
}

// === Approach 2: Generic type aliases ===
type ResultWithMsg<T> = Option<(T, String)>;
type Validator<T> = fn(&T) -> bool;
type Transform<A, B> = fn(A) -> B;

fn validate_positive(x: &i32) -> bool {
    *x > 0
}

// === Approach 3: Complex type aliases for readability ===
type Point = (f64, f64);
type Polygon = Vec<Point>;
type Predicate<T> = Box<dyn Fn(&T) -> bool>;

fn distance(a: Point, b: Point) -> f64 {
    ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt()
}

fn perimeter(poly: &[Point]) -> f64 {
    if poly.len() < 2 {
        return 0.0;
    }
    let mut total = 0.0;
    for i in 0..poly.len() {
        let next = (i + 1) % poly.len();
        total += distance(poly[i], poly[next]);
    }
    total
}

// Type alias for Result with common error
type AppResult<T> = Result<T, String>;

fn parse_age(s: &str) -> AppResult<Age> {
    s.parse::<u32>().map_err(|e| e.to_string())
}

// NOTE: Type aliases do NOT create new types!
// UserId and u64 are interchangeable — no type safety
fn demonstrate_alias_transparency() -> bool {
    let id: UserId = 42;
    let raw: u64 = id; // No error — same type!
    raw == 42
}

fn filter_with<'a, T>(items: &'a [T], pred: &dyn Fn(&T) -> bool) -> Vec<&'a T> {
    items.iter().filter(|x| pred(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let u = create_user(1, "Bob".into(), 25);
        assert_eq!(u.id, 1);
        assert_eq!(u.name, "Bob");
        assert_eq!(u.age, 25);
    }

    #[test]
    fn test_alias_is_transparent() {
        let id: UserId = 42;
        let raw: u64 = id;
        assert_eq!(raw, 42);
    }

    #[test]
    fn test_validator() {
        let v: Validator<i32> = validate_positive;
        assert!(v(&5));
        assert!(!v(&-1));
        assert!(!v(&0));
    }

    #[test]
    fn test_distance() {
        assert!((distance((0.0, 0.0), (3.0, 4.0)) - 5.0).abs() < 1e-10);
        assert!((distance((1.0, 1.0), (1.0, 1.0))).abs() < 1e-10);
    }

    #[test]
    fn test_perimeter() {
        let square: Polygon = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        assert!((perimeter(&square) - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_parse_age() {
        assert_eq!(parse_age("25"), Ok(25));
        assert!(parse_age("abc").is_err());
    }

    #[test]
    fn test_filter_with() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let evens = filter_with(&nums, &|x| x % 2 == 0);
        assert_eq!(evens, vec![&2, &4, &6]);
    }
}
