📖 **[View on hightechmind.io →](https://hightechmind.io/rust/475-string-building)**

---

# String Building
**Difficulty:** ⭐  
**Category:** Functional Programming  



Rust provides several patterns for constructing strings incrementally: `push_str`/`push` for sequential appending, `join` for separating collections, `collect` from iterators, `repeat` for duplication, and `with_capacity` for pre-allocated buffers.

## Problem Statement

Naive string concatenation with `+` or repeated `format!` calls create a new heap allocation on every operation — O(N²) work for N concatenations. Efficient string building requires pre-allocating capacity and appending in-place. This is the same problem Java's `StringBuilder`, Python's `"".join(list)`, and C's `strbuf` solve. Rust's `String` is essentially a `Vec<u8>` with UTF-8 invariants, giving direct access to push and capacity management.

## Learning Outcomes

- Append to a `String` with `push_str` (multiple chars) and `push` (single char)
- Join an iterator of strings with a separator using `slice::join`
- Transform and collect characters from an iterator into a new `String`
- Repeat a string N times with `.repeat(n)`
- Pre-allocate capacity with `String::with_capacity` to avoid reallocation

## Rust Application

`push_str` appends a `&str` and `push` appends a `char`; both are amortised O(1):

```rust
let mut s = String::new();
s.push_str("hi");
s.push('!');
assert_eq!(s, "hi!");
```

`.join("-")` is the idiomatic way to concatenate a slice of strings with a separator — equivalent to Python's `"-".join(list)`. Characters can be transformed and recollected:

```rust
let s: String = "abc".chars().rev().collect();
```

`String::with_capacity(n)` pre-allocates at least `n` bytes, preventing reallocation during a known-size build loop.

## OCaml Approach

OCaml's `Buffer` module is the `StringBuilder` equivalent:

```ocaml
let buf = Buffer.create 64
let () = Buffer.add_string buf "hi"
let () = Buffer.add_char buf '!'
let s = Buffer.contents buf  (* "hi!" *)
```

`String.concat "-" ["a";"b";"c"]` joins with a separator. `String.init n f` builds a string by calling `f i` for each index. OCaml lacks a built-in `repeat`; the idiom is `String.concat "" (List.init n (Fun.const s))`.

## Key Differences

1. **`Buffer` vs. `String`**: OCaml separates the mutable builder (`Buffer.t`) from the immutable result (`string`); Rust's `String` is mutable throughout its lifetime.
2. **Collect from iterator**: Rust collects any `Iterator<Item=char>` or `Iterator<Item=&str>` directly into `String` via the `FromIterator` trait; OCaml requires `String.concat "" (List.map ...)`.
3. **Capacity hints**: Rust's `String::with_capacity` is explicit and inspectable; OCaml's `Buffer.create hint` takes an initial capacity hint but it is advisory.
4. **`repeat`**: Rust has `str::repeat` in the standard library; OCaml needs a manual loop or `String.concat "" (List.init n (Fun.const s))`.

## Exercises

1. **Word capitaliser**: Write `capitalize_words(s: &str) -> String` that capitalises the first letter of each whitespace-delimited word, building the result with `push`/`push_str` and no intermediate `Vec`.
2. **CSV builder**: Implement `fn csv_row(fields: &[&str]) -> String` using `String::with_capacity` to pre-allocate the exact needed capacity (sum of field lengths + separators).
3. **Rope data structure**: Research the Rope data structure and sketch its advantages over `String` for large documents with frequent mid-string insertions.
