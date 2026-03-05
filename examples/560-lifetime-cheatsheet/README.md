📖 **[View on hightechmind.io →](https://hightechmind.io/rust/560-lifetime-cheatsheet)**

---

# 560: Lifetime Annotation Cheatsheet

**Difficulty:** 3  **Level:** Intermediate

All common lifetime annotation patterns in one place. Use this as a reference when you need the syntax and can't remember which form goes where.

## The Problem This Solves

Lifetime annotations appear in many different syntactic positions in Rust — function signatures, struct definitions, `impl` blocks, trait bounds, closures, and type positions. Each position has its own rules and idioms. Having them all in one place prevents the "I know what I need but forgot the syntax" slowdown.

This cheatsheet covers the 90% of cases you'll encounter in real Rust code, with brief notes on when to use each form.

## The Intuition

Lifetime annotations follow a consistent pattern: declare the lifetime parameter in `<>` at the item level, then use it on references within that item. The label `'a` is just a name — choose any name, though `'a`, `'b`, `'static`, and `'_` are idiomatic.

- `&'a T` — reference valid for at least `'a`
- `'a: 'b` — `'a` outlives `'b`
- `for<'a>` — universally quantified over all lifetimes
- `'_` — infer the lifetime (explicit elision marker)
- `'static` — valid for the entire program

## How It Works in Rust

**Function signatures:**

```rust
// Most common: elided — compiler infers
fn first_word(s: &str) -> &str { s.split_whitespace().next().unwrap_or("") }

// Explicit: two inputs, one output — must annotate which source
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// Independent lifetimes: output from x only, y doesn't constrain result
fn first_of<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str { x }

// Static: return lives forever (embedded in binary)
fn greeting() -> &'static str { "Hello!" }

// Anonymous: explicit elision — '_ means "infer it"
fn get_first(v: &[i32]) -> Option<&'_ i32> { v.first() }
```

**Struct definitions:**

```rust
// Struct with one borrowed field
struct StrWrapper<'a> { value: &'a str }

// Multiple independent borrowed fields
struct PairRef<'a, 'b> { first: &'a str, second: &'b str }

// Generic with lifetime
struct Container<'a, T> { items: &'a [T], label: &'a str }
```

**impl blocks:**

```rust
impl<'a> StrWrapper<'a> {
    fn new(s: &'a str) -> Self { StrWrapper { value: s } }
    
    // Rule 3 elision: &self method → return tied to self
    fn get(&self) -> &str { self.value }
    
    // Explicit: tied to 'a (data lifetime), not self's borrow
    fn get_explicit(&self) -> &'a str { self.value }
}
```

**Trait bounds and where clauses:**

```rust
// T must not contain borrows shorter than 'static
fn store<T: 'static>(value: T) { /* ... */ }

// T must outlive 'a (T can contain refs, but they must be >= 'a)
fn use_ref<'a, T: 'a>(r: &'a T) -> &'a T { r }

// 'a outlives 'b
fn constrained<'a: 'b, 'b>(x: &'a str, _y: &'b str) -> &'b str { x }
```

**Higher-ranked trait bounds:**

```rust
// F must work for ANY lifetime — not just one specific 'a
fn apply<F>(s: &str, f: F) -> usize
where F: for<'a> Fn(&'a str) -> usize
{ f(s) }
```

**Closures:**

```rust
// Closure captures &'a str — valid for 'a
fn make_logger<'a>(prefix: &'a str) -> impl Fn(&str) -> String + 'a {
    move |s| format!("{}: {}", prefix, s)
}
```

**dyn Trait:**

```rust
// Default: 'static (owns all data)
fn store_renderer(r: Box<dyn Renderer>) { /* ... */ }

// Explicit lifetime: can borrow from 'a
fn use_renderer<'a>(r: Box<dyn Renderer + 'a>, data: &'a str) { /* ... */ }
```

## What This Unlocks

- **Fast syntax lookup** — all forms in one place means you spend time writing code, not searching docs.
- **Pattern recognition** — seeing all forms together reveals the consistent grammar: declare `'a` in `<>`, use `&'a` on references, add bounds where needed.
- **Confident API design** — you'll know when to use `&'a str` vs `&str` (elided) vs `&'static str` and what each choice communicates to the caller.

## Key Differences

| Annotation | Meaning | When to use |
|------------|---------|-------------|
| `&str` (elided) | Compiler infers via elision rules | 90% of cases — single input, `&self` method |
| `&'a str` | Explicitly named lifetime | Multiple refs where output source is ambiguous |
| `&'static str` | Lives for entire program | String literals, `static` variables |
| `&'_ str` | Explicit elision marker | Type positions where you want to be explicit |
| `T: 'static` | T owns its data | Thread spawning, global storage, `Box<dyn Trait>` |
| `T: 'a` | T's refs live >= 'a | Generic functions that store refs |
| `'a: 'b` | 'a outlives 'b | Lifetime ordering constraints |
| `for<'a>` | Works for any lifetime | Generic callbacks, trait objects with reference arguments |
