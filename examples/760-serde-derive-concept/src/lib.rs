//! # Serde Derive Concept
//!
//! Understanding how derive macros generate serialization code.

/// Simulated derive output for a struct
/// 
/// Given:
/// ```ignore
/// #[derive(Serialize)]
/// struct Point { x: i32, y: i32 }
/// ```
/// 
/// The derive macro generates something like this implementation.

// Manual implementation showing what #[derive(Serialize)] would generate

/// A simple output trait (simulating serde's Serializer)
pub trait Serializer {
    fn serialize_i32(&mut self, v: i32);
    fn serialize_str(&mut self, v: &str);
    fn serialize_struct_start(&mut self, name: &str, len: usize);
    fn serialize_field(&mut self, name: &str);
    fn serialize_struct_end(&mut self);
}

/// Our serialize trait
pub trait Serialize {
    fn serialize<S: Serializer>(&self, serializer: &mut S);
}

// Primitive implementations
impl Serialize for i32 {
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.serialize_i32(*self);
    }
}

impl Serialize for String {
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.serialize_str(self);
    }
}

impl Serialize for &str {
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.serialize_str(self);
    }
}

/// Example struct
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// What #[derive(Serialize)] would generate for Point
impl Serialize for Point {
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.serialize_struct_start("Point", 2);
        serializer.serialize_field("x");
        self.x.serialize(serializer);
        serializer.serialize_field("y");
        self.y.serialize(serializer);
        serializer.serialize_struct_end();
    }
}

/// Another example struct
#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: i32,
}

impl Serialize for Person {
    fn serialize<S: Serializer>(&self, serializer: &mut S) {
        serializer.serialize_struct_start("Person", 2);
        serializer.serialize_field("name");
        self.name.serialize(serializer);
        serializer.serialize_field("age");
        self.age.serialize(serializer);
        serializer.serialize_struct_end();
    }
}

// A JSON-like serializer for testing
pub struct JsonSerializer {
    output: String,
    first_field: bool,
}

impl JsonSerializer {
    pub fn new() -> Self {
        JsonSerializer {
            output: String::new(),
            first_field: true,
        }
    }

    pub fn into_string(self) -> String {
        self.output
    }
}

impl Default for JsonSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializer for JsonSerializer {
    fn serialize_i32(&mut self, v: i32) {
        self.output.push_str(&v.to_string());
    }

    fn serialize_str(&mut self, v: &str) {
        self.output.push('"');
        self.output.push_str(v);
        self.output.push('"');
    }

    fn serialize_struct_start(&mut self, _name: &str, _len: usize) {
        self.output.push('{');
        self.first_field = true;
    }

    fn serialize_field(&mut self, name: &str) {
        if !self.first_field {
            self.output.push_str(", ");
        }
        self.first_field = false;
        self.output.push('"');
        self.output.push_str(name);
        self.output.push_str("\": ");
    }

    fn serialize_struct_end(&mut self) {
        self.output.push('}');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_point() {
        let point = Point { x: 10, y: 20 };
        let mut ser = JsonSerializer::new();
        point.serialize(&mut ser);
        assert_eq!(ser.into_string(), r#"{"x": 10, "y": 20}"#);
    }

    #[test]
    fn test_serialize_person() {
        let person = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        let mut ser = JsonSerializer::new();
        person.serialize(&mut ser);
        assert_eq!(ser.into_string(), r#"{"name": "Alice", "age": 30}"#);
    }

    #[test]
    fn test_serialize_i32() {
        let mut ser = JsonSerializer::new();
        42i32.serialize(&mut ser);
        assert_eq!(ser.into_string(), "42");
    }

    #[test]
    fn test_serialize_string() {
        let mut ser = JsonSerializer::new();
        "hello".serialize(&mut ser);
        assert_eq!(ser.into_string(), "\"hello\"");
    }
}
