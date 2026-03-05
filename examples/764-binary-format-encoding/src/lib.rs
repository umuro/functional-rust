//! # Binary Format Encoding
//!
//! Custom binary serialization format.

/// Binary encoder
pub struct BinaryEncoder {
    buffer: Vec<u8>,
}

impl BinaryEncoder {
    pub fn new() -> Self {
        BinaryEncoder { buffer: Vec::new() }
    }

    pub fn write_u8(&mut self, v: u8) {
        self.buffer.push(v);
    }

    pub fn write_u16(&mut self, v: u16) {
        self.buffer.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u32(&mut self, v: u32) {
        self.buffer.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u64(&mut self, v: u64) {
        self.buffer.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_i32(&mut self, v: i32) {
        self.buffer.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_f64(&mut self, v: f64) {
        self.buffer.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_bool(&mut self, v: bool) {
        self.write_u8(if v { 1 } else { 0 });
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.write_u32(bytes.len() as u32);
        self.buffer.extend_from_slice(bytes);
    }

    pub fn write_string(&mut self, s: &str) {
        self.write_bytes(s.as_bytes());
    }

    /// Variable-length integer encoding (like protobuf varint)
    pub fn write_varint(&mut self, mut v: u64) {
        while v >= 0x80 {
            self.write_u8((v as u8) | 0x80);
            v >>= 7;
        }
        self.write_u8(v as u8);
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl Default for BinaryEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Binary decoder
pub struct BinaryDecoder<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> BinaryDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BinaryDecoder { data, pos: 0 }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        if self.pos < self.data.len() {
            let v = self.data[self.pos];
            self.pos += 1;
            Some(v)
        } else {
            None
        }
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        if self.pos + 2 <= self.data.len() {
            let bytes = &self.data[self.pos..self.pos + 2];
            self.pos += 2;
            Some(u16::from_le_bytes([bytes[0], bytes[1]]))
        } else {
            None
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        if self.pos + 4 <= self.data.len() {
            let bytes = &self.data[self.pos..self.pos + 4];
            self.pos += 4;
            Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        } else {
            None
        }
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        if self.pos + 8 <= self.data.len() {
            let bytes = &self.data[self.pos..self.pos + 8];
            self.pos += 8;
            Some(u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]))
        } else {
            None
        }
    }

    pub fn read_bool(&mut self) -> Option<bool> {
        self.read_u8().map(|v| v != 0)
    }

    pub fn read_bytes(&mut self) -> Option<Vec<u8>> {
        let len = self.read_u32()? as usize;
        if self.pos + len <= self.data.len() {
            let bytes = self.data[self.pos..self.pos + len].to_vec();
            self.pos += len;
            Some(bytes)
        } else {
            None
        }
    }

    pub fn read_string(&mut self) -> Option<String> {
        let bytes = self.read_bytes()?;
        String::from_utf8(bytes).ok()
    }

    pub fn read_varint(&mut self) -> Option<u64> {
        let mut result: u64 = 0;
        let mut shift = 0;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as u64) << shift;
            if byte & 0x80 == 0 {
                return Some(result);
            }
            shift += 7;
            if shift >= 64 {
                return None;
            }
        }
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_roundtrip() {
        let mut enc = BinaryEncoder::new();
        enc.write_u8(42);
        enc.write_u8(255);

        let bytes = enc.into_bytes();
        let mut dec = BinaryDecoder::new(&bytes);
        assert_eq!(dec.read_u8(), Some(42));
        assert_eq!(dec.read_u8(), Some(255));
    }

    #[test]
    fn test_u32_roundtrip() {
        let mut enc = BinaryEncoder::new();
        enc.write_u32(0x12345678);

        let bytes = enc.into_bytes();
        let mut dec = BinaryDecoder::new(&bytes);
        assert_eq!(dec.read_u32(), Some(0x12345678));
    }

    #[test]
    fn test_string_roundtrip() {
        let mut enc = BinaryEncoder::new();
        enc.write_string("hello, world!");

        let bytes = enc.into_bytes();
        let mut dec = BinaryDecoder::new(&bytes);
        assert_eq!(dec.read_string(), Some("hello, world!".to_string()));
    }

    #[test]
    fn test_bool_roundtrip() {
        let mut enc = BinaryEncoder::new();
        enc.write_bool(true);
        enc.write_bool(false);

        let bytes = enc.into_bytes();
        let mut dec = BinaryDecoder::new(&bytes);
        assert_eq!(dec.read_bool(), Some(true));
        assert_eq!(dec.read_bool(), Some(false));
    }

    #[test]
    fn test_varint_small() {
        let mut enc = BinaryEncoder::new();
        enc.write_varint(127);

        let bytes = enc.into_bytes();
        assert_eq!(bytes.len(), 1);

        let mut dec = BinaryDecoder::new(&bytes);
        assert_eq!(dec.read_varint(), Some(127));
    }

    #[test]
    fn test_varint_large() {
        let mut enc = BinaryEncoder::new();
        enc.write_varint(300);

        let bytes = enc.into_bytes();
        assert_eq!(bytes.len(), 2);

        let mut dec = BinaryDecoder::new(&bytes);
        assert_eq!(dec.read_varint(), Some(300));
    }

    #[test]
    fn test_mixed_types() {
        let mut enc = BinaryEncoder::new();
        enc.write_u8(1);
        enc.write_u32(1000);
        enc.write_string("test");
        enc.write_bool(true);

        let bytes = enc.into_bytes();
        let mut dec = BinaryDecoder::new(&bytes);

        assert_eq!(dec.read_u8(), Some(1));
        assert_eq!(dec.read_u32(), Some(1000));
        assert_eq!(dec.read_string(), Some("test".to_string()));
        assert_eq!(dec.read_bool(), Some(true));
        assert_eq!(dec.remaining(), 0);
    }
}
