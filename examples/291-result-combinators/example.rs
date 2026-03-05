//! 291. map(), and_then(), or_else() on Result
//!
//! Result combinators enable composable error handling without nested match.

fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| format!("parse error: {}", e))
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err("division by zero".to_string()) }
    else { Ok(a / b) }
}

fn main() {
    // map: transform Ok value
    let doubled: Result<i32, String> = Ok(5).map(|x| x * 2);
    println!("map Ok(5): {:?}", doubled);

    // map on Err passes through
    let err: Result<i32, String> = Err("bad".to_string());
    println!("map Err: {:?}", err.map(|x| x * 2));

    // and_then: chain fallible operations (monadic bind)
    let result = parse_int("10").and_then(|n| divide(n, 2));
    println!("chain '10'/2: {:?}", result);

    // Short-circuit: first Err stops the chain
    let short = parse_int("abc").and_then(|n| divide(n, 2));
    println!("short-circuit: {:?}", short);

    // map_err: transform the error value
    let rich: Result<i32, String> = "bad".parse::<i32>()
        .map_err(|e| format!("Validation failed: {}", e));
    println!("map_err: {:?}", rich);

    // or_else: recover from errors
    let recovered: Result<i32, String> = parse_int("bad")
        .or_else(|_| Ok(42)); // default to 42 on error
    println!("or_else recovery: {:?}", recovered);

    // Complex chain
    let result = parse_int("20")
        .and_then(|n| divide(n, 4))
        .map(|n| n + 1)
        .map_err(|e| format!("Pipeline failed: {}", e));
    println!("Full pipeline: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_ok() {
        let r: Result<i32, &str> = Ok(5);
        assert_eq!(r.map(|x| x * 2), Ok(10));
    }

    #[test]
    fn test_and_then_chain() {
        let r = parse_int("10").and_then(|n| divide(n, 2));
        assert_eq!(r, Ok(5));
    }

    #[test]
    fn test_and_then_short_circuit() {
        let r = parse_int("abc").and_then(|n| divide(n, 2));
        assert!(r.is_err());
    }

    #[test]
    fn test_or_else_recovery() {
        let r: Result<i32, &str> = Err("bad").or_else(|_| Ok(0));
        assert_eq!(r, Ok(0));
    }

    #[test]
    fn test_map_err() {
        let r: Result<i32, &str> = Err("bad");
        let mapped = r.map_err(|e| format!("Error: {}", e));
        assert_eq!(mapped, Err("Error: bad".to_string()));
    }
}
