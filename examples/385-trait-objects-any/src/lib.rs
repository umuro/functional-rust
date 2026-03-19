#![allow(clippy::all)]
//! Any Trait for Runtime Type Information

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Describe a value using downcasting
pub fn describe(val: &dyn Any) -> String {
    if let Some(n) = val.downcast_ref::<i32>() {
        format!("i32: {}", n)
    } else if let Some(s) = val.downcast_ref::<String>() {
        format!("String: {}", s)
    } else if let Some(b) = val.downcast_ref::<bool>() {
        format!("bool: {}", b)
    } else {
        format!("Unknown type: {:?}", val.type_id())
    }
}

/// Type-safe heterogeneous map
pub struct TypeMap {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn insert<T: Any>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }
}

impl Default for TypeMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_i32() {
        assert_eq!(describe(&42i32), "i32: 42");
    }
    #[test]
    fn test_describe_string() {
        assert_eq!(describe(&String::from("hi")), "String: hi");
    }
    #[test]
    fn test_describe_bool() {
        assert_eq!(describe(&true), "bool: true");
    }
    #[test]
    fn test_typemap() {
        let mut m = TypeMap::new();
        m.insert(42i32);
        m.insert("hello".to_string());
        assert_eq!(m.get::<i32>(), Some(&42));
        assert_eq!(m.get::<String>(), Some(&"hello".to_string()));
    }
}
