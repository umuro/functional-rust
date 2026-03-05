📖 **[View on hightechmind.io →](https://hightechmind.io/rust/569-pattern-range)**

---

# 569: Range Patterns: 1..=10

**Difficulty:** 2  **Level:** Beginner

Match integers and characters against inclusive ranges — replace branching chains with clean interval dispatch.

## The Problem This Solves

Tax brackets. Grade cutoffs. Character classification. Age groups. These all share the same shape: a numeric value falls into one of several non-overlapping ranges, and each range gets different treatment. Without range patterns, you write an if/else chain: `if score >= 90 { 'A' } else if score >= 80 { 'B' } ...`. That works, but it's easy to introduce gaps or overlaps — a value of exactly 80 could be unintentionally handled by the wrong branch if you flip a comparison.

The deeper issue is readability: the intent is "90 through 100 maps to A." The code says "greater than or equal to 90, and we'll check the upper bound implicitly because of arm ordering." Range patterns make the bounds explicit and the match compiler checks for gaps.

## The Intuition

Range patterns in `match` work exactly like ranges in everyday math notation, just spelled with `..=`. `90..=100` means "every integer from 90 to 100, inclusive." The `..=` is always inclusive in patterns — there is no exclusive range pattern.

OCaml doesn't have native range patterns; it simulates them with guards (`when n >= 90`). Guards are fine but miss one thing: the compiler can't prove exhaustiveness with guards. With Rust range patterns, the compiler can often verify coverage. Write `i32::MIN..=-1 | 0 | 1..=i32::MAX` and Rust knows you've covered every integer.

Range patterns also work on `char`, which is a clean win for character classification — `'a'..='z'`, `'A'..='Z'`, `'0'..='9'` cover exactly the right characters.

## How It Works in Rust

```rust
// Grade bands — inclusive on both ends
fn grade(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89  => 'B',
        70..=79  => 'C',
        60..=69  => 'D',
        _        => 'F',   // 0..=59
    }
}

// char ranges — works for any Unicode scalar value
fn classify_char(c: char) -> &'static str {
    match c {
        'A'..='Z' => "upper",
        'a'..='z' => "lower",
        '0'..='9' => "digit",
        _         => "other",
    }
}

// Tax brackets — use underscores for readability
fn tax_rate(income: u32) -> f64 {
    match income {
        0..=10_000          => 0.10,
        10_001..=40_000     => 0.12,
        40_001..=85_000     => 0.22,
        85_001..=163_300    => 0.24,
        _                   => 0.32,
    }
}

// Full coverage without _ — compiler verifies completeness
fn sign(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0             => "zero",
        1..=i32::MAX  => "positive",
    }
}
```

## What This Unlocks

- **Explicit interval dispatch** — upper and lower bounds are right there in the pattern; no implicit ordering dependency.
- **Character classification** — `'a'..='z'` is more readable and more correct than ASCII arithmetic.
- **Compiler-assisted exhaustiveness** — combine ranges with specific values to prove all integers are covered.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Syntax | Guards: `when n >= 90 && n <= 100` | `90..=100` pattern |
| Char range | `when Char.code c >= 65 && ...` | `'A'..='Z'` |
| Exhaustiveness | Compiler can't verify guard ranges | Can verify with full range coverage |
| Exclusive range | N/A | `..` (not valid in patterns — only `..=`) |
| Readability | Verbose numeric comparisons | Literal ranges matching intent |
