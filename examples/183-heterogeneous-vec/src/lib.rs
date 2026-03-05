// Example 183: Heterogeneous Vector with Safe Downcast
// Store different types in one Vec, downcast safely via Any

use std::any::Any;
use std::fmt;

// === Approach 1: Box<dyn Any> with downcast ===

struct HeteroVec {
    items: Vec<Box<dyn Any>>,
}

impl HeteroVec {
    fn new() -> Self { HeteroVec { items: Vec::new() } }

    fn push<T: 'static>(&mut self, val: T) {
        self.items.push(Box::new(val));
    }

    fn get<T: 'static>(&self, index: usize) -> Option<&T> {
        self.items.get(index)?.downcast_ref::<T>()
    }

    fn len(&self) -> usize { self.items.len() }
}

// === Approach 2: Custom trait object with Display + Any ===

trait AnyDisplay: Any + fmt::Display {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any + fmt::Display> AnyDisplay for T {
    fn as_any(&self) -> &dyn Any { self }
}

struct DisplayVec {
    items: Vec<Box<dyn AnyDisplay>>,
}

impl DisplayVec {
    fn new() -> Self { DisplayVec { items: Vec::new() } }

    fn push<T: Any + fmt::Display>(&mut self, val: T) {
        self.items.push(Box::new(val));
    }

    fn get<T: 'static>(&self, index: usize) -> Option<&T> {
        self.items.get(index)?.as_any().downcast_ref::<T>()
    }

    fn display_all(&self) -> Vec<String> {
        self.items.iter().map(|x| format!("{}", x)).collect()
    }
}

// === Approach 3: Enum-based (like OCaml value type) ===

#[derive(Debug, Clone)]
enum Value {
    Int(i64),
    Str(String),
    Bool(bool),
    Float(f64),
}

impl Value {
    fn as_int(&self) -> Option<i64> {
        match self { Value::Int(n) => Some(*n), _ => None }
    }
    fn as_str(&self) -> Option<&str> {
        match self { Value::Str(s) => Some(s), _ => None }
    }
    fn as_bool(&self) -> Option<bool> {
        match self { Value::Bool(b) => Some(*b), _ => None }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Float(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hetero_vec() {
        let mut hv = HeteroVec::new();
        hv.push(42i64);
        hv.push(String::from("hello"));
        hv.push(true);
        assert_eq!(hv.get::<i64>(0), Some(&42));
        assert_eq!(hv.get::<i64>(1), None);
        assert_eq!(hv.get::<String>(1), Some(&String::from("hello")));
        assert_eq!(hv.get::<bool>(2), Some(&true));
        assert_eq!(hv.len(), 3);
    }

    #[test]
    fn test_display_vec() {
        let mut dv = DisplayVec::new();
        dv.push(42i64);
        dv.push(String::from("hi"));
        assert_eq!(dv.display_all(), vec!["42", "hi"]);
        assert_eq!(dv.get::<i64>(0), Some(&42));
    }

    #[test]
    fn test_value_enum() {
        assert_eq!(Value::Int(42).as_int(), Some(42));
        assert_eq!(Value::Int(42).as_str(), None);
        assert_eq!(Value::Str("x".into()).as_str(), Some("x"));
        assert_eq!(Value::Bool(true).as_bool(), Some(true));
    }

    #[test]
    fn test_value_display() {
        assert_eq!(format!("{}", Value::Int(42)), "42");
        assert_eq!(format!("{}", Value::Str("hi".into())), "hi");
        assert_eq!(format!("{}", Value::Float(3.14)), "3.14");
    }
}
