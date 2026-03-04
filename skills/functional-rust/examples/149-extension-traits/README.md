# 149: Extension Traits

**Difficulty:** 3  **Level:** Intermediate

Add new methods to types you don't own — without modifying them.

## The Problem This Solves

You're using a library type — `String`, `Iterator`, `Option` — and you wish it had a method it doesn't have. In most languages you'd subclass or monkey-patch. Rust's orphan rules prevent adding trait impls arbitrarily, but there's a clean pattern: the extension trait.

Define a new trait in *your* crate with the methods you want. Provide a blanket `impl` for all types that meet the requirements. Now every `T: Ord` gains your `clamp_to` and `is_between` methods — without touching the standard library.

This is how iterator adapter libraries work. It's how test utilities add `.assert_eq_sorted()` to iterators. It's structured and composable, unlike monkey-patching.

## The Intuition

An extension trait is just a regular trait with:
1. **Default method implementations** derived from existing capabilities
2. **A blanket `impl`** that gives those methods to every qualifying type automatically

```rust
pub trait OrdExt: Ord + Sized + Clone {
    fn clamp_to(&self, lo: &Self, hi: &Self) -> Self { ... }
}
// Every T: Ord + Clone gets clamp_to for free:
impl<T: Ord + Clone> OrdExt for T {}
```

The blanket impl says: "for any type `T` that already knows how to compare itself (`Ord`) and copy itself (`Clone`), give it these extra methods." You wrote the methods once, they work everywhere.

## How It Works in Rust

```rust
// Extension trait for any Ord + Clone type
pub trait OrdExt: Ord + Sized + Clone {
    fn clamp_to(&self, lo: &Self, hi: &Self) -> Self {
        if self < lo { lo.clone() }
        else if self > hi { hi.clone() }
        else { self.clone() }
    }

    fn is_between(&self, lo: &Self, hi: &Self) -> bool {
        self >= lo && self <= hi
    }
}

// Blanket impl — automatically applies to every T: Ord + Clone
impl<T: Ord + Clone> OrdExt for T {}

// Now all of these work:
15_i32.clamp_to(&0, &10);          // 10
"banana".clamp_to(&"apple", &"cherry");  // "banana"
```

Extension trait for iterators:

```rust
pub trait IterExt: Iterator + Sized {
    fn sorted(self) -> Vec<Self::Item> where Self::Item: Ord {
        let mut v: Vec<_> = self.collect();
        v.sort();
        v
    }

    fn join_display(self, sep: &str) -> String where Self::Item: std::fmt::Display {
        self.map(|x| x.to_string()).collect::<Vec<_>>().join(sep)
    }
}

impl<I: Iterator> IterExt for I {}  // blanket impl

// Every iterator gets these:
[3, 1, 4, 1, 5].iter().copied().sorted()   // [1, 1, 3, 4, 5]
[1, 2, 3].iter().join_display(", ")         // "1, 2, 3"
```

Extension trait on `str` (a foreign type):

```rust
pub trait StrExt {
    fn title_case(&self) -> String;
    fn is_palindrome(&self) -> bool;
}

impl StrExt for str {  // str is foreign, but our trait is local — allowed!
    fn title_case(&self) -> String { /* ... */ }
    fn is_palindrome(&self) -> bool { /* ... */ }
}

"hello world".title_case()   // "Hello World"
"racecar".is_palindrome()    // true
```

## What This Unlocks

- **Ergonomic utility layers** — add `.sorted()`, `.join_display()`, `.chunks_of()` to iterators in your project
- **Domain-specific extensions** — `ValidationExt` for `String` adds `.is_email()`, `.is_url()` without forking the type
- **Test helpers** — extension traits that add assertion methods to types only in test builds

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mechanism | Functor: `OrdExt(O: ORD)` produces a module with derived ops | Blanket `impl<T: Ord> OrdExt for T {}` |
| Activation | Explicit: `module IntOrdExt = OrdExt(Int)` | Automatic: import the trait, all qualifying types gain it |
| Foreign types | Functors work regardless of where `T` is from | Extension trait on foreign type works as long as your trait is local |
| Multiple instances | Multiple explicit module applications | One blanket impl (but can be more specific) |
| Discoverability | Explicit module application required | Trait must be in scope (`use crate::IterExt`) |
