//! # Type Ascription in Patterns
//!
//! Type annotations and runtime type dispatch in Rust.

use std::any::Any;

/// A dynamic value type that can hold different types.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

impl Value {
    /// Get the type name of the value.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Str(_) => "str",
            Value::Bool(_) => "bool",
        }
    }

    /// Convert to f64 if possible.
    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Value::Int(n) => Some(*n as f64),
            Value::Float(f) => Some(*f),
            Value::Str(s) => s.parse().ok(),
            Value::Bool(_) => None,
        }
    }

    /// Convert to i64 if possible.
    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Value::Int(n) => Some(*n),
            Value::Float(f) => Some(*f as i64),
            Value::Str(s) => s.parse().ok(),
            Value::Bool(b) => Some(if *b { 1 } else { 0 }),
        }
    }

    /// Check if this is a numeric type.
    pub fn is_numeric(&self) -> bool {
        matches!(self, Value::Int(_) | Value::Float(_))
    }

    /// Try to add two Values.
    pub fn add(&self, other: &Value) -> Option<Value> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Some(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Some(Value::Float(a + *b as f64)),
            (Value::Str(a), Value::Str(b)) => Some(Value::Str(format!("{}{}", a, b))),
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(v) => write!(f, "{}", v),
            Value::Str(s) => write!(f, "{:?}", s),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

/// Describe the type of an Any value using downcast.
pub fn describe_any(v: &dyn Any) -> &'static str {
    if v.downcast_ref::<i32>().is_some() {
        "i32"
    } else if v.downcast_ref::<i64>().is_some() {
        "i64"
    } else if v.downcast_ref::<f64>().is_some() {
        "f64"
    } else if v.downcast_ref::<String>().is_some() {
        "String"
    } else if v.downcast_ref::<&str>().is_some() {
        "&str"
    } else if v.downcast_ref::<bool>().is_some() {
        "bool"
    } else {
        "unknown"
    }
}

/// Extract a specific type from Any.
pub fn extract_i32(v: &dyn Any) -> Option<i32> {
    v.downcast_ref::<i32>().copied()
}

/// Extract string from Any.
pub fn extract_string(v: &dyn Any) -> Option<String> {
    v.downcast_ref::<String>()
        .cloned()
        .or_else(|| v.downcast_ref::<&str>().map(|s| s.to_string()))
}

/// Demonstrate numeric type casting.
pub fn cast_demo(x: i32) -> (u8, i64, f64) {
    (x as u8, x as i64, x as f64)
}

/// Safe numeric conversion using TryFrom.
pub fn safe_cast_to_u8(x: i32) -> Option<u8> {
    u8::try_from(x).ok()
}

/// Type-safe extraction from tuple.
pub fn extract_typed<T: Clone + 'static>(values: &[Box<dyn Any>]) -> Vec<T> {
    values
        .iter()
        .filter_map(|v| v.downcast_ref::<T>().cloned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_type_name() {
        assert_eq!(Value::Int(42).type_name(), "int");
        assert_eq!(Value::Float(3.14).type_name(), "float");
        assert_eq!(Value::Str("hi".into()).type_name(), "str");
        assert_eq!(Value::Bool(true).type_name(), "bool");
    }

    #[test]
    fn test_value_to_f64() {
        assert_eq!(Value::Int(42).to_f64(), Some(42.0));
        assert_eq!(Value::Float(3.14).to_f64(), Some(3.14));
        assert_eq!(Value::Str("2.5".into()).to_f64(), Some(2.5));
        assert_eq!(Value::Bool(true).to_f64(), None);
    }

    #[test]
    fn test_value_to_i64() {
        assert_eq!(Value::Int(42).to_i64(), Some(42));
        assert_eq!(Value::Float(3.9).to_i64(), Some(3));
        assert_eq!(Value::Str("100".into()).to_i64(), Some(100));
        assert_eq!(Value::Bool(true).to_i64(), Some(1));
        assert_eq!(Value::Bool(false).to_i64(), Some(0));
    }

    #[test]
    fn test_value_is_numeric() {
        assert!(Value::Int(1).is_numeric());
        assert!(Value::Float(1.0).is_numeric());
        assert!(!Value::Str("1".into()).is_numeric());
        assert!(!Value::Bool(true).is_numeric());
    }

    #[test]
    fn test_value_add() {
        assert_eq!(Value::Int(1).add(&Value::Int(2)), Some(Value::Int(3)));
        assert_eq!(
            Value::Float(1.5).add(&Value::Float(2.5)),
            Some(Value::Float(4.0))
        );
        assert_eq!(
            Value::Str("a".into()).add(&Value::Str("b".into())),
            Some(Value::Str("ab".into()))
        );
        assert_eq!(Value::Bool(true).add(&Value::Int(1)), None);
    }

    #[test]
    fn test_value_display() {
        assert_eq!(format!("{}", Value::Int(42)), "42");
        assert_eq!(format!("{}", Value::Bool(true)), "true");
    }

    #[test]
    fn test_describe_any() {
        let i: Box<dyn Any> = Box::new(42i32);
        let f: Box<dyn Any> = Box::new(3.14f64);
        let s: Box<dyn Any> = Box::new(String::from("hello"));
        let b: Box<dyn Any> = Box::new(true);

        assert_eq!(describe_any(i.as_ref()), "i32");
        assert_eq!(describe_any(f.as_ref()), "f64");
        assert_eq!(describe_any(s.as_ref()), "String");
        assert_eq!(describe_any(b.as_ref()), "bool");
    }

    #[test]
    fn test_extract_i32() {
        let i: Box<dyn Any> = Box::new(42i32);
        let f: Box<dyn Any> = Box::new(3.14f64);

        assert_eq!(extract_i32(i.as_ref()), Some(42));
        assert_eq!(extract_i32(f.as_ref()), None);
    }

    #[test]
    fn test_extract_string() {
        let s: Box<dyn Any> = Box::new(String::from("hello"));
        let sr: Box<dyn Any> = Box::new("world");
        let i: Box<dyn Any> = Box::new(42i32);

        assert_eq!(extract_string(s.as_ref()), Some("hello".to_string()));
        assert_eq!(extract_string(sr.as_ref()), Some("world".to_string()));
        assert_eq!(extract_string(i.as_ref()), None);
    }

    #[test]
    fn test_cast_demo() {
        assert_eq!(cast_demo(300), (44, 300, 300.0)); // 300 as u8 wraps to 44
        assert_eq!(cast_demo(42), (42, 42, 42.0));
    }

    #[test]
    fn test_safe_cast() {
        assert_eq!(safe_cast_to_u8(42), Some(42));
        assert_eq!(safe_cast_to_u8(300), None);
        assert_eq!(safe_cast_to_u8(-1), None);
    }

    #[test]
    fn test_extract_typed() {
        let values: Vec<Box<dyn Any>> = vec![
            Box::new(1i32),
            Box::new("hello"),
            Box::new(2i32),
            Box::new(3.14f64),
            Box::new(3i32),
        ];
        let ints: Vec<i32> = extract_typed(&values);
        assert_eq!(ints, vec![1, 2, 3]);
    }
}
