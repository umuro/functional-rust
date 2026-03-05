📖 **[View on hightechmind.io →](https://hightechmind.io/rust/161-digit-parser)**

---

# 161: Digit Parser

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

A practical application: compose `satisfy`, `many1`, `map`, and `opt` to parse single digits, natural numbers, and signed integers.

## The Problem This Solves

Numbers are everywhere in text. Config values, JSON numbers, source code literals, protocol fields. Every time you need to parse a number, you face the same sub-problems: which characters are digits, how do you turn multiple digit characters into a numeric value, how do you handle the optional sign.

This example is less about new combinators and more about *putting it all together*. Everything from examples 153–159 comes together here into something practical: a `Parser<i64>` that correctly parses `"42"`, `"-17"`, and `"+100"`.

The implementation also shows a subtle point: the correct conversion from a `char` digit to its numeric value isn't `'3' - '0'` in the usual sense — in Rust, you cast both to `u32` first. Getting this right is one of those details that matters in practice.

## The Intuition

A number is a sequence of digits. "Sequence of digits" means `many1(satisfy(is_digit))` — which gives you a `Vec<char>`. But a `Vec<char>` of `['4', '2']` isn't the number 42 yet. You need to fold it: start with 0, for each digit character multiply the accumulator by 10 and add the digit's value.

Digit value: `'7'` as Unicode/ASCII has code point 55, and `'0'` has code point 48. So `'7' as u32 - '0' as u32` = 7. This is the standard trick for char-to-digit conversion.

A signed integer is an optional sign character followed by a natural number. `opt(satisfy(|c| c == '+' || c == '-', "sign"))` returns `Some('+')`, `Some('-')`, or `None`. Pattern-match on that to decide whether to negate.

## How It Works in Rust

**Single digit → `u32`:**
```rust
fn digit<'a>() -> Parser<'a, u32> {
    map(
        satisfy(|c| c.is_ascii_digit(), "digit"),  // parse one digit char
        |c| c as u32 - '0' as u32,                 // convert char to numeric value
    )
}
// digit()("5rest") = Ok((5, "rest"))
```

**Natural number (unsigned) → `u64`:**
```rust
fn natural<'a>() -> Parser<'a, u64> {
    map(
        many1(satisfy(|c| c.is_ascii_digit(), "digit")),  // Vec<char>: ['4','2']
        |digits| digits.iter().fold(0u64, |acc, &d| {
            acc * 10 + (d as u64 - '0' as u64)  // positional value: 0 → 4 → 42
        }),
    )
}
// natural()("42rest") = Ok((42, "rest"))
// natural()("abc")    = Err (many1 requires at least one digit)
```
`iter().fold(init, f)` replaces OCaml's `List.fold_left`. It starts with `0u64`, and for each digit char `d`, computes `acc * 10 + digit_value(d)`.

**Signed integer → `i64`:**
```rust
fn integer<'a>() -> Parser<'a, i64> {
    Box::new(|input: &'a str| {
        // opt returns Some('+'), Some('-'), or None
        let (sign, rest) = opt(satisfy(|c| c == '+' || c == '-', "sign"))(input)?;
        let (n, rem) = natural()(rest)?;
        let value = match sign {
            Some('-') => -(n as i64),  // negate for minus
            _         => n as i64,     // plus or absent: positive
        };
        Ok((value, rem))
    })
}
// integer()("42")   = Ok((42,  ""))
// integer()("-42")  = Ok((-42, ""))
// integer()("+42")  = Ok((42,  ""))
// integer()("abc")  = Err
```
`n as i64` is safe as long as `n` fits in `i64`. For production code you'd add a bounds check. The `_` arm handles both `Some('+')` and `None` — both mean positive.

## What This Unlocks

- **Configuration and data file parsing** — integer fields, port numbers, counts, indices all follow this pattern.
- **JSON number parsing** — a starting point (JSON numbers also have decimals and exponents, but the digit parsing is identical).
- **Protocol fields** — any protocol with numeric fields (count, length, version) uses exactly this pattern.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Digit-to-int | `Char.code c - Char.code '0'` | `c as u32 - '0' as u32` |
| Number types | `int` (GC-managed, platform-sized) | `u32`, `u64`, `i64` (explicit width, stack-allocated) |
| Fold over list | `List.fold_left (fun acc d -> ...) 0 digits` | `digits.iter().fold(0u64, \|acc, &d\| ...)` |
| Negation | `- n` | `-(n as i64)` (cast needed: `u64 → i64`) |
| Optional sign | `opt sign >>= fun s -> ...` | `opt(satisfy(...))(input)?` then `match sign` |
