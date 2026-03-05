// Option.map and Option.bind — Safe Value Transformation
// Demonstrates chaining operations on optional values using
// `Option::map` and `Option::and_then` (OCaml's `Option.bind`).

// Solution 1: Idiomatic Rust — method chaining on Option
pub fn parse_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

pub fn safe_div(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

/// Chain: parse → double → divide — idiomatic method chaining
pub fn parse_double_divide(s: &str, divisor: i32) -> Option<i32> {
    parse_int(s)
        .map(|x| x * 2)
        .and_then(|x| safe_div(x, divisor))
}

// Solution 2: Explicit early-return pattern matching — mirrors OCaml exhaustive match style
pub fn parse_double_divide_explicit(s: &str, divisor: i32) -> Option<i32> {
    let n = match s.parse::<i32>() {
        Ok(n) => n,
        Err(_) => return None,
    };
    let doubled = n * 2;
    match divisor {
        0 => None,
        d => Some(doubled / d),
    }
}

// Solution 3: Using the `?` operator — idiomatic Rust for fallible chains
pub fn parse_double_divide_question(s: &str, divisor: i32) -> Option<i32> {
    let n: i32 = s.parse().ok()?;
    let doubled = n * 2;
    safe_div(doubled, divisor)
}

fn main() {
    // Happy path: "42" -> 42 -> 84 -> 84/7 = 12
    println!(
        "parse_double_divide(\"42\", 7) = {:?}",
        parse_double_divide("42", 7)
    );

    // Parse fails: None propagates through the whole chain
    println!(
        "parse_double_divide(\"abc\", 7) = {:?}",
        parse_double_divide("abc", 7)
    );

    // Division by zero: and_then returns None
    println!(
        "parse_double_divide(\"42\", 0) = {:?}",
        parse_double_divide("42", 0)
    );

    // Explicit match style — same results
    println!(
        "parse_double_divide_explicit(\"42\", 7) = {:?}",
        parse_double_divide_explicit("42", 7)
    );

    // ? operator style — same results
    println!(
        "parse_double_divide_question(\"42\", 7) = {:?}",
        parse_double_divide_question("42", 7)
    );
}

/* Output:
   parse_double_divide("42", 7) = Some(12)
   parse_double_divide("abc", 7) = None
   parse_double_divide("42", 0) = None
   parse_double_divide_explicit("42", 7) = Some(12)
   parse_double_divide_question("42", 7) = Some(12)
*/
