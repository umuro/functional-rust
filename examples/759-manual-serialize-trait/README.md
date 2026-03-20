📖 **[View on hightechmind.io →](https://hightechmind.io/rust/759-manual-serialize-trait)**

---

# 759-manual-serialize-trait — Manual Serialize Trait

## Problem Statement

Before `serde` existed, Rust programmers wrote serialization by hand. Understanding manual serialization reveals what `#[derive(Serialize)]` generates and why `serde`'s design is what it is. Manual serialization is still needed when: the format has custom requirements, you target `no_std` environments, you need maximum performance without abstraction overhead, or you are implementing a wire protocol like MessagePack or Protocol Buffers.

## Learning Outcomes

- Design a `Serialize` trait with `serialize(&self, out: &mut Output)` method
- Implement an `Output` buffer with typed write methods (`write_u32`, `write_string`, `write_byte`)
- Implement `Serialize` for primitives, strings, and nested structs manually
- Create a corresponding `Deserialize` trait and `Input` buffer
- Understand the difference between text serialization (JSON) and binary serialization (this example)

## Rust Application

`Output` wraps `Vec<u8>` with typed write methods: `write_byte`, `write_u32` (little-endian), `write_string` (length-prefixed). The `Serialize` trait provides `serialize(&self, out: &mut Output)` and a convenience `to_bytes(&self) -> Vec<u8>`. Implementations cover `u8`, `u32`, `bool`, `String`, `&str`, and composite structs. Roundtrip tests verify `from_bytes(to_bytes(x)) == x` for all types.

## OCaml Approach

OCaml's `Marshal` module provides automatic binary serialization of any value, including closures and mutable references. For typed serialization, `Bin_prot` (Jane Street) provides a manual trait-like approach: `bin_write_t`, `bin_read_t`, and `bin_size_t` functions per type. `sexplib` (also Jane Street) does the same for S-expression serialization. These are the OCaml equivalents of Rust's `serde`.

## Key Differences

1. **Trait vs function**: Rust uses a `Serialize` trait with method dispatch; OCaml's `Bin_prot` uses module values (`bin_writer_t`) without a unified trait.
2. **derive**: Both languages have derive macros for automatic implementation; Rust's `#[derive(Serialize)]` and OCaml's `[@@deriving bin_io]` generate similar code.
3. **Generic containers**: Rust's `impl<T: Serialize> Serialize for Vec<T>` is idiomatic; OCaml uses parametrized `bin_write_list` functions.
4. **no_std**: Rust's manual serialization works in `no_std` environments; OCaml's runtime always requires a full GC.

## Exercises

1. Implement `Serialize` for `Option<T: Serialize>` using a one-byte discriminant (0 for None, 1 for Some) followed by the serialized value.
2. Add `Serialize` for `Vec<T: Serialize>` using a 4-byte length prefix, and write a roundtrip test for a struct containing a `Vec<String>`.
3. Implement a length-delimited framing layer: write `[4 bytes: total_len][serialized_data]` and read it back, enabling streaming protocols over TCP.
