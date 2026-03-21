📖 **[View on hightechmind.io →](https://hightechmind.io/rust/471-string-vs-str)**

---

# String vs &str
**Difficulty:** ⭐  
**Category:** Functional Programming  



The distinction between an owned heap-allocated `String` and a borrowed string slice `&str` is fundamental to Rust's ownership model and zero-copy string handling.

## Problem Statement

Many languages have a single string type. Rust distinguishes between ownership and borrowing at the type level: `String` owns heap memory and can grow; `&str` is a fat pointer (address + length) into any UTF-8 bytes — a string literal in the binary, a slice of a `String`, or a network buffer. This design enables functions to accept both `"literals"` and `String` values without copying, a guarantee enforced at compile time rather than at runtime.

## Learning Outcomes

- Understand `String` as `Vec<u8>` with a UTF-8 invariant versus `&str` as a borrowed view
- Write functions that accept `&str` to work with both `String` and string literals
- Use `String::from` / `.to_string()` / `format!` to create owned strings
- Slice a `String` to obtain a `&str` with the same lifetime
- Implement `first_word` using byte-level `find` to return a slice of the input

## Rust Application

Functions that only read a string should accept `&str` — callers can pass `&my_string` (via `Deref<Target=str>`) or a literal directly:

```rust
fn greet(name: &str) { println!("Hello, {}!", name); }
```

`make_greeting` returns an owned `String` because it builds a new value. `first_word` returns `&str` with the same lifetime as the input: it finds the first space with `s.find(' ')` and slices `&s[..pos]`. No allocation occurs — the returned slice points into the caller's memory.

## OCaml Approach

OCaml's `string` is an immutable byte sequence; `Bytes.t` is the mutable counterpart. There is no ownership distinction at the type level:

```ocaml
let greet name = Printf.printf "Hello, %s!\n" name
let make_greeting name = "Hello, " ^ name ^ "!"
let first_word s =
  match String.index_opt s ' ' with
  | Some i -> String.sub s 0 i
  | None   -> s
```

`String.sub` always allocates a new string; there is no zero-copy slice type in the standard library (`Bigstring` from `core` provides views for I/O).

## Key Differences

1. **Ownership**: Rust's `String` is uniquely owned and freed when it goes out of scope; OCaml strings are garbage-collected — ownership is irrelevant.
2. **Zero-copy slices**: Rust `&str` slices point into existing memory; OCaml `String.sub` always copies.
3. **Mutability**: Rust `String` is mutable via `push_str`/`push`; OCaml `string` is immutable — mutation requires `Bytes.t`.
4. **Deref coercion**: `&String` coerces to `&str` automatically; OCaml has no such coercion — you pass `string` values directly.

## Exercises

1. **Longest word**: Write `longest_word(s: &str) -> &str` that returns the longest whitespace-delimited word as a slice without allocating.
2. **String conversion benchmark**: Use `criterion` to measure the cost of `.to_string()` vs. `String::from()` vs. `format!("{}", s)` for a 100-byte input.
3. **Accept `impl AsRef<str>`**: Rewrite `greet` to accept `impl AsRef<str>` and verify it works with `String`, `&str`, and `Cow<str>`.
