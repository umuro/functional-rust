//! 292. map(), filter(), and_then() on Option
//!
//! Option combinators replace verbose match expressions with composable chains.

fn safe_sqrt(x: f64) -> Option<f64> {
    if x >= 0.0 { Some(x.sqrt()) } else { None }
}

fn main() {
    let some5: Option<i32> = Some(5);
    let none: Option<i32> = None;

    // map: transform Some, pass through None
    println!("map Some(5)*2: {:?}", some5.map(|x| x * 2));
    println!("map None*2:    {:?}", none.map(|x| x * 2));

    // filter: keep Some only if predicate holds
    println!("filter even Some(5): {:?}", some5.filter(|&x| x % 2 == 0));
    println!("filter even Some(6): {:?}", Some(6i32).filter(|&x| x % 2 == 0));

    // and_then: chain optional computations (monadic bind)
    let config_str: Option<&str> = Some("4.0");
    let result = config_str
        .and_then(|s| s.parse::<f64>().ok())
        .and_then(safe_sqrt);
    println!("Parse and sqrt '4.0': {:?}", result);

    // Short-circuit on None
    let bad: Option<&str> = None;
    let result2 = bad.and_then(|s| s.parse::<f64>().ok()).and_then(safe_sqrt);
    println!("None chain: {:?}", result2);

    // or and or_else: provide defaults
    let default = none.or(Some(42));
    println!("None.or(Some(42)): {:?}", default);

    let computed = none.or_else(|| {
        println!("  (computing default...)");
        Some(99)
    });
    println!("or_else: {:?}", computed);

    // Chain of operations on a user lookup
    let users = std::collections::HashMap::from([
        ("alice", 30u32), ("bob", 25),
    ]);
    let age_squared = users.get("alice")
        .map(|&age| age * age);
    println!("alice's age²: {:?}", age_squared);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_map_some() {
        assert_eq!(Some(5i32).map(|x| x * 2), Some(10));
    }

    #[test]
    fn test_map_none() {
        assert_eq!(None::<i32>.map(|x| x * 2), None);
    }

    #[test]
    fn test_filter() {
        assert_eq!(Some(4i32).filter(|&x| x % 2 == 0), Some(4));
        assert_eq!(Some(3i32).filter(|&x| x % 2 == 0), None);
    }

    #[test]
    fn test_and_then_chain() {
        let result = Some("4.0")
            .and_then(|s| s.parse::<f64>().ok())
            .map(|x| x * x);
        assert!((result.unwrap() - 16.0).abs() < 1e-10);
    }

    #[test]
    fn test_or_default() {
        let result = None::<i32>.or(Some(42));
        assert_eq!(result, Some(42));
    }
}
