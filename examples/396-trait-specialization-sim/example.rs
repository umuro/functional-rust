// Simulating specialization in Rust
// Real specialization is unstable; we simulate with wrapper types + traits

// The "default" behavior via a wrapper
struct Default<T>(T);
struct Specialized<T>(T);

trait Serialize {
    fn serialize(&self) -> String;
}

// Blanket impl for Default wrapper (generic fallback)
impl<T: std::fmt::Debug> Serialize for Default<T> {
    fn serialize(&self) -> String {
        format!("{{"debug": "{:?}"}}", self.0)
    }
}

// "Specialized" impl for numbers
impl Serialize for Specialized<i32> {
    fn serialize(&self) -> String {
        self.0.to_string()  // Just the number, no quotes
    }
}

impl Serialize for Specialized<f64> {
    fn serialize(&self) -> String {
        format!("{:.6}", self.0)
    }
}

impl Serialize for Specialized<bool> {
    fn serialize(&self) -> String {
        if self.0 { "true".to_string() } else { "false".to_string() }
    }
}

impl Serialize for Specialized<String> {
    fn serialize(&self) -> String {
        format!(""{}"", self.0.replace('"', "\""))
    }
}

// Macro to select specialized vs default
macro_rules! serialize_val {
    ($val:expr, specialized) => { Specialized($val).serialize() };
    ($val:expr, default) => { Default($val).serialize() };
    ($val:expr) => { Default($val).serialize() };
}

fn main() {
    println!("i32 (specialized): {}", Specialized(42i32).serialize());
    println!("f64 (specialized): {}", Specialized(3.14f64).serialize());
    println!("bool (specialized): {}", Specialized(true).serialize());
    println!("String (specialized): {}", Specialized("hello".to_string()).serialize());

    #[derive(Debug)] struct Point { x: i32, y: i32 }
    println!("Point (default): {}", Default(Point { x: 1, y: 2 }).serialize());

    println!("\nVia macro:");
    println!("{}", serialize_val!(99i32, specialized));
    println!("{}", serialize_val!(vec![1,2,3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specialized_int() {
        assert_eq!(Specialized(7i32).serialize(), "7");
    }

    #[test]
    fn test_specialized_bool() {
        assert_eq!(Specialized(false).serialize(), "false");
    }

    #[test]
    fn test_default_fallback() {
        #[derive(Debug)] struct Foo(i32);
        let s = Default(Foo(1)).serialize();
        assert!(s.contains("debug"));
    }
}
