# 485: Efficient String Concatenation

**Difficulty:** 1  **Level:** Beginner

Concatenate strings without O(n²) copies — and understand why `+` moves its left operand.

## The Problem This Solves

In Python, `"a" + "b" + "c"` creates two intermediate strings. In a loop, `result += part` is O(n²). The fix: `"".join(parts)`. JavaScript has the same issue; the fix is `Array.join` or template literals.

Rust has a twist: the `+` operator on `String` **moves** the left operand. `let s = a + &b` consumes `a` — you can't use `a` after this. This is Rust reusing `a`'s buffer for the result, avoiding one allocation. But `a + &b + &c + &d` still creates intermediate strings, just like Python.

For multiple pieces, Rust has better options: `.join()` for slices of strings (single allocation), `format!()` for readability, and `String::with_capacity` + `push_str` for maximum control. Knowing which to use matters when you're building long strings in hot paths.

## The Intuition

The `+` operator for `String` is `fn add(self, rhs: &str) -> String` — it takes `self` by value (moves it), borrows the right side. This reuses the left-hand buffer when possible. But chaining `+` — `a + &b + &c` — is `(a + &b) + &c`: the first `+` returns a temporary `String`, the second `+` moves that temporary and appends `&c`. Still O(n²) in a loop.

Mental model:
- **`format!()`** — readable, always allocates a new string, fine for small/one-off strings
- **`.join(sep)`** — for "I have a slice of strings, join them with a separator" — single allocation
- **`push_str` + `with_capacity`** — for building incrementally with known max size
- **`+`** — fine for 2-3 pieces, but moves the left operand

## How It Works in Rust

```rust
// + operator — moves left, borrows right
let a = String::from("Hello");
let b = String::from(", World!");
let s = a + &b;   // a is MOVED — can no longer use `a`
                  // b is borrowed — still usable after

// Many pieces — use join (single allocation)
let parts = ["the", "quick", "brown", "fox"];
let sentence = parts.join(" ");  // "the quick brown fox"

// format! — clear, readable, allocates once
let (x, y, z) = ("foo", "bar", "baz");
let result = format!("{} {} {}", x, y, z);

// with_capacity + push_str — maximum efficiency
let total_len: usize = parts.iter().map(|s| s.len()).sum::<usize>() + parts.len();
let mut buf = String::with_capacity(total_len);
for (i, p) in parts.iter().enumerate() {
    if i > 0 { buf.push(' '); }
    buf.push_str(p);
}

// concat! — compile-time literal concatenation (no allocation)
let s = concat!("hello", ", ", "world");  // &str, evaluated at compile time

// Iterator approach — flat_map then collect
let s: String = parts.iter()
    .flat_map(|w| w.chars().chain(std::iter::once(' ')))
    .collect::<String>()
    .trim_end()
    .to_string();
```

## What This Unlocks

- **Log line building** — `with_capacity` + `push_str` for zero-reallocation log assembly in hot paths.
- **CSV generation** — `fields.join(",")` produces valid CSV rows in one allocation.
- **Template rendering** — `format!()` for readable, type-safe string construction.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Concatenate two | `a ^ b` (allocates) | `a + &b` (moves `a`, borrows `b`) |
| Join list | `String.concat sep list` | `slice.join(sep)` |
| Format into string | `Printf.sprintf "..."` | `format!("...")` |
| Buffer building | `Buffer.create` + `Buffer.add_string` | `String::with_capacity` + `push_str` |
| Compile-time concat | N/A | `concat!("a", "b")` → `&str` |
| `+=` in loop | O(n²) | O(n²) with `+`; use `push_str` for O(n) |
