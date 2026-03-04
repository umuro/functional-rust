# 106: Lifetime Elision

**Difficulty:** 2  **Level:** Intermediate

Three simple rules let the compiler infer lifetimes automatically in the most common cases — so you only write annotations when the situation is ambiguous.

## The Problem This Solves

If every function that takes or returns references needed explicit lifetime annotations, Rust would be extremely verbose. Early Rust (pre-1.0) actually required them everywhere. `fn first_word<'a>(s: &'a str) -> &'a str` for what should just be `fn first_word(s: &str) -> &str`. The annotations added noise without adding information — the relationship was obvious.

The lifetime elision rules were introduced to codify the patterns that appear in almost all real Rust code. They're not magic: the compiler applies three deterministic rules, and if those rules produce an unambiguous answer, the lifetime is inferred. If they don't, the compiler asks you to be explicit. This means you write annotations only when the relationship between input and output lifetimes is genuinely non-obvious.

Understanding the rules also helps you read other people's Rust code: when you see `fn get_name(&self) -> &str`, you can mentally expand it to `fn get_name<'a>(&'a self) -> &'a str` and know exactly what it means.

## The Intuition

The compiler has three simple rules for guessing lifetimes; when the rules give a clear answer, you write nothing — only write annotations when you're doing something the rules can't figure out on their own.

## How It Works in Rust

The three elision rules:
1. **Each reference parameter gets its own lifetime.**
2. **If there's exactly one input lifetime, all output lifetimes get that lifetime.**
3. **If one of the inputs is `&self` or `&mut self`, all output lifetimes get `self`'s lifetime.**

```rust
// Rule 2: one input → output gets its lifetime
fn first_word(s: &str) -> &str {
    // Expanded: fn first_word<'a>(s: &'a str) -> &'a str
    s.split_whitespace().next().unwrap_or("")
}

// Rule 3: &self method → output gets self's lifetime
struct Config {
    name: String,
}
impl Config {
    fn name(&self) -> &str {
        // Expanded: fn name<'a>(&'a self) -> &'a str
        &self.name
    }
}

// Rules don't resolve this — must annotate explicitly
// Which input does the return value come from?
fn longest(s1: &str, s2: &str) -> &str {  // ERROR: ambiguous
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Fixed: explicit annotation
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Rule 1: multiple inputs, each gets its own lifetime (doesn't help resolve output)
fn first_of_two<'a>(s1: &'a str, _s2: &str) -> &'a str {
    // Explicitly says: return comes from s1, not s2
    s1
}
```

## What This Unlocks

- **Less boilerplate** — the common patterns (single-argument functions, methods on `&self`) require zero annotations, keeping code readable.
- **Explicit when it matters** — when the rules can't infer, the compiler asks you to annotate, ensuring non-obvious reference relationships are visible in function signatures.
- **Mental model building** — understanding elision rules lets you read any Rust code confidently: you can always mentally expand elided lifetimes to understand what the compiler sees.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Reference lifetimes | Not tracked (GC handles) | Inferred via elision rules or written explicitly |
| Common function signatures | No annotations needed | No annotations needed (elided) |
| Ambiguous cases | Not possible (GC) | Compiler asks for explicit annotation |
| Learning curve | No lifetime concept | Elision rules reduce annotation burden significantly |
| Documentation value | N/A | Explicit `'a` annotations document reference relationships |
