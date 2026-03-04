# 764: Binary Serialization: Length-Prefixed Records

**Difficulty:** 3  **Level:** Intermediate

Encode structured data as compact bytes — length-prefixed strings, big-endian integers, booleans — and decode them back exactly, with safe error handling for truncated input.

## The Problem This Solves

Text formats like JSON and CSV are convenient but expensive. Parsing a JSON float requires scanning character by character and calling a string-to-float converter. A binary `f64` is 8 bytes read directly into memory — no parsing, no allocation. For high-throughput systems (network protocols, file formats, message queues), binary serialization is often 10-100x faster and produces significantly smaller payloads.

Binary formats also appear in protocol implementations: network packets, database file formats, binary message queues, firmware communication. When you're talking to hardware, you're always dealing with binary. When you're writing a file format, binary gives you control over exact byte layout and compatibility across architectures.

The central challenge in binary serialization is *length prefixes*. Unlike JSON where a string ends at `"`, binary strings need their length encoded first. "Alice" becomes `[0, 0, 0, 5, 65, 108, 105, 99, 101]` — 4 bytes of length, then 5 bytes of UTF-8. The reader knows exactly how many bytes to consume.

## The Intuition

Think of TCP/IP framing: each packet has a header that says how long the payload is. Length-prefixed records work the same way. Writer prepends length; reader reads length first, then reads exactly that many bytes.

Big-endian byte order (`to_be_bytes`, `from_be_bytes`) is the network standard. It means the most significant byte comes first — `256u32` is `[0, 0, 1, 0]`. All systems agree on this order, unlike native endian which varies by CPU architecture.

The `BinaryWriter` / `BinaryReader` pair is a classic I/O abstraction: one type appends bytes; the other tracks a cursor into a `&[u8]` slice. The cursor advances as you read — if you ever try to read past the end, you get `DecodeError::UnexpectedEof` rather than a panic or silent wrong data.

## How It Works in Rust

```rust
// Writer — accumulates bytes into a Vec
pub struct BinaryWriter(Vec<u8>);

impl BinaryWriter {
    pub fn write_u32_be(&mut self, v: u32) {
        self.0.extend_from_slice(&v.to_be_bytes()); // 4 bytes, big-endian
    }

    pub fn write_string(&mut self, s: &str) {
        self.write_u32_be(s.len() as u32);      // length prefix
        self.0.extend_from_slice(s.as_bytes()); // then the content
    }

    pub fn finish(self) -> Vec<u8> { self.0 }
}

// Reader — cursor into a byte slice
pub struct BinaryReader<'a> { data: &'a [u8], pos: usize }

impl<'a> BinaryReader<'a> {
    fn consume(&mut self, n: usize) -> Result<&'a [u8], DecodeError> {
        if self.pos + n > self.data.len() {
            return Err(DecodeError::UnexpectedEof);  // safe: no panic
        }
        let slice = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(slice)
    }

    pub fn read_u32_be(&mut self) -> Result<u32, DecodeError> {
        let b = self.consume(4)?;
        Ok(u32::from_be_bytes(b.try_into().unwrap()))  // slice → [u8;4] → u32
    }

    pub fn read_string(&mut self) -> Result<&'a str, DecodeError> {
        let len = self.read_u32_be()? as usize;        // read the length
        let bytes = self.consume(len)?;                 // read exactly that many bytes
        std::str::from_utf8(bytes).map_err(|_| DecodeError::InvalidUtf8)
    }
}

// Encode/decode a domain type
impl Person {
    pub fn encode(&self) -> Vec<u8> {
        let mut w = BinaryWriter::new();
        w.write_string(&self.name); // [4 bytes len][name bytes]
        w.write_u32_be(self.age);   // [4 bytes]
        w.write_bool(self.active);  // [1 byte: 0 or 1]
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

// Multi-record stream: length-prefix each record for framing
let mut buf = Vec::new();
for person in &people {
    let encoded = person.encode();
    buf.extend_from_slice(&(encoded.len() as u32).to_be_bytes()); // frame length
    buf.extend_from_slice(&encoded);
}
```

Key points:
- `to_be_bytes()` / `from_be_bytes()` — zero-copy conversion between integers and byte arrays
- `try_into().unwrap()` converts `&[u8]` to `[u8; 4]` — safe when you just consumed exactly 4 bytes
- UTF-8 string round-trips safely: `s.as_bytes()` → `std::str::from_utf8()` — handles Ümür, 中文, emoji
- A `DecodeError` is required because any read can fail on truncated data — never use indexing `data[pos]` directly
- For multiple records in one buffer, wrap each encoded record in another length prefix (framing)

## What This Unlocks

- **Network protocols**: implement a custom binary protocol, or understand how existing ones (Redis RESP, MessagePack, protobuf) work
- **High-performance file formats**: store millions of records per second without JSON parsing overhead
- **Deterministic wire format**: same bytes on any machine, any language — binary with explicit endian is universally interoperable

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Binary I/O | `Bytes.create`, `Bytes.set_int32_be` | `Vec<u8>` + `u32::to_be_bytes()` |
| Reading | `Buffer.contents`, `Bytes.get_int32_be` | `BinaryReader` cursor struct |
| Endianness | `Bytes.set_int32_be` / `Bytes.get_int32_be` | `.to_be_bytes()` / `from_be_bytes()` |
| Truncated input | Exception | `Result<_, DecodeError::UnexpectedEof>` |
| Length-prefixed string | Manual | `write_u32_be(len); write bytes; read_u32_be(); consume(len)` |
| Production library | `bin_prot`, `marshal` | `bincode`, `prost` (protobuf), `rkyv` |
