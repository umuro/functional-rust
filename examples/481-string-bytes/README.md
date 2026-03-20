📖 **[View on hightechmind.io →](https://hightechmind.io/rust/481-string-bytes)**

---

# String Bytes
**Difficulty:** ⭐  
**Category:** Functional Programming  


Rust provides `.bytes()` for raw byte iteration and `String::from_utf8` / `from_utf8_lossy` for converting byte vectors back to strings with explicit UTF-8 validation.

## Problem Statement

Network protocols, file formats, and cryptographic functions operate on bytes, not characters. A Rust `String` is a validated UTF-8 `Vec<u8>`, but sometimes you need the raw bytes: serialising to a binary protocol, computing a checksum, or interfacing with a C library that returns `*const u8`. The reverse — constructing a `String` from bytes — requires validation because not all byte sequences are valid UTF-8. Rust makes this validation explicit with `from_utf8` (strict) and `from_utf8_lossy` (replaces invalid bytes with U+FFFD).

## Learning Outcomes

- Iterate raw bytes with `.bytes()` yielding `u8` values
- Convert a `Vec<u8>` to `String` with `String::from_utf8`, which returns `Result`
- Validate a `&[u8]` slice as UTF-8 with `std::str::from_utf8`
- Use `String::from_utf8_lossy` to convert potentially invalid bytes with replacement characters
- Understand the relationship between `&str`, `&[u8]`, `String`, and `Vec<u8>`

## Rust Application

`.bytes()` yields the raw UTF-8 bytes — for ASCII strings each byte equals the character code:

```rust
"hi".bytes().collect::<Vec<_>>()  // [104, 105]
```

Constructing a `String` from bytes requires explicit validation:

```rust
String::from_utf8(vec![104, 105]).unwrap()  // "hi"
String::from_utf8(vec![0xFF]).is_err()       // true — invalid UTF-8
```

`from_utf8_lossy` returns `Cow<str>` — `Borrowed` if the bytes are valid UTF-8, `Owned` with replacements if not:

```rust
let s = String::from_utf8_lossy(&[104, 0xFF, 105]);  // "h\u{FFFD}i"
```

## OCaml Approach

OCaml's `Bytes.t` is a mutable byte sequence; `string` is an immutable byte sequence. There is no UTF-8 validation in the standard library:

```ocaml
(* Bytes to string — unsafe in OCaml, no validation *)
let bytes = Bytes.of_string "hi"
let s = Bytes.to_string bytes

(* For UTF-8 validation, use uutf *)
let is_valid_utf8 s =
  Uutf.String.fold_utf_8 (fun ok _ d ->
    ok && d <> `Malformed) true s
```

OCaml makes no UTF-8 guarantees at the `string` type level — it is the programmer's responsibility.

## Key Differences

1. **Type-level guarantee**: Rust's `String`/`&str` guarantee UTF-8 validity; OCaml's `string` is unchecked bytes.
2. **Explicit conversion**: Rust requires `from_utf8` (returning `Result`) to go from bytes to string; OCaml's `Bytes.to_string` is unconditional.
3. **`from_utf8_lossy`**: Rust provides a built-in lossy decoder that replaces invalid bytes; OCaml needs `Uutf` or manual implementation.
4. **`&str` as `&[u8]`**: Rust's `str::as_bytes()` gives a `&[u8]` view with no copy; OCaml's `String.to_bytes` allocates a new `Bytes.t`.

## Exercises

1. **Hex encode**: Write `to_hex(s: &str) -> String` that formats each byte as two lowercase hex digits using `.bytes()` and `format!("{:02x}", b)`.
2. **UTF-8 validator**: Implement `is_valid_utf8(bytes: &[u8]) -> bool` using `std::str::from_utf8` and write tests for valid ASCII, valid multibyte sequences, and truncated multibyte sequences.
3. **Null-terminated bytes**: Write `to_c_str(s: &str) -> Vec<u8>` that appends a null byte — handling any embedded nulls as an error — to produce a C-compatible byte string.
