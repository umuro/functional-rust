// 764. Binary Serialization: Length-Prefixed Records
// TLV-style binary format, std-only

// ── Encoder ────────────────────────────────────────────────────────────────────

pub struct BinaryWriter(Vec<u8>);

impl BinaryWriter {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn write_u8(&mut self, v: u8) {
        self.0.push(v);
    }

    pub fn write_u32_be(&mut self, v: u32) {
        self.0.extend_from_slice(&v.to_be_bytes());
    }

    pub fn write_u64_be(&mut self, v: u64) {
        self.0.extend_from_slice(&v.to_be_bytes());
    }

    pub fn write_bool(&mut self, v: bool) {
        self.write_u8(if v { 1 } else { 0 });
    }

    /// Length-prefixed string: u32 length then UTF-8 bytes
    pub fn write_string(&mut self, s: &str) {
        self.write_u32_be(s.len() as u32);
        self.0.extend_from_slice(s.as_bytes());
    }

    pub fn finish(self) -> Vec<u8> { self.0 }
}

// ── Decoder ────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum DecodeError {
    UnexpectedEof,
    InvalidUtf8,
}

pub struct BinaryReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> BinaryReader<'a> {
    pub fn new(data: &'a [u8]) -> Self { Self { data, pos: 0 } }

    fn consume(&mut self, n: usize) -> Result<&'a [u8], DecodeError> {
        if self.pos + n > self.data.len() { return Err(DecodeError::UnexpectedEof); }
        let slice = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(slice)
    }

    pub fn read_u8(&mut self) -> Result<u8, DecodeError> {
        Ok(self.consume(1)?[0])
    }

    pub fn read_u32_be(&mut self) -> Result<u32, DecodeError> {
        let b = self.consume(4)?;
        Ok(u32::from_be_bytes(b.try_into().unwrap()))
    }

    pub fn read_u64_be(&mut self) -> Result<u64, DecodeError> {
        let b = self.consume(8)?;
        Ok(u64::from_be_bytes(b.try_into().unwrap()))
    }

    pub fn read_bool(&mut self) -> Result<bool, DecodeError> {
        Ok(self.read_u8()? != 0)
    }

    pub fn read_string(&mut self) -> Result<&'a str, DecodeError> {
        let len = self.read_u32_be()? as usize;
        let bytes = self.consume(len)?;
        std::str::from_utf8(bytes).map_err(|_| DecodeError::InvalidUtf8)
    }
}

// ── Domain type ────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub active: bool,
}

impl Person {
    pub fn encode(&self) -> Vec<u8> {
        let mut w = BinaryWriter::new();
        w.write_string(&self.name);
        w.write_u32_be(self.age);
        w.write_bool(self.active);
        w.finish()
    }

    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        let mut r = BinaryReader::new(data);
        let name   = r.read_string()?.to_string();
        let age    = r.read_u32_be()?;
        let active = r.read_bool()?;
        Ok(Person { name, age, active })
    }
}

fn hex_dump(data: &[u8]) -> String {
    data.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")
}

fn main() {
    let alice = Person { name: "Alice".into(), age: 30, active: true };
    let encoded = alice.encode();
    println!("Encoded ({} bytes): {}", encoded.len(), hex_dump(&encoded));

    let decoded = Person::decode(&encoded).expect("decode failed");
    println!("Decoded: {decoded:?}");

    // Multiple records in one buffer
    let records = vec![
        Person { name: "Bob".into(),   age: 25, active: false },
        Person { name: "Carol".into(), age: 35, active: true  },
    ];
    let mut buf = Vec::new();
    for r in &records {
        let enc = r.encode();
        buf.extend_from_slice(&(enc.len() as u32).to_be_bytes());
        buf.extend_from_slice(&enc);
    }
    println!("\nMulti-record buffer ({} bytes): {}", buf.len(), hex_dump(&buf));

    // Decode multi-record
    let mut pos = 0;
    while pos + 4 <= buf.len() {
        let len = u32::from_be_bytes(buf[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        let p = Person::decode(&buf[pos..pos+len]).unwrap();
        println!("  Record: {p:?}");
        pos += len;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let p = Person { name: "Dave".into(), age: 40, active: true };
        assert_eq!(p, Person::decode(&p.encode()).unwrap());
    }

    #[test]
    fn utf8_name() {
        let p = Person { name: "Ümür".into(), age: 33, active: false };
        assert_eq!(p, Person::decode(&p.encode()).unwrap());
    }

    #[test]
    fn eof_error() {
        assert!(matches!(Person::decode(&[]), Err(DecodeError::UnexpectedEof)));
    }

    #[test]
    fn length_prefix_correct() {
        let p = Person { name: "Ed".into(), age: 1, active: false };
        let enc = p.encode();
        // first 4 bytes = 2 (length of "Ed")
        assert_eq!(&enc[..4], &[0, 0, 0, 2]);
        // next 2 bytes = "Ed"
        assert_eq!(&enc[4..6], b"Ed");
    }
}
