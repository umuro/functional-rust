📖 **[View on hightechmind.io →](https://hightechmind.io/rust/768-zero-copy-deserialize)**

---

# 768-zero-copy-deserialize — Zero-Copy Deserialize
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Deserialization normally copies data: the input bytes are parsed and new heap-allocated strings and vectors are created. Zero-copy deserialization avoids this by returning references (`&str`, `&[u8]`) that point directly into the input buffer. For high-throughput network servers processing thousands of requests per second, eliminating these copies can halve memory bandwidth usage. `serde`'s `#[serde(borrow)]` attribute enables zero-copy deserialization for string fields.

## Learning Outcomes

- Return `&str` slices from parsing functions that borrow from the input
- Understand lifetime parameters on parsed types: `Message<'a>`, `KeyValue<'a>`
- Implement `parse_message`, `parse_kv`, and `parse_csv_row` returning borrowed references
- See how Rust's lifetime system prevents use-after-free from zero-copy parsing
- Understand the trade-off: zero-copy requires the input buffer to live as long as the parsed value

## Rust Application

`Message<'a>` holds `header: &'a str` and `body: &'a str` — both point into the input string. `parse_message` finds the newline, slices the input, and returns references without copying. `KeyValue<'a>` has `key: &'a str` and `value: &'a str`. `CsvRow<'a>` holds `fields: Vec<&'a str>` — field strings point into the input. All parsing is done with `str::find` and slice indexing; no `String::from` or `to_owned` calls occur.

## OCaml Approach

OCaml's GC makes zero-copy more complex: since strings are GC-managed, returning a substring typically requires either a copy or using `String.sub` (which copies). `Bigstringaf` provides a mutable, GC-tracked byte buffer where substrings can be represented as offset-length pairs without copying. `Angstrom` uses this for zero-copy network parsing. The `Cstruct` library in MirageOS provides zero-copy buffer slices for network protocols.

## Key Differences

1. **Lifetime tracking**: Rust's `'a` lifetime on returned references is checked at compile time; OCaml has no equivalent — the GC handles lifetime but cannot prevent logical errors.
2. **String representation**: Rust's `&str` is a fat pointer (ptr + len) into an existing buffer; OCaml's substring always allocates a new string.
3. **Production use**: Rust's `serde` with `#[serde(borrow)]` enables zero-copy JSON parsing; `serde_json::from_str::<Message<'_>>` avoids all string allocation for borrowed fields.
4. **Buffer lifetime**: Rust enforces that the parsed value cannot outlive the input buffer; OCaml's GC keeps the buffer alive as long as any string derived from it exists.

## Exercises

1. Implement `parse_http_request<'a>(input: &'a str) -> Option<HttpRequest<'a>>` where `HttpRequest` borrows method, path, and header values from the input.
2. Add a `split_fields<'a>(s: &'a str, delim: char) -> impl Iterator<Item = &'a str>` that returns borrowed field slices without allocating a `Vec`.
3. Write a benchmark comparing `parse_message` (zero-copy, returns `&str`) against a copying version (returns `String`) for 1 million parses. Measure allocation count with a custom allocator.
