// Any trait for runtime type info in Rust
use std::any::{Any, TypeId};
use std::collections::HashMap;

fn describe(val: &dyn Any) {
    if let Some(n) = val.downcast_ref::<i32>() {
        println!("i32: {}", n);
    } else if let Some(s) = val.downcast_ref::<String>() {
        println!("String: {}", s);
    } else if let Some(b) = val.downcast_ref::<bool>() {
        println!("bool: {}", b);
    } else if let Some(v) = val.downcast_ref::<Vec<i32>>() {
        println!("Vec<i32>: {:?}", v);
    } else {
        println!("Unknown type: {:?}", val.type_id());
    }
}

// Type-safe heterogeneous map using TypeId
struct TypeMap {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeMap {
    fn new() -> Self { TypeMap { map: HashMap::new() } }

    fn insert<T: Any>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }

    fn get<T: Any>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }
}

fn main() {
    let values: Vec<Box<dyn Any>> = vec![
        Box::new(42i32),
        Box::new(String::from("hello")),
        Box::new(true),
        Box::new(vec![1i32, 2, 3]),
    ];

    for v in &values {
        describe(v.as_ref());
    }

    println!();
    let mut map = TypeMap::new();
    map.insert(100i32);
    map.insert("world".to_string());
    map.insert(3.14f64);

    println!("i32 from TypeMap: {:?}", map.get::<i32>());
    println!("String from TypeMap: {:?}", map.get::<String>());
    println!("f64 from TypeMap: {:?}", map.get::<f64>());
    println!("bool from TypeMap: {:?}", map.get::<bool>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downcast() {
        let val: Box<dyn Any> = Box::new(42i32);
        assert_eq!(val.downcast_ref::<i32>(), Some(&42));
        assert!(val.downcast_ref::<String>().is_none());
    }

    #[test]
    fn test_type_map() {
        let mut map = TypeMap::new();
        map.insert(99u8);
        map.insert("test".to_string());
        assert_eq!(map.get::<u8>(), Some(&99));
        assert_eq!(map.get::<String>(), Some(&"test".to_string()));
        assert!(map.get::<i64>().is_none());
    }
}
