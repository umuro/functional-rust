📖 **[View on hightechmind.io →](https://hightechmind.io/rust/113-string-str)**

---

# 113-string-str — String vs &str

## Problem Statement

Rust has two primary string types: `String` (owned, heap-allocated, growable) and `&str` (borrowed string slice — a pointer + length into any UTF-8 data). This distinction is analogous to `std::string` vs `const char*` in C++, but with full UTF-8 guarantees and lifetime safety. Choosing the right type for function parameters and return types affects performance, API ergonomics, and ownership semantics.

The rule of thumb: accept `&str` in function parameters (works with both `String` and `&str`), return `String` when ownership is needed, and return `&str` only when borrowing from an input.

## Learning Outcomes

- Understand `String` as owned, heap-allocated, growable UTF-8
- Understand `&str` as a borrowed slice with a length (no ownership)
- Write functions that accept `&str` and work with both `String` and string literals
- Build `String`s by pushing, appending, and formatting
- Understand `Deref` coercion: `&String` automatically coerces to `&str`

## Rust Application

`src/lib.rs` demonstrates three patterns. `first_word(s: &str) -> &str` accepts any string and returns a borrowed slice — zero allocation. `char_count(s: &str) -> usize` counts Unicode scalar values. `append(base: &str, suffix: &str) -> String` builds a new owned `String` from two borrows. `greet(name: &str) -> String` shows `String::from` and `push_str` for incremental building.

`Deref` coercion means you can pass `&my_string` where `&str` is expected — the compiler automatically converts `&String` to `&str`.

## OCaml Approach

OCaml has one string type: `string` (immutable byte sequence since OCaml 4.06). There is no distinction between owned and borrowed strings — the GC handles all lifetime management:

```ocaml
let first_word s = String.split_on_char ',' s |> List.hd |> String.trim
let char_count s = String.length s  (* byte count, not Unicode *)
let append base suffix = base ^ suffix  (* allocates new string *)
```

OCaml's `^` operator always allocates a new string. Rust's `push_str` avoids allocation when the `String` has sufficient capacity.

## Key Differences

1. **Ownership**: Rust's `String` is owned (dropped when out of scope); OCaml strings are GC-managed — no explicit ownership.
2. **Zero-copy return**: Rust can return `&str` pointing into the original data (zero allocation); OCaml's `String.sub` allocates.
3. **Growability**: Rust's `String` can grow with `push_str`; OCaml strings are immutable (use `Buffer` for mutable building).
4. **Deref coercion**: `&String` coerces to `&str` automatically; OCaml has no equivalent because there is only one type.

## Exercises

1. Write a function `count_words(s: &str) -> usize` that counts space-separated words without allocating.
2. Implement `title_case(s: &str) -> String` that capitalizes the first letter of each word.
3. Write `split_once_custom<'a>(s: &'a str, delim: char) -> Option<(&'a str, &'a str)>` that returns borrowed slices of the input.
