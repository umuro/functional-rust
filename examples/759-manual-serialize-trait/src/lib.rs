//! # Manual Serialize Trait
//!
//! Building a serialization trait from scratch.

/// A simple output buffer for serialization
pub struct Output {
    buffer: Vec<u8>,
}

impl Output {
    pub fn new() -> Self {
        Output { buffer: Vec::new() }
    }

    pub fn write_byte(&mut self, b: u8) {
        self.buffer.push(b);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    pub fn write_u32(&mut self, n: u32) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }

    pub fn write_string(&mut self, s: &str) {
        self.write_u32(s.len() as u32);
        self.write_bytes(s.as_bytes());
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

/// The serialize trait
pub trait Serialize {
    fn serialize(&self, out: &mut Output);

    fn to_bytes(&self) -> Vec<u8> {
        let mut out = Output::new();
        self.serialize(&mut out);
        out.into_bytes()
    }
}

// Implement for primitive types

impl Serialize for u8 {
    fn serialize(&self, out: &mut Output) {
        out.write_byte(*self);
    }
}

impl Serialize for u32 {
    fn serialize(&self, out: &mut Output) {
        out.write_u32(*self);
    }
}

impl Serialize for i32 {
    fn serialize(&self, out: &mut Output) {
        out.write_u32(*self as u32);
    }
}

impl Serialize for bool {
    fn serialize(&self, out: &mut Output) {
        out.write_byte(if *self { 1 } else { 0 });
    }
}

impl Serialize for String {
    fn serialize(&self, out: &mut Output) {
        out.write_string(self);
    }
}

impl Serialize for &str {
    fn serialize(&self, out: &mut Output) {
        out.write_string(self);
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, out: &mut Output) {
        out.write_u32(self.len() as u32);
        for item in self {
            item.serialize(out);
        }
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, out: &mut Output) {
        match self {
            Some(v) => {
                out.write_byte(1);
                v.serialize(out);
            }
            None => {
                out.write_byte(0);
            }
        }
    }
}

/// Example: A user-defined struct
#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub active: bool,
}

impl Serialize for Person {
    fn serialize(&self, out: &mut Output) {
        self.name.serialize(out);
        self.age.serialize(out);
        self.active.serialize(out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_u8() {
        let bytes = 42u8.to_bytes();
        assert_eq!(bytes, vec![42]);
    }

    #[test]
    fn test_serialize_u32() {
        let bytes = 0x12345678u32.to_bytes();
        assert_eq!(bytes, vec![0x78, 0x56, 0x34, 0x12]); // little-endian
    }

    #[test]
    fn test_serialize_bool() {
        assert_eq!(true.to_bytes(), vec![1]);
        assert_eq!(false.to_bytes(), vec![0]);
    }

    #[test]
    fn test_serialize_string() {
        let bytes = "hi".to_bytes();
        // length (4 bytes) + content
        assert_eq!(bytes, vec![2, 0, 0, 0, b'h', b'i']);
    }

    #[test]
    fn test_serialize_vec() {
        let v: Vec<u8> = vec![1, 2, 3];
        let bytes = v.to_bytes();
        assert_eq!(bytes, vec![3, 0, 0, 0, 1, 2, 3]);
    }

    #[test]
    fn test_serialize_option_some() {
        let opt: Option<u8> = Some(42);
        let bytes = opt.to_bytes();
        assert_eq!(bytes, vec![1, 42]);
    }

    #[test]
    fn test_serialize_option_none() {
        let opt: Option<u8> = None;
        let bytes = opt.to_bytes();
        assert_eq!(bytes, vec![0]);
    }

    #[test]
    fn test_serialize_person() {
        let person = Person {
            name: "Al".to_string(),
            age: 30,
            active: true,
        };
        let bytes = person.to_bytes();
        // name: 4 (len) + 2 (chars) = 6 bytes
        // age: 4 bytes
        // active: 1 byte
        assert_eq!(bytes.len(), 6 + 4 + 1);
    }
}
