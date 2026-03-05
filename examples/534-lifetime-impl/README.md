# 534: Lifetimes in impl Blocks

**Difficulty:** 3  **Level:** Intermediate

The `impl<'a>` block declares the lifetime that the struct was parameterized with. Methods can then return references tied to the *data's* lifetime — not to `self`'s borrow.

## The Problem This Solves

Here's the subtle issue: when a method returns a reference, it can come from two different sources — either from `self` (the temporary borrow of the struct), or from the data the struct *wraps* (the `'a` lifetime). These are different scopes and the distinction matters.

```rust
struct View<'a, T> {
    data: &'a [T],
}

impl<'a, T> View<'a, T> {
    // WRONG idea: fn get(&self) -> Option<&T>
    // This ties the return to &self, not to 'a.
    // The caller drops the View, and the returned reference becomes invalid!
    
    // RIGHT: fn get(&self) -> Option<&'a T>
    // Return is tied to 'a — the underlying data's lifetime.
    // Caller can drop the View and still use the returned element.
}
```

If you get this wrong, you either unnecessarily restrict the caller (forcing them to keep the view alive), or you create a use-after-free if the compiler doesn't catch it. Explicit `'a` on the return type is the fix.

## The Intuition

Think of a `View<'a, T>` as a telescope pointed at an array. The telescope (`View`) can be dropped; the stars (`T` data) stay in the sky. When `get()` returns `&'a T`, it's returning a pointer to the stars — not to the telescope. The caller can put the telescope away and still look at what they found.

The `impl<'a>` syntax just brings the `'a` parameter into scope for all the methods. It's not declaring a new lifetime — it's saying "I'm implementing methods for the `View` that was already declared with `'a`."

## How It Works in Rust

**The impl block header:**

```rust
struct View<'a, T> {
    data: &'a [T],
}

// impl<'a, T>: bring 'a into scope for all methods
// View<'a, T>: this is the specific type we're implementing for
impl<'a, T> View<'a, T> {
    fn new(data: &'a [T]) -> Self {
        View { data }
    }
    
    // Returns &'a T — tied to the data's lifetime, NOT self's borrow
    fn get(&self, index: usize) -> Option<&'a T> {
        self.data.get(index)
    }
    
    // Sub-view has the same 'a — shares the same data lifetime
    fn slice(&self, start: usize, end: usize) -> View<'a, T> {
        View { data: &self.data[start..end] }
    }
}
```

**Proof the distinction matters:**

```rust
let data = vec![10, 20, 30, 40, 50];
let view = View::new(&data);

let elem = view.get(1);   // returns Option<&'a T>
drop(view);                // view is gone
println!("{:?}", elem);   // fine! elem tied to data, not view
```

**Method returning `'a` from a field:**

```rust
struct Formatter<'a> {
    prefix: &'a str,
}

impl<'a> Formatter<'a> {
    // Returns &'a str — caller can drop Formatter, still has the prefix
    fn get_prefix(&self) -> &'a str {
        self.prefix // returning the 'a field directly
    }
    
    // Contrast: if we returned &str (elided, tied to &self),
    // the result would be invalid after Formatter drops
}
```

## What This Unlocks

- **Drop the wrapper, keep the element** — iterators, views, and cursor types can return `&'a T` pointing directly into the underlying data. The view itself is disposable.
- **Composable views** — a `slice()` method that returns `View<'a, T>` shares the same lifetime as the original, making it safe to chain view operations.
- **Explict lifetime documentation** — `fn get_prefix(&self) -> &'a str` instantly tells readers "this prefix's validity is determined by construction, not by the method call."

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Method return references | GC manages all lifetimes — no annotations | Must specify if return is tied to `'a` (data) or implicitly to `&self` |
| Drop wrapper, keep element | Automatic — GC keeps element alive | Explicit `&'a T` return type makes this safe without GC |
| `impl` block parameters | No equivalent — methods don't have lifetime params | `impl<'a> Foo<'a>` brings the struct's `'a` into scope |
| View/iterator types | Typically return new values | Common pattern: view that returns `&'a T` — borrow outlives the view itself |
| Lifetime elision in methods | N/A | Rule 3: `&self` method — output implicitly tied to self; must override for `'a` |
