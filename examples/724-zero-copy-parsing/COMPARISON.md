# OCaml vs Rust: Zero-Copy Parsing with Byte Slices

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml tracks positions as (start, length) pairs over a shared Bytes buffer *)
type span = { buf: bytes; start: int; len: int }

let span_split_at sep s =
  let rec find i =
    if i >= s.len then None
    else if Bytes.get s.buf (s.start + i) = sep then
      Some ({ s with len = i },
            { s with start = s.start + i + 1; len = s.len - i - 1 })
    else find (i + 1)
  in find 0

let span_to_string s = Bytes.sub_string s.buf s.start s.len
```

### Rust (idiomatic — slice references)
```rust
pub fn take_until(buf: &[u8], delimiter: u8) -> Result<(&[u8], &[u8]), ParseError> {
    buf.iter()
        .position(|&b| b == delimiter)
        .map(|pos| (&buf[..pos], &buf[pos + 1..]))
        .ok_or(ParseError::MissingDelimiter(delimiter))
}

pub fn parse_key_value(line: &[u8]) -> Result<KeyValue<'_>, ParseError> {
    let (key_bytes, value_bytes) = take_until(line, b'=')?;
    Ok(KeyValue { key: as_str(key_bytes)?, value: as_str(value_bytes)? })
}
```

### Rust (Span — index-pair approach, mirrors OCaml)
```rust
#[derive(Debug, Clone, Copy)]
pub struct Span { pub start: usize, pub len: usize }

impl Span {
    pub fn slice<'a>(&self, buf: &'a [u8]) -> &'a [u8] {
        &buf[self.start..self.start + self.len]
    }
}

pub fn span_split_at(buf: &[u8], start: usize, len: usize, sep: u8) -> Option<(Span, Span)> {
    let slice = &buf[start..start + len];
    slice.iter().position(|&b| b == sep).map(|pos| {
        (Span::new(start, pos), Span::new(start + pos + 1, len - pos - 1))
    })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Buffer view | `type span = { buf: bytes; start: int; len: int }` | `&[u8]` (fat pointer: ptr + len) |
| Split result | `span * span` (tuple of spans) | `(&[u8], &[u8])` (tuple of slices) |
| UTF-8 view | `Bytes.sub_string` — **copies** | `str::from_utf8` — **borrows** |
| Lifetime contract | Implicit — GC keeps buffer alive | `<'a>` annotation — compiler enforced |
| Optional result | `'a option` | `Option<T>` |
| Fallible result | `option` or exception | `Result<T, ParseError>` |

## Key Insights

1. **OCaml's `String.sub` allocates; Rust's `&[u8]` slice never does.**
   In OCaml, extracting a substring almost always copies bytes into a new heap object.
   Rust `&[u8]` and `&str` are fat pointers (address + length) into existing memory —
   the parsed value *is* the original bytes, viewed differently.

2. **Lifetimes replace garbage collection as the safety mechanism.**
   OCaml's GC ensures the underlying `bytes` buffer is kept alive as long as any span
   references it.  Rust achieves the same guarantee at compile time through lifetime
   annotations: `struct RequestLine<'a>` cannot outlive the `&'a [u8]` it was parsed from.
   Use-after-free is rejected before the binary is produced.

3. **The `Span` struct is the OCaml idiom; slice references are the Rust idiom.**
   OCaml must carry `buf` inside every span because references are opaque.
   Rust fat-pointer slices already carry both address and length, so the idiomatic
   Rust equivalent of a span is just `&[u8]` — no wrapper struct required.

4. **Iterator-based field parsers compose without allocation.**
   `CsvFields` is a lazy `Iterator<Item = Result<&str, _>>` that yields slices into the
   original row buffer.  In OCaml a comparable implementation would either allocate a
   list of substrings or thread an explicit index through a recursive function.

5. **`?` operator + `Result` makes zero-copy parsers as ergonomic as exception-based ones.**
   OCaml parsers often raise exceptions for error paths.  Rust's `?` propagates
   `Result::Err` up the call stack with the same brevity, but without hidden control flow
   and with explicit error types that the caller can inspect or recover from.

## When to Use Each Style

**Use idiomatic Rust `&[u8]` / `&str` slices when:** you control the full parser pipeline
in one crate and want maximum ergonomics — the compiler infers lifetimes in most cases
and the code reads like a sequence of combinator calls.

**Use the `Span` index-pair style when:** you need to store multiple parsed views alongside
the buffer in a single struct (a self-referential pattern that Rust slices cannot express
directly without unsafe), or when passing parsed results across FFI boundaries where raw
pointer + length pairs are expected.
