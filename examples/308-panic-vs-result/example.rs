//! 308. When to panic vs return Result
//!
//! Guidelines: Result for recoverable errors, panic for programming bugs/invariant violations.

/// Library function: user provides invalid input -> use Result
fn parse_age(s: &str) -> Result<u8, String> {
    let n: i32 = s.parse().map_err(|_| format!("'{}' is not a number", s))?;
    if n < 0 || n > 150 {
        return Err(format!("age {} is out of range [0, 150]", n));
    }
    Ok(n as u8)
}

/// Internal function: programmer error -> can panic
fn get_element<T>(arr: &[T], index: usize) -> &T {
    // Panicking is OK here: wrong index is a programming bug, not user error
    &arr[index] // will panic with descriptive message if out of bounds
}

/// Invariant that must always hold internally
fn divide(a: i32, b: i32) -> i32 {
    assert!(b != 0, "divide: b must not be zero (caller's responsibility)");
    a / b
}

/// Application entry point: can convert errors to panics
fn process_user_input(age_str: &str) {
    match parse_age(age_str) {
        Ok(age) => println!("Valid age: {}", age),
        Err(e)  => println!("Invalid input: {}", e),
    }
}

fn main() {
    // Result for user-facing errors
    println!("--- Result pattern ---");
    process_user_input("25");
    process_user_input("200");
    process_user_input("abc");

    // Panic for programming bugs
    println!("--- Panic pattern ---");
    let arr = [1i32, 2, 3, 4, 5];
    println!("Element 2: {}", get_element(&arr, 2));

    // unwrap() in tests/prototypes is OK; in production prefer ?
    let result: Result<i32, &str> = Ok(42);
    let val = result.expect("this should never be Err in production code");
    println!("Expected value: {}", val);

    // unreachable! for exhaustive match that should never hit a branch
    let x = 5u32;
    let _msg = match x {
        0 => "zero",
        1..=9 => "single digit",
        _ => "multi-digit",
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_age_valid() {
        assert_eq!(parse_age("25"), Ok(25));
        assert_eq!(parse_age("0"), Ok(0));
        assert_eq!(parse_age("150"), Ok(150));
    }

    #[test]
    fn test_parse_age_invalid() {
        assert!(parse_age("abc").is_err());
        assert!(parse_age("200").is_err());
        assert!(parse_age("-1").is_err());
    }

    #[test]
    fn test_get_element() {
        let arr = [10i32, 20, 30];
        assert_eq!(*get_element(&arr, 1), 20);
    }

    #[test]
    #[should_panic]
    fn test_get_element_panics_out_of_bounds() {
        let arr = [1i32];
        get_element(&arr, 99); // should panic
    }
}
