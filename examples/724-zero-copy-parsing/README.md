# Zero-Copy Parsing

> **Functional Rust** · [hightechmind.io](https://hightechmind.io)

## Problem Statement

A naive parser copies every token from the input buffer into a freshly allocated
`String`. For a 100 MB JSON file, this means allocating millions of strings, applying
GC pressure, and touching twice the memory bandwidth. Zero-copy parsing eliminates
these allocations by returning borrowed references (`&str`, `&[u8]`) into the original
input buffer. The parse result's lifetime is tied to the input's lifetime, preventing
the input from being freed while tokens are still accessible.

The pattern originated in high-frequency trading systems (parsing FIX protocol messages
millions of times per second), network proxies (forwarding HTTP headers without copying),
and embedded systems (parsing sensor data from a DMA buffer). Rust's lifetime system
makes zero-copy safe by statically ensuring that borrowed parse results cannot outlive
their input. Languages with GC (Java, Python, OCaml) achieve this only through careful
discipline; Rust enforces it at compile time.

## Learning Outcomes

- Implement a zero-copy parser that returns `&str` / `&[u8]` slices into input
- Use lifetime annotations to tie parse output lifetimes to input lifetimes
- Represent parse errors with `enum ParseError` without heap allocation
- Apply `split_once`, `splitn`, and manual byte scanning to avoid allocation
- Understand when zero-copy is impossible (e.g., unescaping, base64 decoding)

## Rust Application

```rust
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEof,
    InvalidToken,
    ExpectedChar(char),
}

// Parse result borrows from input — 'input lifetime
pub struct Record<'input> {
    pub key:   &'input str,
    pub value: &'input str,
}

// Returns a slice into `input`, no allocation
pub fn parse_record(input: &str) -> Result<Record<'_>, ParseError> {
    let (key, rest) = input
        .split_once('=')
        .ok_or(ParseError::InvalidToken)?;
    let value = rest
        .split_once(';')
        .map(|(v, _)| v)
        .unwrap_or(rest);
    Ok(Record { key: key.trim(), value: value.trim() })
}

// Parse a delimited list without allocating
pub fn parse_csv_fields(line: &str) -> impl Iterator<Item = &str> {
    line.split(',').map(str::trim)
}

// Binary zero-copy: parse a length-prefixed frame
pub fn parse_frame(buf: &[u8]) -> Result<(&[u8], &[u8]), ParseError> {
    if buf.len() < 4 {
        return Err(ParseError::UnexpectedEof);
    }
    let len = u32::from_be_bytes(buf[..4].try_into().unwrap()) as usize;
    if buf.len() < 4 + len {
        return Err(ParseError::UnexpectedEof);
    }
    Ok((&buf[4..4 + len], &buf[4 + len..]))
}
```

The lifetime `'input` on `Record` communicates to callers: "you cannot drop the
original string while this record is alive." The compiler enforces this.

## OCaml Approach

OCaml strings are immutable and the GC manages their lifetime, so "zero-copy" means
using `String.sub` (which does copy) or `Bytes.sub_string`. True zero-copy requires
`Bigstring`/`Bigarray` or the `Angstrom` parser combinator library with `Lwt`:

```ocaml
(* Copies substring — not zero-copy *)
let parse_key_value s =
  match String.split_on_char '=' s with
  | [k; v] -> Ok (String.trim k, String.trim v)
  | _       -> Error "invalid"

(* Zero-copy with Angstrom (returns Bigstring slices) *)
(* let record_parser = ... Angstrom.take_while ... *)
```

The `Angstrom` library uses `Bigstring` (a `Bigarray.Array1` of `char`) as the backing
buffer and returns offsets rather than copies, achieving true zero-copy in practice.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Borrowing input | Lifetime-annotated `&str` | GC-managed; string.sub copies |
| Safety enforcement | Compile-time lifetime check | Runtime / discipline |
| Binary frames | `&[u8]` slices, no copy | `Bigstring` with `Angstrom` |
| Error type | Enum, stack-allocated | `string` or exception |
| Parser libraries | `nom`, `winnow` (zero-copy) | `Angstrom` (Bigstring) |

## Exercises

1. Implement a zero-copy HTTP/1.1 request-line parser returning `(&str, &str, &str)`
   for method, path, and version. Write property tests verifying no allocation occurs
   (use `bumpalo` as allocator oracle).
2. Extend `parse_frame` to return an iterator over multiple consecutive frames in a
   buffer, with each frame borrowing from the original `&[u8]`.
3. Implement a zero-copy JSON string tokenizer that returns `&str` slices for
   unescaped strings but falls back to `String` for strings containing `\uXXXX` escapes
   (use `Cow<'_, str>`).
4. Benchmark your CSV field parser vs one that collects into `Vec<String>`. Measure
   allocations with `heaptrack` or the `dhat` allocator.
5. Write a `nom`-based parser for a simple binary format and compare its generated code
   to your hand-rolled version using `cargo asm`.
