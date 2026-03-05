// 073: Parse Don't Validate — Validated Types

// Approach 1: NonEmptyString
#[derive(Debug, Clone, PartialEq)]
struct NonEmptyString(String); // private field!

impl NonEmptyString {
    fn new(s: &str) -> Option<Self> {
        if s.is_empty() { None } else { Some(NonEmptyString(s.to_string())) }
    }

    fn as_str(&self) -> &str {
        &self.0
    }

    fn len(&self) -> usize {
        self.0.len() // always >= 1
    }
}

// Approach 2: PositiveInt
#[derive(Debug, Clone, Copy, PartialEq)]
struct PositiveInt(u32); // private field, always > 0

impl PositiveInt {
    fn new(n: i32) -> Option<Self> {
        if n <= 0 { None } else { Some(PositiveInt(n as u32)) }
    }

    fn value(&self) -> u32 {
        self.0
    }

    fn add(self, other: Self) -> Self {
        PositiveInt(self.0 + other.0) // sum of positives is positive
    }
}

// Approach 3: Email
#[derive(Debug, Clone, PartialEq)]
struct Email(String);

impl Email {
    fn new(s: &str) -> Result<Self, String> {
        if !s.contains('@') {
            Err("Missing @".into())
        } else if s.len() < 3 {
            Err("Too short".into())
        } else {
            Ok(Email(s.to_string()))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

// Using validated types — no further validation needed
fn greet(name: &NonEmptyString) -> String {
    format!("Hello, {}!", name.as_str())
}

fn double_positive(n: PositiveInt) -> u32 {
    n.value() * 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_string() {
        assert!(NonEmptyString::new("").is_none());
        assert!(NonEmptyString::new("hello").is_some());
        let s = NonEmptyString::new("Alice").unwrap();
        assert_eq!(s.as_str(), "Alice");
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_positive_int() {
        assert!(PositiveInt::new(0).is_none());
        assert!(PositiveInt::new(-5).is_none());
        let n = PositiveInt::new(42).unwrap();
        assert_eq!(n.value(), 42);
        assert_eq!(double_positive(n), 84);
    }

    #[test]
    fn test_positive_add() {
        let a = PositiveInt::new(3).unwrap();
        let b = PositiveInt::new(4).unwrap();
        assert_eq!(a.add(b).value(), 7);
    }

    #[test]
    fn test_email() {
        assert!(Email::new("bad").is_err());
        assert!(Email::new("a@b.com").is_ok());
        assert_eq!(Email::new("a@b.com").unwrap().as_str(), "a@b.com");
    }

    #[test]
    fn test_greet() {
        let name = NonEmptyString::new("Alice").unwrap();
        assert_eq!(greet(&name), "Hello, Alice!");
    }
}
