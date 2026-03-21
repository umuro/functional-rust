📖 **[View on hightechmind.io →](https://hightechmind.io/rust/762-custom-deserialize-logic)**

---

# 762-custom-deserialize-logic — Custom Deserialize Logic
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Deserialization is where input data meets your domain model — it is the boundary where untrusted data enters the system. Custom deserialization lets you validate during parsing (not after), normalize data (trim whitespace, convert case), handle version differences (accept V1 and V2 formats), and reject invalid states before they reach business logic. This is the parse-don't-validate principle applied to serialization.

## Learning Outcomes

- Implement an `Input` cursor that reads from a byte slice with bounds checking
- Read length-prefixed strings and verify UTF-8 validity during deserialization
- Validate field values during deserialization: reject empty names, negative ages
- Handle deserialization errors with typed `DeserializeError` variants
- Implement roundtrip tests: `deserialize(serialize(x)) == x`

## Rust Application

`Input<'a>` holds a `&'a [u8]` and a position cursor. `read_byte`, `read_bytes`, `read_u32`, and `read_string` advance the cursor and return `Option<T>` or `Option<String>` (validating UTF-8). The `Deserialize` trait has a `deserialize(input: &mut Input) -> Option<Self>` method. Custom `User` deserialization validates that `name` is non-empty and `age` is within a valid range during parsing, returning `None` for invalid data.

## OCaml Approach

OCaml's `Angstrom` library is a parser combinator framework for binary and text deserialization with validation. `Bin_prot` generates direct binary readers with custom validation hooks. For JSON, `Yojson.Safe` provides typed decoding with custom validators. OCaml's `Result` monad (via `let*`) chains validation steps elegantly. `ppx_sexp_conv` allows custom `t_of_sexp` implementations for validation during deserialization.

## Key Differences

1. **Cursor vs reader**: Rust's `Input` struct mirrors OCaml's `Angstrom` parser state; both track position through a byte stream.
2. **Error handling**: Rust uses `Option<T>` for simplicity here (with `?` in production using `Result`); OCaml's `Angstrom` uses `Angstrom.t` parser monad with explicit error types.
3. **Zero-copy**: Rust's `Input<'a>` borrows from the input; `serde` with Zerocopy support enables zero-copy string deserialization; OCaml requires copying for GC safety.
4. **Validation placement**: Both approaches validate during deserialization rather than after, implementing parse-don't-validate at the deserialization layer.

## Exercises

1. Extend `Deserialize for User` to handle an optional `email` field: if the length prefix is `u32::MAX`, treat it as absent (`None`); otherwise decode it as a UTF-8 string.
2. Implement `deserialize_versioned` that reads a `version: u8` byte first and dispatches to `deserialize_v1` or `deserialize_v2` accordingly.
3. Write a `Stream<T: Deserialize>` that reads repeatedly from a byte slice, yielding deserialized values until the input is exhausted — handling partial/truncated records gracefully.
