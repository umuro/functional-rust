📖 **[View on hightechmind.io →](https://hightechmind.io/rust/483-string-encoding)**

---

# String Encoding
**Difficulty:** ⭐  
**Category:** Functional Programming  



Rust's `char::encode_utf8`, `str::from_utf8`, and `char::len_utf8` expose the UTF-8 encoding mechanism, while `strip_prefix` handles BOM detection — the building blocks for custom encoding and protocol parsing.

## Problem Statement

Software systems communicate using standardised text encodings: HTTP headers are Latin-1 or UTF-8, XML files may start with a byte-order mark (BOM, U+FEFF), JSON must be UTF-8, and legacy databases often use Windows-1252. Rust's strings are always UTF-8 internally, but interfacing with the outside world requires encoding knowledge: how many bytes does a character occupy, how do I detect a BOM, how do I validate arbitrary bytes as UTF-8 before accepting them as `&str`?

## Learning Outcomes

- Encode a single `char` to its UTF-8 byte representation with `encode_utf8(&mut buf)`
- Query the UTF-8 byte length of a char with `char::len_utf8()`
- Validate a byte slice as UTF-8 with `std::str::from_utf8` returning `Result<&str, Utf8Error>`
- Detect and strip a UTF-8 BOM with `strip_prefix('\u{FEFF}')`
- Understand the 1/2/3/4 byte UTF-8 encoding ranges for Unicode codepoints

## Rust Application

`encode_utf8` writes a char's UTF-8 bytes into a caller-provided buffer:

```rust
let mut buf = [0u8; 4];
'A'.encode_utf8(&mut buf)   // "A" (1 byte)
'é'.len_utf8()              // 2 bytes
'\u{1F600}'.len_utf8()      // 4 bytes
```

`std::str::from_utf8` reinterprets a `&[u8]` as a `&str` with zero copying if valid:

```rust
std::str::from_utf8(&[104, 105])  // Ok("hi")
std::str::from_utf8(&[0xFF])      // Err(Utf8Error)
```

BOM stripping — important for files saved by Windows text editors:

```rust
let s = "\u{FEFF}hi";
s.strip_prefix('\u{FEFF}')  // Some("hi")
```

## OCaml Approach

OCaml encodes/decodes UTF-8 via the `Uutf` library:

```ocaml
(* Encode a Unicode codepoint to UTF-8 bytes *)
let encode_utf8 uchar =
  let buf = Buffer.create 4 in
  Uutf.Buffer.add_utf_8 buf uchar;
  Buffer.to_bytes buf

(* Validate UTF-8 *)
let is_valid_utf8 s =
  Uutf.String.fold_utf_8 (fun ok _ d ->
    ok && d <> `Malformed) true s
```

OCaml has no BOM-stripping in the standard library; a manual `if String.length s >= 3 && String.sub s 0 3 = "\xef\xbb\xbf" then String.sub s 3 ...` check is typical.

## Key Differences

1. **Zero-copy validation**: Rust's `str::from_utf8` validates and returns a `&str` pointing into the original bytes; OCaml's equivalent requires external crates and always decodes.
2. **`encode_utf8` to stack buffer**: Rust encodes a char into a stack-allocated `[u8; 4]`; OCaml's `Uutf.Buffer.add_utf_8` writes to a heap `Buffer`.
3. **BOM handling**: Rust's `strip_prefix('\u{FEFF}')` handles BOM as a normal char; OCaml needs manual byte prefix matching.
4. **`len_utf8`**: Rust provides `char::len_utf8()` as a O(1) query; OCaml has no equivalent — you must encode and measure.

## Exercises

1. **UTF-8 byte length table**: Write a function that prints each codepoint range (U+0000–U+007F, U+0080–U+07FF, U+0800–U+FFFF, U+10000–U+10FFFF) and its byte count.
2. **Streaming validator**: Implement `Utf8Validator` that accepts bytes one at a time and returns `Valid`, `Invalid`, or `Incomplete` (for a multibyte sequence split across buffers).
3. **BOM-aware reader**: Write `read_text_file(path: &Path) -> Result<String>` that reads raw bytes, detects UTF-8/UTF-16 BOM, and returns a normalised UTF-8 string (transcode UTF-16 using the `encoding_rs` crate).
