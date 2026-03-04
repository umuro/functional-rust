# 768: Zero-Copy Deserialisation with Lifetime Tricks

**Difficulty:** 4  **Level:** Advanced

Deserialize structured data into a struct whose string fields borrow directly from the input buffer — no heap allocation for string data.

## The Problem This Solves

Every time you deserialize a JSON or CSV record into an owned `String`, you pay for an allocation and a copy of each string field. For parsing millions of records in a hot loop, this is a significant cost. Zero-copy deserialization eliminates it: the parsed struct holds `&str` slices that point directly into the original input bytes.

This technique is the foundation of `serde`'s `Deserialize<'de>` design — the `'de` lifetime is exactly this: it ties the struct's fields to the lifetime of the deserialization input. Understanding how to implement it without `serde` first makes `serde`'s lifetime signature (`impl<'de> Deserialize<'de> for MyType`) intuitive rather than mysterious.

The tradeoff: the input buffer must outlive the parsed struct. When you're done processing, convert to owned values with `PersonOwned::from(view)`.

## The Intuition

A `PersonView<'de>` is a view into a specific input buffer — the `'de` lifetime parameter says "I was carved out of a buffer that lives at least as long as `'de`." The struct's `&'de str` fields are just pointers and lengths into that buffer — no allocation.

The parser returns `PersonView<'_>` where `'_` is inferred from the input: the returned struct can't outlive the string passed to `parse_view`. The compiler enforces this automatically.

## How It Works in Rust

**The zero-copy struct** — fields borrow from `'de`:
```rust
#[derive(Debug)]
pub struct PersonView<'de> {
    pub name:    &'de str,
    pub age_raw: &'de str,
    pub city:    Option<&'de str>,
}
```
No `String`, no allocation. Every field is a slice of the input.

**The parser** — returns borrows from the input:
```rust
pub fn parse_view(input: &str) -> Result<PersonView<'_>, ParseError> {
    fn find_field<'a>(input: &'a str, key: &str) -> Option<&'a str> {
        for part in input.split('|') {
            if let Some(v) = part.strip_prefix(&format!("{key}=")) {
                return Some(v);  // slice of input, not a copy
            }
        }
        None
    }
    // ...
}
```
`'_` in the return type is shorthand for "borrows from `input`" — the compiler infers the lifetime.

**Explicit lifetime version** — showing `'de` in full:
```rust
pub fn deserialize_person<'de>(input: &'de str) -> Result<PersonView<'de>, ParseError> {
    parse_view(input)
}
```
Input and output share the same lifetime `'de` — the struct fields will be valid exactly as long as `input` is.

**Batch parsing** — multiple views from one buffer:
```rust
pub fn parse_many(input: &str) -> Vec<PersonView<'_>> {
    input.lines()
         .filter(|l| !l.is_empty())
         .filter_map(|line| parse_view(line).ok())
         .collect()
}
```
All views share the same input buffer — one allocation (for `Vec`), no copies of string data.

**Converting to owned when needed:**
```rust
impl<'de> From<PersonView<'de>> for PersonOwned {
    fn from(v: PersonView<'de>) -> Self {
        PersonOwned {
            name: v.name.to_string(),  // allocate only when leaving the zero-copy context
            age: v.age().unwrap_or(0),
            city: v.city.map(|s| s.to_string()),
        }
    }
}
```

## What This Unlocks

- **High-throughput parsing** — process millions of records with zero string allocation per record.
- **Understanding `serde`'s `'de` lifetime** — `impl<'de> Deserialize<'de>` is exactly this pattern, built into `serde`.
- **View/owned duality** — parse as zero-copy views, convert to owned only for the records you actually need to store.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Zero-copy string | `Bytes.sub` (view) or bigstring libs | `&'de str` — lifetime tracks input buffer |
| Lifetime on struct | N/A (GC) | `struct Foo<'de>` — struct borrows from `'de` |
| `serde`-style `'de` | `ppx_deriving`, `jsonaf` (own GC strings) | `impl<'de> Deserialize<'de>` — borrows from input |
| Convert view to owned | `String.copy s` | `.to_string()` — explicit allocation at crossing point |
