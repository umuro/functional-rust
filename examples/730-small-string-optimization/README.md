📖 **[View on hightechmind.io →](https://hightechmind.io/rust/730-small-string-optimization)**

---

# 730-small-string-optimization — Small String Optimization

## Problem Statement

Most strings in real applications are short: identifiers, keys, tags, status codes. Yet `String` always heap-allocates, adding a pointer indirection, an allocator round-trip, and cache pressure. Small String Optimization (SSO) stores strings up to a threshold length (here 23 bytes) directly inside the enum variant, avoiding any heap allocation. This technique is used in C++'s `std::string`, Rust's `smol_str` and `compact_str` crates, and many database engines for short column values.

## Learning Outcomes

- Implement an SSO string type as a Rust enum with `Inline` and `Heap` variants
- Store inline bytes in a `[u8; 23]` array with a separate `len: u8` field to fit 24 bytes total
- Understand how the enum discriminant and data fit into the same 24-byte footprint as `String`
- Recognize when to fall back to heap allocation for longer strings
- See how `is_inline()` can guide hot-path decisions in query engines

## Rust Application

`SsoString` is a `Debug` enum with two variants: `Inline { buf: [u8; 23], len: u8 }` and `Heap(Box<str>)`. Both fit in 24 bytes on 64-bit systems — the same size as `String`. `SsoString::new` inspects `s.len()` and chooses the variant; `as_str()` pattern-matches to return a `&str` from either. The `is_inline()` helper lets callers fast-path queries that know they are working with short strings.

## OCaml Approach

OCaml's `string` type is heap-allocated via the GC but represented as a flat byte array with no separate length word overhead (length is stored in the GC block header). For very short strings OCaml's minor GC makes allocation nearly free. The `Bytes` module provides mutable string buffers. There is no standard SSO type, but libraries like `Base` use compact representations for identifiers.

## Key Differences

1. **Allocation model**: Rust SSO avoids the heap entirely for short strings; OCaml relies on the GC's minor heap to make short allocations cheap rather than avoiding them.
2. **Size control**: Rust enums give explicit control over the 24-byte layout; OCaml's `string` size is determined by the runtime block header format.
3. **Mutability**: Rust's `SsoString` is immutable after construction; OCaml's `Bytes.t` is a mutable byte array, distinct from the immutable `string` type.
4. **Crate ecosystem**: Rust has `smol_str`, `compact_str`, and `inline-str` crates for production SSO; OCaml has no widely adopted equivalent.

## Exercises

1. Extend `SsoString` to support `push_str` that transitions `Inline` to `Heap` when the result exceeds 23 bytes.
2. Add a `Concat` associated function that combines two `SsoString` values without allocating when both fit inline.
3. Benchmark `SsoString::new` against `String::from` for strings of length 1, 12, 23, and 50 bytes. Plot the crossover point.
