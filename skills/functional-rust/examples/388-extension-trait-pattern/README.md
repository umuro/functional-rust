# 388: Extension Trait Pattern

**Difficulty:** 2  **Level:** Intermediate

Add methods to types you don't own — the Rust idiom for safe, zero-cost "monkey-patching."

## The Problem This Solves

You're using `str`, `Vec<T>`, or some third-party type and you keep writing the same helper functions: `word_count(&s)`, `is_palindrome(&s)`, `chunk_average(&v)`. They work, but calling them as free functions is awkward — it breaks method chaining, and IDE autocomplete won't suggest them. You want `s.word_count()`, not `word_count(&s)`.

The obvious fix — add methods to `str` directly — is impossible. You don't own `str`. The orphan rules prevent you from implementing standard traits on foreign types you don't own (more on that in example 399). You're stuck... unless you define your own trait.

The extension trait pattern turns this limitation into a feature: define a trait with your extra methods, implement it for the foreign type, and import the trait where you need it. The methods appear on the type as if you owned it. The Rust standard library does this heavily — `itertools`, `rayon`, `byteorder` — all extend foreign types this way.

## The Intuition

An extension trait is a regular trait that exists *only* to add methods to an existing type. You never use the trait abstractly in a `<T: ExtTrait>` bound — you just import it and call the methods. It's a convention, not a language feature, but it's idiomatic and universally understood.

The pattern: define `trait StrExt`, implement `impl StrExt for str`, then `use my_crate::StrExt`. Now every `&str` has your methods. Behind the scenes it's static dispatch — zero overhead, inlined by the compiler exactly like calling a free function.

## How It Works in Rust

```rust
// Step 1: Define the trait with your extra methods
trait StrExt {
    fn word_count(&self) -> usize;
    fn is_palindrome(&self) -> bool;
    fn truncate_with_ellipsis(&self, max_len: usize) -> String;
}

// Step 2: Implement it for the foreign type
impl StrExt for str {
    fn word_count(&self) -> usize {
        self.split_whitespace().count()
    }

    fn is_palindrome(&self) -> bool {
        let chars: Vec<char> = self.chars().collect();
        chars.iter().rev().eq(chars.iter())
    }

    fn truncate_with_ellipsis(&self, max_len: usize) -> String {
        if self.len() <= max_len {
            self.to_string()
        } else {
            format!("{}…", &self[..max_len.saturating_sub(1)])
        }
    }
}

// Step 3: Use it — just call the methods directly
fn main() {
    println!("{}", "hello world foo".word_count());   // 3
    println!("{}", "racecar".is_palindrome());         // true
    println!("{}", "Hello, World!".truncate_with_ellipsis(8)); // Hello,…
}
```

Extension traits also work on generic types:
```rust
trait VecExt<T> {
    fn second(&self) -> Option<&T>;
}

impl<T> VecExt<T> for Vec<T> {
    fn second(&self) -> Option<&T> { self.get(1) }
}
```

The trait must be in scope (`use crate::StrExt`) for the methods to be visible. This is intentional — it prevents accidental method shadowing.

## What This Unlocks

- **Ergonomic utilities on stdlib types** — add domain-specific helpers to `String`, `Vec`, `Iterator`, `Path`, etc. without forking the type.
- **Crate-level extension APIs** — crates like `itertools` export extension traits that supercharge `Iterator` with 50+ extra methods, all zero-cost.
- **Method chaining** — transform `process(validate(parse(input)))` into `input.parse().validate().process()` by extending foreign types.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Adding methods to foreign types | Separate module (`StringExt.word_count s`) — called as function | Extension trait — called as method directly on value |
| Opt-in visibility | Module must be opened/imported | Trait must be `use`-d to bring methods into scope |
| Overhead | None (function call) | None (static dispatch, same as a free function) |
| Naming convention | `StringExt`, `ListUtils` modules | `StrExt`, `IteratorExt`, `PathExt` traits — `Ext` suffix is idiomatic |
