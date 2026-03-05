# 537: Lifetime Coercion and Subtyping

**Difficulty:** 4  **Level:** Advanced

Longer lifetimes are subtypes of shorter ones. A `&'long T` can be used anywhere a `&'short T` is expected — but not the reverse.

## The Problem This Solves

Subtyping for lifetimes is what makes `&'static str` usable everywhere a `&str` is expected. Without it, every function that takes `&str` would need a separate overload for `&'static str`. Instead, Rust automatically *coerces* longer lifetimes to shorter ones at the use site.

The confusion is that this feels backwards from what you'd expect: "longer lifetime" is the *more specific* type. A `&'static str` is a stronger guarantee than a `&'a str` — it commits to living forever. Being a subtype means it can fill in wherever less is required.

Understanding this subtyping relationship also explains why `&mut T` can't be coerced the same way — mutable references are invariant, not covariant, and mixing them would break safety.

## The Intuition

Lifetime subtyping: `'long: 'short` means "'long outlives 'short," which makes `'long` the *subtype*. You can always use a subtype where a supertype is expected.

Think of it like a precision hierarchy: `'static` is maximally precise (lives forever). Any `'a` is less precise (lives for some scope). A precise tool works wherever a less precise one would — you can always use a laser where a flashlight would do.

Coercion is *narrowing* — the compiler silently shrinks the lifetime annotation to match the expected type. No data moves; it's purely a compile-time label adjustment.

## How It Works in Rust

**`&'static` coerces to any lifetime:**

```rust
let literal: &'static str = "I live forever";

// Coercion: &'static str → &'shorter str
// Happens automatically — no cast needed
let shorter: &str = literal;
fn accepts_any(s: &str) { println!("{}", s); }
accepts_any(literal);  // 'static coerces to whatever 'a the function expects
```

**Explicit coercion in storage:**

```rust
fn store_long<'long, 'short>(
    storage: &mut Vec<&'short str>,
    item: &'long str,
) where 'long: 'short {  // 'long outlives 'short — coercion is valid
    storage.push(item);  // &'long str coerced to &'short str
}

let mut storage: Vec<&str> = Vec::new();
let permanent = "permanent";  // &'static str
store_long(&mut storage, permanent);  // &'static coerces to &'short
```

**Why `&mut T` is invariant — the danger:**

```rust
// If &mut T were covariant, this would be possible:
fn bad_coerce<'long, 'short>(r: &mut &'long str, s: &'short str) {
    // *r = s; // If allowed: r now holds &'short str
    // But r was promised to hold &'long str!
    // After 'short expires, *r would be a dangling reference
}
// Rust forbids this — &mut T is invariant to prevent it
```

**Cache demonstrating lifetime storage:**

```rust
struct Cache<'a> {
    entries: Vec<&'a str>,
}

impl<'a> Cache<'a> {
    // Only accepts refs that live at least as long as 'a
    // &'static str satisfies this (coercion)
    // &'shorter str would fail
    fn insert(&mut self, entry: &'a str) {
        self.entries.push(entry);
    }
}
```

## What This Unlocks

- **`&'static str` is universally usable** — pass string literals to any function taking `&str` without type errors. The coercion is automatic.
- **Generic containers for mixed-lifetime data** — a `Vec<&'a str>` can hold both `&'static str` literals and shorter-lived slices, as long as all satisfy `'a`.
- **Understanding compiler errors** — when you see "lifetime may not live long enough," you know you're trying to *widen* a lifetime (use shorter where longer is expected), which is invalid.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Reference validity | GC guarantees all live references are valid | Subtyping: `'long <: 'short` means longer refs can fill shorter slots |
| String literal usage | String literals work anywhere | `&'static str` coerces to `&'a str` — automatic at call sites |
| Mutable reference aliasing | GC allows multiple mutable refs (with care) | `&mut T` is invariant — no coercion, prevents aliasing bugs |
| Subtype direction | Subtypes typically extend supertypes | `'long: 'short` makes longer lifetime the *subtype* — more specific = subtype |
| Type coercions | Implicit where safe | Lifetime coercions (shortening) are implicit; widening is a compile error |
