# 730: Small String Optimization

**Difficulty:** 3  **Level:** Advanced

Store strings up to 23 bytes inline in the enum variant — heap allocation only for longer strings.

## The Problem This Solves

In most programs, the majority of strings are short: identifiers, labels, error codes, keys in hash maps, command names. Rust's `String` allocates every string on the heap, even `"ok"`. For a hash map with millions of short-string keys, that's millions of heap allocations — each with allocation overhead, an extra pointer to follow, and a distinct cache line for each string's characters.

The Small String Optimisation (SSO) trades a slightly larger stack footprint for heap allocation elimination on short strings. Strings up to a threshold length (commonly 15–23 bytes) are stored directly in the enum variant — on the stack or inline in the struct. Only longer strings go to the heap. C++'s `std::string`, Rust's `compact_str` crate, and many production systems use this technique.

The beauty of Rust's enum is that SSO is expressible in entirely safe code: `Inline { buf: [u8; 23], len: u8 }` holds the string bytes directly, and the discriminant tracks which variant is active. The `match` at read time is free — the CPU branch predictor learns the pattern in hot loops.

## The Intuition

Normally, a Rust `String` is three words: pointer + length + capacity. For a 3-character string like `"yes"`, you pay three words of overhead plus a heap round-trip just to store 3 bytes. SSO says: if the string fits in the space we'd use for the pointer and its friends, just put the string there directly. No heap, no pointer, no allocation.

The 23-byte limit in this example is not arbitrary: on a 64-bit system, `size_of::<String>()` is 24 bytes. We use 23 bytes for characters and 1 byte for the length — exactly 24 bytes total. The `SsoString` enum has the same size as `String`, but avoids the heap for short strings.

## How It Works in Rust

```rust
const INLINE_CAP: usize = 23;

#[derive(Debug)]
enum SsoString {
    Inline { buf: [u8; INLINE_CAP], len: u8 },
    Heap(Box<str>),
}

impl SsoString {
    pub fn new(s: &str) -> Self {
        if s.len() <= INLINE_CAP {
            let mut buf = [0u8; INLINE_CAP];
            buf[..s.len()].copy_from_slice(s.as_bytes());
            SsoString::Inline { buf, len: s.len() as u8 }
        } else {
            SsoString::Heap(s.into())
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SsoString::Inline { buf, len } =>
                std::str::from_utf8(&buf[..*len as usize]).unwrap(),
            SsoString::Heap(s) => s,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            SsoString::Inline { len, .. } => *len as usize,
            SsoString::Heap(s) => s.len(),
        }
    }
}
```

For production use, the `compact_str` crate implements a battle-tested SSO string with a 24-byte `String`-compatible layout, `Display`, `Debug`, `Eq`, `Hash`, and `Deref<Target=str>`.

## What This Unlocks

- **Zero heap allocation for short strings**: Hash map keys, enum variant names, command strings, labels — all stored inline without touching the allocator.
- **Better cache locality**: Short strings live next to the struct that contains them — no pointer chase to a distant heap allocation.
- **Pattern matching dispatch**: The `Inline`/`Heap` branch is predictable — in a workload where most strings are short, the branch predictor eliminates its cost.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String storage | Always heap-allocated `string` | `String` = heap; `&str` = borrowed slice |
| Custom SSO type | GADT or abstract type | `enum` with `Inline`/`Heap` variants |
| Inline bytes | Not expressible | `[u8; N]` inline in enum variant |
| Size control | No control | `repr(C)` / explicit layout matches `String` |
| Production SSO | Not in stdlib | `compact_str` crate |
| Length encoding | Not applicable | 1 byte in `len: u8` field |
