📖 **[View on hightechmind.io →](https://hightechmind.io/rust/557-lifetime-output-lifetime)**

---

# 557: Output Lifetimes in Traits

**Difficulty:** 4  **Level:** Intermediate-Advanced

Specify which input a trait method's return value borrows from — so the compiler can track validity across trait boundaries.

## The Problem This Solves

When a trait method returns a reference, the compiler needs to know which input it borrows from. For simple cases (one input, `&self`), lifetime elision handles it. But when a method has both `&self` and a `&'a str` input, and the output borrows from the *string argument* rather than `self`, you must be explicit — elision gets it wrong.

This pattern appears constantly in extractors, parsers, adapters, and any abstraction over "give me a view into this data." Without explicit output lifetimes in traits, you either over-restrict callers (requiring both to live equally long) or end up copying data unnecessarily.

The lifetime parameter on the trait itself (`trait Extractor<'a>`) vs on the method is the key design decision, and each choice has different tradeoffs for implementors.

## The Intuition

Think of a lifetime annotation on the output as a label: "this return value was carved out of *that* input." When you write `fn extract(&self, source: &'a str) -> Vec<&'a str>`, you're saying "the returned slices point into `source`, not into `self`." The caller can then drop `self` while still using the returned slices — as long as `source` lives.

Putting `'a` on the *trait* (`trait Extractor<'a>`) instead of on the method means the lifetime is fixed when you implement the trait, not when you call the method. This is useful for implementors that need to store `'a`-lived data.

## How It Works in Rust

**Lifetime on the trait** — fixed at impl time:
```rust
trait Extractor<'a> {
    type Output;
    fn extract(&self, source: &'a str) -> Self::Output;
}

impl<'a> Extractor<'a> for WordExtractor {
    type Output = Vec<&'a str>;
    fn extract(&self, source: &'a str) -> Vec<&'a str> {
        source.split_whitespace().collect()
    }
}
```
The associated type `Output` is generic over `'a`, so callers get `Vec<&'a str>` — slices of the input string.

**Using the trait generically** — the bound ties input and output lifetimes:
```rust
fn extract_and_print<'a>(
    extractor: &impl Extractor<'a, Output = Vec<&'a str>>,
    text: &'a str,
) {
    let words = extractor.extract(text);
    println!("{:?}", words);
}
```

**`&self` lifetime in output** — elision works here:
```rust
trait AsRef2 {
    fn as_str(&self) -> &str;  // implicitly: fn as_str<'a>(&'a self) -> &'a str
}
```
Elision rule 3: when `&self` is the only input, the output borrows from `self`.

**Proving the output borrows from the argument, not `self`:**
```rust
let text = String::from("hello world rust");
let extractor = WordExtractor;
let words = extractor.extract(&text);
// extractor can be dropped here — words only borrow from text
drop(extractor);
println!("{:?}", &words[..2]);  // still valid
```

## What This Unlocks

- **Zero-copy trait abstractions** — extractors, parsers, splitters that return slices of their input without allocation.
- **Lifetime-polymorphic trait objects** — by putting `'a` on the trait, implementors can vary the output lifetime.
- **Documenting borrow sources** — explicit lifetime annotations in trait signatures are machine-checked documentation of "this output came from that input."

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Trait with lifetime | N/A (no lifetimes) | `trait Foo<'a>` or method-level `'a` |
| Return borrows from argument | GC tracks implicitly | Explicit `'a` on both input and output |
| Associated type with lifetime | N/A | `type Output;` resolved at `impl` with `'a` in scope |
| Elision for `&self` output | N/A | Rule 3: output inherits `self`'s lifetime automatically |
