//! # Custom Deserialize Logic
//!
//! Parsing and validation during deserialization.

/// Input buffer for deserialization
pub struct Input<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Input<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Input { data, pos: 0 }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if self.pos < self.data.len() {
            let b = self.data[self.pos];
            self.pos += 1;
            Some(b)
        } else {
            None
        }
    }

    pub fn read_bytes(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.pos + n <= self.data.len() {
            let slice = &self.data[self.pos..self.pos + n];
            self.pos += n;
            Some(slice)
        } else {
            None
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        let bytes = self.read_bytes(4)?;
        Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_string(&mut self) -> Option<String> {
        let len = self.read_u32()? as usize;
        let bytes = self.read_bytes(len)?;
        String::from_utf8(bytes.to_vec()).ok()
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }
}

/// Deserialize error types
#[derive(Debug, PartialEq)]
pub enum DeserializeError {
    UnexpectedEof,
    InvalidUtf8,
    InvalidValue(String),
    ValidationFailed(String),
}

/// The deserialize trait
pub trait Deserialize: Sized {
    fn deserialize(input: &mut Input) -> Result<Self, DeserializeError>;
}

impl Deserialize for u8 {
    fn deserialize(input: &mut Input) -> Result<Self, DeserializeError> {
        input.read_byte().ok_or(DeserializeError::UnexpectedEof)
    }
}

impl Deserialize for u32 {
    fn deserialize(input: &mut Input) -> Result<Self, DeserializeError> {
        input.read_u32().ok_or(DeserializeError::UnexpectedEof)
    }
}

impl Deserialize for bool {
    fn deserialize(input: &mut Input) -> Result<Self, DeserializeError> {
        match input.read_byte() {
            Some(0) => Ok(false),
            Some(1) => Ok(true),
            Some(v) => Err(DeserializeError::InvalidValue(format!(
                "bool must be 0 or 1, got {}",
                v
            ))),
            None => Err(DeserializeError::UnexpectedEof),
        }
    }
}

impl Deserialize for String {
    fn deserialize(input: &mut Input) -> Result<Self, DeserializeError> {
        input.read_string().ok_or(DeserializeError::InvalidUtf8)
    }
}

/// Email with validation
#[derive(Debug, PartialEq, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(s: &str) -> Result<Self, DeserializeError> {
        if s.contains('@') && s.len() >= 3 {
            Ok(Email(s.to_string()))
        } else {
            Err(DeserializeError::ValidationFailed(
                "invalid email format".to_string(),
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deserialize for Email {
    fn deserialize(input: &mut Input) -> Result<Self, DeserializeError> {
        let s = String::deserialize(input)?;
        Email::new(&s)
    }
}

/// Positive integer (validated)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PositiveInt(u32);

impl PositiveInt {
    pub fn new(n: u32) -> Result<Self, DeserializeError> {
        if n > 0 {
            Ok(PositiveInt(n))
        } else {
            Err(DeserializeError::ValidationFailed(
                "must be positive".to_string(),
            ))
        }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

impl Deserialize for PositiveInt {
    fn deserialize(input: &mut Input) -> Result<Self, DeserializeError> {
        let n = u32::deserialize(input)?;
        PositiveInt::new(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_u8() {
        let data = [42u8];
        let mut input = Input::new(&data);
        assert_eq!(u8::deserialize(&mut input), Ok(42));
    }

    #[test]
    fn test_deserialize_u32() {
        let data = [0x78, 0x56, 0x34, 0x12];
        let mut input = Input::new(&data);
        assert_eq!(u32::deserialize(&mut input), Ok(0x12345678));
    }

    #[test]
    fn test_deserialize_bool() {
        let data = [0, 1];
        let mut input = Input::new(&data);
        assert_eq!(bool::deserialize(&mut input), Ok(false));
        assert_eq!(bool::deserialize(&mut input), Ok(true));
    }

    #[test]
    fn test_deserialize_bool_invalid() {
        let data = [2];
        let mut input = Input::new(&data);
        assert!(matches!(
            bool::deserialize(&mut input),
            Err(DeserializeError::InvalidValue(_))
        ));
    }

    #[test]
    fn test_deserialize_string() {
        let data = [2, 0, 0, 0, b'h', b'i'];
        let mut input = Input::new(&data);
        assert_eq!(String::deserialize(&mut input), Ok("hi".to_string()));
    }

    #[test]
    fn test_email_valid() {
        let data = [11, 0, 0, 0, b't', b'e', b's', b't', b'@', b'm', b'a', b'i', b'l', b'.', b'x'];
        let mut input = Input::new(&data);
        let email = Email::deserialize(&mut input).unwrap();
        assert_eq!(email.as_str(), "test@mail.x");
    }

    #[test]
    fn test_email_invalid() {
        let data = [4, 0, 0, 0, b't', b'e', b's', b't'];
        let mut input = Input::new(&data);
        assert!(matches!(
            Email::deserialize(&mut input),
            Err(DeserializeError::ValidationFailed(_))
        ));
    }

    #[test]
    fn test_positive_int_valid() {
        let data = [5, 0, 0, 0];
        let mut input = Input::new(&data);
        let pi = PositiveInt::deserialize(&mut input).unwrap();
        assert_eq!(pi.get(), 5);
    }

    #[test]
    fn test_positive_int_zero() {
        let data = [0, 0, 0, 0];
        let mut input = Input::new(&data);
        assert!(matches!(
            PositiveInt::deserialize(&mut input),
            Err(DeserializeError::ValidationFailed(_))
        ));
    }
}
