📖 **[View on hightechmind.io →](https://hightechmind.io/rust/541-lifetime-elision)**

---

# 541: Lifetime Elision Rules

**Difficulty:** 3  **Level:** Intermediate

90% of Rust lifetimes are never written — the compiler infers them using three deterministic rules. Knowing the rules tells you exactly when you must write `'a` and when the compiler will figure it out.

## The Problem This Solves

Early Rust required every lifetime to be written explicitly. Every function signature involving references needed `<'a>` annotations. The community found that most lifetime annotations were *obvious* from the signature structure — so Rust introduced elision rules to infer them automatically.

Without knowing the rules, you can't tell whether a function signature is correct or just happens to compile. You also can't diagnose "missing lifetime specifier" errors — you won't know which cases require annotation.

## The Intuition

Lifetime elision is the compiler filling in lifetime annotations you didn't write. The three rules are applied in order to each function signature:

- **Rule 1**: Every input reference gets its own fresh lifetime. (`&str` becomes `&'a str`, another `&str` becomes `&'b str`)
- **Rule 2**: If there's exactly one input reference lifetime after Rule 1, all output references get that lifetime.
- **Rule 3**: If one of the inputs is `&self` or `&mut self`, all output references get self's lifetime.

If after applying all three rules, any output reference lifetime is still unknown, the compiler requires an explicit annotation.

## How It Works in Rust

**Rule 1 — one input, output unrelated (no elision needed):**

```rust
fn strlen(s: &str) -> usize { s.len() }
// Expands to: fn strlen<'a>(s: &'a str) -> usize
// No output reference — rule 1 fires but rule 2 doesn't need to
```

**Rule 2 — single input reference, output gets that lifetime:**

```rust
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
// Expands to: fn first_word<'a>(s: &'a str) -> &'a str
// Rule 1: s gets 'a. Rule 2: exactly one input lifetime → output gets 'a
```

**Rule 3 — `&self` method, output gets self's lifetime:**

```rust
struct Cache { data: Vec<String> }

impl Cache {
    fn get(&self, index: usize) -> Option<&str> {
        self.data.get(index).map(|s| s.as_str())
    }
    // Expands to: fn get<'a>(&'a self, index: usize) -> Option<&'a str>
    // Rule 3: &self present → output gets 'a (self's lifetime)
}
```

**When elision fails — must annotate:**

```rust
// Two input references, output reference, no self → rule 2 can't apply (two candidates)
fn longest(x: &str, y: &str) -> &str { ... }
// error: missing lifetime specifier
// Fix:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
```

**Rule 2 works with one input even if there are more:**

```rust
// Output comes from s, not prefix — must annotate even though one "main" input
fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    // 'a on s only; prefix gets its own lifetime but output is tied to s
    s.strip_prefix(prefix).unwrap_or(s)
}
```

**The `'_` placeholder — explicit elision:**

```rust
// '_ means "infer the lifetime here" — documents that a lifetime is present
fn get_first(v: &[i32]) -> Option<&'_ i32> { v.first() }
// Same as: fn get_first<'a>(v: &'a [i32]) -> Option<&'a i32>
```

## What This Unlocks

- **Readable signatures** — common functions like `first_word`, `trim`, `get` don't need cluttered annotations. The rules make them clean by default.
- **Diagnosing "missing lifetime specifier"** — you can immediately tell *why* an annotation is required: two input references with an output reference, and no `self`.
- **Confident API design** — choosing between `fn get(&self) -> &str` (rule 3: tied to self) and `fn get(&self) -> &'a str` (tied to the data) becomes a deliberate decision, not guesswork.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Reference annotations | No reference annotations — GC manages all | Lifetimes usually inferred via elision; explicit when ambiguous |
| Inference rules | Full Hindley-Milner type inference everywhere | Three elision rules for lifetimes; everything else explicit |
| Method return references | Always valid (GC) | Rule 3: `&self` methods return borrow tied to self by default |
| Single input functions | No concept | Rule 2: one input `&T` → output `&T` gets same lifetime |
| `'_` syntax | N/A | Explicit elision marker — "infer this lifetime" — useful in type positions |
