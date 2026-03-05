//! 305. unwrap_or, unwrap_or_else, unwrap_or_default
//!
//! Safe alternatives to `unwrap()` when a default value is available.

#[derive(Debug, Default)]
struct Config {
    timeout: u32,
    retries: u32,
}

fn main() {
    let some42: Option<i32> = Some(42);
    let none: Option<i32> = None;

    // unwrap_or: eager default (always evaluated)
    println!("Some(42).unwrap_or(0) = {}", some42.unwrap_or(0));
    println!("None.unwrap_or(0)     = {}", none.unwrap_or(0));

    // unwrap_or_else: lazy default (only evaluated if None)
    println!("Some(42).unwrap_or_else(||99) = {}",
        some42.unwrap_or_else(|| { println!("  (evaluating default)"); 99 }));
    println!("None.unwrap_or_else(||99) = {}",
        none.unwrap_or_else(|| { println!("  (evaluating default)"); 99 }));

    // unwrap_or_default: uses Default trait
    let none_vec: Option<Vec<i32>> = None;
    println!("None::<Vec<i32>>.unwrap_or_default() = {:?}", none_vec.unwrap_or_default());

    let none_str: Option<String> = None;
    println!("None::<String>.unwrap_or_default() = '{}'", none_str.unwrap_or_default());

    // On Result
    let ok: Result<i32, &str> = Ok(7);
    let err: Result<i32, &str> = Err("bad");
    println!("Ok(7).unwrap_or(0)  = {}", ok.unwrap_or(0));
    println!("Err.unwrap_or(0)    = {}", err.unwrap_or(0));

    // Practical: config parsing with defaults
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    println!("Port: {}", port);

    let config = None::<Config>.unwrap_or_default();
    println!("Default config: {:?}", config);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unwrap_or_some() {
        assert_eq!(Some(5i32).unwrap_or(0), 5);
    }

    #[test]
    fn test_unwrap_or_none() {
        assert_eq!(None::<i32>.unwrap_or(0), 0);
    }

    #[test]
    fn test_unwrap_or_else_lazy() {
        let mut called = false;
        let _: i32 = Some(5).unwrap_or_else(|| { called = true; 0 });
        assert!(!called); // not called when Some
    }

    #[test]
    fn test_unwrap_or_default_vec() {
        let v: Vec<i32> = None::<Vec<i32>>.unwrap_or_default();
        assert!(v.is_empty());
    }
}
