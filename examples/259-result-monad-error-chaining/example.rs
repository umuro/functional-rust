// Solution 1: Idiomatic Rust — Result's built-in monadic combinator
// `and_then` is Rust's bind (>>=) for Result: propagates Err, applies f to Ok
pub fn parse_int(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|_| format!("Not an integer: {s}"))
}

pub fn check_positive(n: i64) -> Result<i64, String> {
    if n > 0 {
        Ok(n)
    } else {
        Err("Must be positive".to_string())
    }
}

pub fn check_even(n: i64) -> Result<i64, String> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err("Must be even".to_string())
    }
}

// Railway-oriented: each step either advances the train or diverts to the error track
pub fn validate_idiomatic(s: &str) -> Result<i64, String> {
    parse_int(s).and_then(check_positive).and_then(check_even)
}

// Solution 2: Explicit bind — mirrors OCaml's >>= operator exactly
fn bind<T, U, E>(r: Result<T, E>, f: impl FnOnce(T) -> Result<U, E>) -> Result<U, E> {
    match r {
        Err(e) => Err(e),
        Ok(x) => f(x),
    }
}

pub fn validate_explicit(s: &str) -> Result<i64, String> {
    bind(bind(parse_int(s), check_positive), check_even)
}

// Solution 3: Using the `?` operator — Rust's ergonomic monadic shorthand
pub fn validate_question_mark(s: &str) -> Result<i64, String> {
    let n = parse_int(s)?;
    let n = check_positive(n)?;
    check_even(n)
}

fn main() {
    let inputs = ["42", "-3", "abc", "7"];

    println!("=== validate_idiomatic (and_then chain) ===");
    for s in &inputs {
        match validate_idiomatic(s) {
            Ok(n) => println!("{s} -> Ok {n}"),
            Err(e) => println!("{s} -> Error: {e}"),
        }
    }

    println!("\n=== validate_question_mark (? operator) ===");
    for s in &inputs {
        match validate_question_mark(s) {
            Ok(n) => println!("{s} -> Ok {n}"),
            Err(e) => println!("{s} -> Error: {e}"),
        }
    }

    println!("\n=== validate_explicit (explicit bind) ===");
    for s in &inputs {
        match validate_explicit(s) {
            Ok(n) => println!("{s} -> Ok {n}"),
            Err(e) => println!("{s} -> Error: {e}"),
        }
    }
}

/* Output:
   === validate_idiomatic (and_then chain) ===
   42 -> Ok 42
   -3 -> Error: Must be positive
   abc -> Error: Not an integer: abc
   7 -> Error: Must be even

   === validate_question_mark (? operator) ===
   42 -> Ok 42
   -3 -> Error: Must be positive
   abc -> Error: Not an integer: abc
   7 -> Error: Must be even

   === validate_explicit (explicit bind) ===
   42 -> Ok 42
   -3 -> Error: Must be positive
   abc -> Error: Not an integer: abc
   7 -> Error: Must be even
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_positive_even_succeeds() {
        assert_eq!(validate_idiomatic("42"), Ok(42));
        assert_eq!(validate_explicit("42"), Ok(42));
        assert_eq!(validate_question_mark("42"), Ok(42));
    }

    #[test]
    fn test_negative_number_fails_check_positive() {
        let expected = Err("Must be positive".to_string());
        assert_eq!(validate_idiomatic("-3"), expected);
        assert_eq!(validate_explicit("-3"), expected);
        assert_eq!(validate_question_mark("-3"), expected);
    }

    #[test]
    fn test_non_integer_string_fails_parse() {
        let expected = Err("Not an integer: abc".to_string());
        assert_eq!(validate_idiomatic("abc"), expected);
        assert_eq!(validate_explicit("abc"), expected);
        assert_eq!(validate_question_mark("abc"), expected);
    }

    #[test]
    fn test_positive_odd_fails_check_even() {
        let expected = Err("Must be even".to_string());
        assert_eq!(validate_idiomatic("7"), expected);
        assert_eq!(validate_explicit("7"), expected);
        assert_eq!(validate_question_mark("7"), expected);
    }

    #[test]
    fn test_zero_fails_check_positive() {
        let expected = Err("Must be positive".to_string());
        assert_eq!(validate_idiomatic("0"), expected);
        assert_eq!(validate_explicit("0"), expected);
        assert_eq!(validate_question_mark("0"), expected);
    }

    #[test]
    fn test_parse_int_valid() {
        assert_eq!(parse_int("100"), Ok(100));
        assert_eq!(parse_int("-5"), Ok(-5));
        assert_eq!(parse_int("0"), Ok(0));
    }

    #[test]
    fn test_parse_int_invalid() {
        assert!(parse_int("abc").is_err());
        assert!(parse_int("1.5").is_err());
        assert!(parse_int("").is_err());
    }

    #[test]
    fn test_error_stops_at_first_failure() {
        let result = validate_idiomatic("abc");
        assert!(result.unwrap_err().contains("Not an integer"));
    }
}
