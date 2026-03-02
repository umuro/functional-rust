// Algebraic Data Types: Option and Result

// Option type for nullable values
fn safe_divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn find_index<T, F>(pred: F, lst: &[T]) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    lst.iter().position(pred)
}

// Result type for error handling
fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("Not a valid integer: {}", s))
}

fn safe_sqrt(x: f64) -> Result<f64, &'static str> {
    if x < 0.0 {
        Err("Cannot sqrt negative")
    } else {
        Ok(x.sqrt())
    }
}

// Chaining with and_then (monadic bind)
// Option already has .and_then() and .map()

// Combinators
fn option_default<T>(default: T, opt: Option<T>) -> T {
    opt.unwrap_or(default)
}

// Examples using Rust's built-in methods
fn chained_division() -> Option<i32> {
    safe_divide(100, 5)
        .and_then(|x| safe_divide(x, 2))
        .map(|x| x * 2)
}

fn chained_computation() -> Result<f64, String> {
    parse_int("16")
        .map(|x| x as f64)
        .and_then(|x| safe_sqrt(x).map_err(|e| e.to_string()))
        .map(|x| x * 2.0)
}

fn main() {
    // Option examples
    match safe_divide(10, 2) {
        Some(n) => println!("10 / 2 = {}", n),
        None => println!("Division by zero"),
    }
    
    match safe_divide(10, 0) {
        Some(n) => println!("10 / 0 = {}", n),
        None => println!("Division by zero"),
    }
    
    let numbers = vec![1, 3, 5, 7];
    match find_index(|x| *x > 4, &numbers) {
        Some(i) => println!("First > 4 at index {}", i),
        None => println!("Not found"),
    }
    
    // Result examples
    match parse_int("42") {
        Ok(n) => println!("Parsed: {}", n),
        Err(msg) => println!("Error: {}", msg),
    }
    
    match parse_int("hello") {
        Ok(n) => println!("Parsed: {}", n),
        Err(msg) => println!("Error: {}", msg),
    }
    
    // Chaining
    match chained_division() {
        Some(n) => println!("Chained: {}", n),
        None => println!("Chained: None"),
    }
    
    match chained_computation() {
        Ok(n) => println!("Sqrt chain: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(10, 0), None);
    }

    #[test]
    fn test_find_index() {
        assert_eq!(find_index(|x| *x > 4, &[1, 3, 5, 7]), Some(2));
        assert_eq!(find_index(|x| *x > 10, &[1, 3, 5]), None);
    }

    #[test]
    fn test_parse_int() {
        assert!(parse_int("42").is_ok());
        assert!(parse_int("hello").is_err());
    }

    #[test]
    fn test_chaining() {
        assert_eq!(chained_division(), Some(10));
    }
}
