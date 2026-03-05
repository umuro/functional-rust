/// Parse a string into an integer, returning an error string on failure.
///
/// OCaml: `int_of_string_opt` wrapped in `Result`
pub fn parse_int(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|_| format!("Not a number: {s}"))
}

/// Require n > 0.
pub fn check_positive(n: i64) -> Result<i64, String> {
    if n > 0 {
        Ok(n)
    } else {
        Err("Must be positive".to_string())
    }
}

/// Require n <= 100.
pub fn check_range(n: i64) -> Result<i64, String> {
    if n <= 100 {
        Ok(n)
    } else {
        Err("Must be <= 100".to_string())
    }
}

/// Idiomatic Rust: `and_then` chains (= OCaml `Result.bind`), `map` transforms.
///
/// Pipeline: parse → check_positive → check_range → double
pub fn validate(s: &str) -> Result<i64, String> {
    parse_int(s)
        .and_then(check_positive)
        .and_then(check_range)
        .map(|n| n * 2)
}

/// Functional style using the `?` operator — short-circuits on the first error.
/// Cleaner when there are many steps; mirrors OCaml monadic `let*` notation.
pub fn validate_question_mark(s: &str) -> Result<i64, String> {
    let n = parse_int(s)?;
    let n = check_positive(n)?;
    let n = check_range(n)?;
    Ok(n * 2)
}

fn main() {
    let cases = ["42", "0", "-5", "100", "101", "abc"];
    for s in cases {
        match validate(s) {
            Ok(v) => println!("validate({s:?}) = Ok({v})"),
            Err(e) => println!("validate({s:?}) = Err({e:?})"),
        }
    }

    println!();
    println!("--- same results via ? operator ---");
    for s in cases {
        match validate_question_mark(s) {
            Ok(v) => println!("validate_question_mark({s:?}) = Ok({v})"),
            Err(e) => println!("validate_question_mark({s:?}) = Err({e:?})"),
        }
    }
}

/* Output:
   validate("42")  = Ok(84)
   validate("0")   = Err("Must be positive")
   validate("-5")  = Err("Must be positive")
   validate("100") = Ok(200)
   validate("101") = Err("Must be <= 100")
   validate("abc") = Err("Not a number: abc")

   --- same results via ? operator ---
   validate_question_mark("42")  = Ok(84)
   validate_question_mark("0")   = Err("Must be positive")
   validate_question_mark("-5")  = Err("Must be positive")
   validate_question_mark("100") = Ok(200)
   validate_question_mark("101") = Err("Must be <= 100")
   validate_question_mark("abc") = Err("Not a number: abc")
*/
