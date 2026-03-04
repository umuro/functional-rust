# 480: chars() and Char-Level Operations

**Difficulty:** 1  **Level:** Intermediate

Iterate Unicode scalar values correctly — and why you can't index a Rust string with `[i]`.

## The Problem This Solves

In Python, `"café"[3]` gives you `'é'`. In JavaScript, `"café"[3]` gives you `'é'` too (at least for Basic Multilingual Plane characters). It just works. In Rust, `"café"[3]` is a **compile error**. You can't index a string with `[]`. 

Why? Because Rust strings are UTF-8 bytes. `"café"` is 5 bytes, not 4. The `é` takes 2 bytes. If Rust let you write `s[3]`, what would you get — a byte? Half a character? A panic? The designers decided: if the operation is ambiguous or potentially unsound, it shouldn't compile.

The correct approach: `.chars()` gives you an iterator of Unicode scalar values (Rust's `char` — a 4-byte value covering all of Unicode). Use `.chars().nth(3)` instead of `s[3]`. Use `.chars().count()` instead of `s.len()` when you want character count. Use `.chars().filter()`, `.chars().map()`, `.collect()` for character-level transformations.

## The Intuition

`chars()` is "iterate this string as a sequence of Unicode code points." Each element is a `char` — a 32-bit value representing one Unicode scalar value (emoji 🌍 included).

Python's `for c in "café"` does the same thing. The difference: Rust makes it explicit that you're iterating chars, not bytes. You can't accidentally iterate bytes when you meant characters.

The workflow for character transformations: iterate with `.chars()`, transform with iterator adapters (`map`, `filter`, `rev`, `enumerate`), then collect back to a `String` with `.collect()`. This is idiomatic, composable, and clear.

Key limitation: `.chars()` doesn't give you grapheme clusters. `"e\u{0301}"` (e + combining accent) is *two* chars but *one* user-perceived character. For grapheme clusters, use the `unicode-segmentation` crate.

## How It Works in Rust

```rust
let s = "Hello, World! 🌍";

// len() = byte count, chars().count() = Unicode scalar count
println!("{} bytes, {} chars", s.len(), s.chars().count());
// 18 bytes, 15 chars  (🌍 is 4 bytes, 1 char)

// chars() with enumerate — gives index + char
for (i, c) in s.chars().enumerate().take(5) {
    println!("[{}] '{}' U+{:04X}", i, c, c as u32);
}

// Map — transform each char, collect back to String
let upper: String = s.chars()
    .map(|c| c.to_uppercase().next().unwrap())
    .collect();

// Filter — keep only alphabetic chars
let alpha: String = s.chars()
    .filter(|c| c.is_alphabetic())
    .collect();  // "HelloWorld"

// Reverse — works correctly for multi-byte chars!
let rev: String = s.chars().rev().collect();
// String reversal by byte index would break UTF-8

// nth — O(n) but correct for Unicode
s.chars().nth(2)  // Some('l')

// Can't do: s[2]  ← compile error
// Can't do: s[2..3]  ← panics if not on char boundary
```

## What This Unlocks

- **Safe string reversal** — `.chars().rev().collect()` handles multi-byte characters correctly.
- **Unicode-aware transformations** — filter emoji, count letters, apply ROT13, all via iterator chains.
- **Character-level validation** — `s.chars().all(|c| c.is_ascii())` without importing anything.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Iterate characters | `String.iter s` (bytes in practice) | `s.chars()` (Unicode scalar values) |
| Index by char | `s.[i]` (byte, not char) | `s.chars().nth(i)` — `Option<char>` |
| Character count | Manual UTF-8 decode | `s.chars().count()` |
| Map over chars | `String.map f s` | `s.chars().map(f).collect::<String>()` |
| Reverse | Custom loop | `s.chars().rev().collect()` |
| Filter chars | `String.concat "" (List.filter ...)` | `s.chars().filter(\|c\| ...).collect()` |
| Direct indexing | `s.[i]` (unsafe for UTF-8) | **Compile error** — intentionally disallowed |
