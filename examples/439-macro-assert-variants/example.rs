// 439. assert_matches! and variants

#[derive(Debug, PartialEq)]
enum Parsed { Int(i64), Float(f64), Invalid(String) }

fn parse(s: &str) -> Parsed {
    if let Ok(n) = s.parse::<i64>() { Parsed::Int(n) }
    else if let Ok(f) = s.parse::<f64>() { Parsed::Float(f) }
    else { Parsed::Invalid(s.to_string()) }
}

fn parse_positive(s: &str) -> Result<u32, String> {
    s.parse::<u32>().map_err(|e| e.to_string())
}

fn main() {
    let r = parse("42");
    // matches! — inline pattern check, no allocation
    assert!(matches!(r, Parsed::Int(n) if n == 42), "expected Int(42)");

    // Use in conditional
    if matches!(parse("3.14"), Parsed::Float(f) if f > 3.0) {
        println!("Got float > 3");
    }

    // assert_matches! equivalent (use matches! + assert!)
    let results = vec![
        parse_positive("1"),
        parse_positive("2"),
        parse_positive("3"),
    ];
    assert!(results.iter().all(|r| matches!(r, Ok(n) if *n > 0)));
    println!("All assertions passed!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_matches_int()   { assert!(matches!(parse("7"),     Parsed::Int(7))); }
    #[test] fn test_matches_float() { assert!(matches!(parse("1.5"),   Parsed::Float(_))); }
    #[test] fn test_matches_guard() { assert!(matches!(parse("100"),   Parsed::Int(n) if n > 50)); }
    #[test] fn test_matches_err()   { assert!(matches!(parse_positive("-1"), Err(_))); }
    #[test] fn test_matches_ok()    { assert!(matches!(parse_positive("5"),  Ok(5))); }
}
