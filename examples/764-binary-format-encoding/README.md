📖 **[View on hightechmind.io →](https://hightechmind.io/rust/764-binary-format-encoding)**

---

# 764-binary-format-encoding — Binary Format Encoding
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Binary serialization is 2–10x smaller and faster than JSON for the same data. It is used in database wire protocols (PostgreSQL binary format), game networking (ENet, Quake protocol), inter-service communication (Protocol Buffers, FlatBuffers, MessagePack), and file formats (ELF, PNG, MP4). Understanding binary encoding teaches endianness, alignment, length prefixing, and the fundamental trade-offs between human readability and machine efficiency.

## Learning Outcomes

- Implement a `BinaryEncoder` with typed write methods using `to_le_bytes()` for portability
- Write length-prefixed byte arrays and strings (`u32` prefix + data)
- Implement a corresponding `BinaryDecoder` with checked read methods
- Handle endianness explicitly: always use little-endian for cross-platform compatibility
- Write roundtrip tests verifying `decode(encode(x)) == x` for all types

## Rust Application

`BinaryEncoder` wraps `Vec<u8>` and provides `write_u8`, `write_u16`, `write_u32`, `write_u64`, `write_i32`, `write_f64`, `write_bool`, `write_bytes` (length-prefixed), and `write_string`. `BinaryDecoder` reads from a `&[u8]` slice with an offset cursor, providing corresponding `read_*` methods. A `Record` struct with mixed types tests the full encoder/decoder cycle. Performance is compared against JSON for the same data.

## OCaml Approach

OCaml's `Bytes` module provides `set_int32_le`, `get_int64_le`, and similar methods for binary encoding. `Bin_prot` (Jane Street) is the production binary serialization library used in Jane Street's trading systems — it generates optimal readers/writers from type definitions. `Bigstringaf` provides zero-copy binary processing for high-throughput servers. The `Faraday` library provides efficient binary encoding with buffering.

## Key Differences

1. **Endianness**: Rust's `to_le_bytes()` makes endianness explicit; OCaml's `Bytes.set_int32_le` does the same; both prefer little-endian for cross-platform compatibility.
2. **Length prefixes**: Both use 4-byte length prefixes for variable-length data; the encoding is identical since it's a wire format convention.
3. **Zero-copy reading**: Rust's `&[u8]` + offset is a simple zero-copy reader; OCaml's `Bigstringaf` provides more sophisticated zero-copy I/O with GC integration.
4. **Type safety**: Rust's typed encode/decode methods prevent mixing up byte counts; OCaml's `Bin_prot` generates typed readers with the same guarantee.

## Exercises

1. Add varint encoding (like Protocol Buffers' `varint`): encode small integers in 1 byte, larger ones in 2-5 bytes using continuation bits.
2. Implement a `FramedCodec` that wraps messages with a 4-byte magic number and 4-byte length, and provides framed read/write for streaming protocols.
3. Write a benchmark comparing `BinaryEncoder` throughput against `serde_json` for encoding a `Vec<Record>` of 10 000 elements. Measure bytes per second.
