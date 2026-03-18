pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_world() {
        assert_eq!(greet("world"), "Hello, world!");
    }

    #[test]
    fn test_greet_name() {
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn test_greet_empty() {
        assert_eq!(greet(""), "Hello, !");
    }

    #[test]
    fn test_greet_with_spaces() {
        assert_eq!(greet("functional Rust"), "Hello, functional Rust!");
    }
}
