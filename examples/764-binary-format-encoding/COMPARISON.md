# OCaml vs Rust: Binary Format Encoding

## Binary Encoder

### Rust
```rust
pub struct BinaryEncoder {
    buffer: Vec<u8>,
}

impl BinaryEncoder {
    pub fn write_u32(&mut self, v: u32) {
        self.buffer.extend_from_slice(&v.to_le_bytes());
    }
    
    pub fn write_string(&mut self, s: &str) {
        self.write_u32(s.len() as u32);
        self.buffer.extend_from_slice(s.as_bytes());
    }
}
```

### OCaml
```ocaml
let write_int32 buf v =
  Buffer.add_int32_le buf v

let write_string buf s =
  write_int32 buf (Int32.of_int (String.length s));
  Buffer.add_string buf s
```

## Variable-Length Integer (Varint)

### Rust
```rust
pub fn write_varint(&mut self, mut v: u64) {
    while v >= 0x80 {
        self.write_u8((v as u8) | 0x80);
        v >>= 7;
    }
    self.write_u8(v as u8);
}
```

### OCaml
```ocaml
let rec write_varint buf v =
  if v >= 0x80 then begin
    Buffer.add_char buf (Char.chr ((v land 0x7F) lor 0x80));
    write_varint buf (v lsr 7)
  end else
    Buffer.add_char buf (Char.chr v)
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Buffer type | `Buffer.t` | `Vec<u8>` |
| Byte conversion | `Char.chr` | Cast to `u8` |
| Endianness | `add_int32_le` | `to_le_bytes()` |
| Slice operations | `Bytes.sub` | `&[..n]` |
