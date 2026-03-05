# 543: Lifetimes in dyn Trait

**Difficulty:** 4  **Level:** Advanced

`Box<dyn Trait>` defaults to `Box<dyn Trait + 'static>`. If you want to store a trait object that borrows, you must say so explicitly with a lifetime bound.

## The Problem This Solves

This is a common beginner stumble: you create a struct implementing a trait, but it borrows from some data. Then you try to box it:

```rust
struct BorrowedRenderer<'a> {
    content: &'a str,
}
impl Renderer for BorrowedRenderer<'_> { ... }

let text = String::from("hello");
let r = BorrowedRenderer { content: &text };
let boxed: Box<dyn Renderer> = Box::new(r);
// ERROR: `BorrowedRenderer<'_>` doesn't satisfy `'static`
// The default Box<dyn Renderer + 'static> requires the trait object to own all its data
```

The fix is either to use an owned type (satisfying `'static`), or to add a lifetime bound to the trait object: `Box<dyn Renderer + 'a>`.

## The Intuition

`dyn Trait` erases the concrete type but *not* the lifetime information. The compiler needs to know: "can this trait object outlive some scope?" Without a bound, it assumes you want the most flexible option — `'static`, meaning "no borrowed data."

When you write `Box<dyn Trait + 'a>`, you're saying: "this trait object may borrow from a source that lives for `'a`. It can only be used as long as `'a` is alive."

The `+ 'a` on `dyn Trait` is a *lifetime bound*, not a lifetime parameter of the trait itself. It constrains the trait *object's* validity, not the trait's interface.

## How It Works in Rust

**The default — `'static` required:**

```rust
// Box<dyn Renderer> is Box<dyn Renderer + 'static>
fn store_renderer(r: Box<dyn Renderer>) {
    // r must contain no borrowed data with limited lifetime
}

// OK: OwnedRenderer satisfies 'static (owns its String)
store_renderer(Box::new(OwnedRenderer { content: "hello".to_string() }));

// FAILS: BorrowedRenderer borrows &str — doesn't satisfy 'static
let text = String::from("hello");
// store_renderer(Box::new(BorrowedRenderer { content: &text })); // ERROR
```

**Explicit lifetime bound — allows borrowed trait objects:**

```rust
// 'a bound: trait object can borrow from data with lifetime 'a
struct Screen<'a> {
    renderer: Box<dyn Renderer + 'a>,  // can borrow — not required to be 'static
}

impl<'a> Screen<'a> {
    fn new(r: impl Renderer + 'a) -> Self {
        Screen { renderer: Box::new(r) }
    }
    fn draw(&self) {
        println!("{}", self.renderer.render());
    }
}

let text = String::from("borrowed content");
let screen = Screen::new(BorrowedRenderer { content: &text });
screen.draw(); // works — BorrowedRenderer satisfies Renderer + '_ (borrows text)
```

**`&dyn Trait` vs `Box<dyn Trait>`:**

```rust
// &dyn Trait already implies a lifetime — no explicit + 'a needed for references
fn use_borrowed(r: &dyn Renderer) {
    println!("{}", r.render()); // r is a reference — lifetime is the borrow's lifetime
}

let r = BorrowedRenderer { content: "hello" };
use_borrowed(&r); // works — reference carries implicit lifetime
```

**Vec of mixed trait objects:**

```rust
let data = String::from("shared");
let renderers: Vec<Box<dyn Renderer + '_>> = vec![
    Box::new(BorrowedRenderer { content: &data }),
    Box::new(OwnedRenderer { content: "owned".to_string() }),
    // 'static satisfies '_, owned satisfies '_ (no restricted borrows)
];
```

## What This Unlocks

- **Zero-allocation trait object collections** — a `Vec<Box<dyn Trait + 'a>>` can hold objects that borrow from an arena or input buffer. No need to clone data just to satisfy `'static`.
- **Temporary strategy objects** — a `dyn Visitor + '_` can borrow configuration state for one pass over a tree, then be discarded. Clean, zero-cost.
- **Plugin systems with mixed ownership** — some plugins own their state (`'static`), others borrow shared config (`+ 'a`). The `+ 'a` bound handles both.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Dynamic dispatch | First-class values + GC — no lifetime concern | `dyn Trait` needs a lifetime bound — defaults to `'static` |
| Heterogeneous collections | `('a list)` or similar — GC handles validity | `Vec<Box<dyn Trait + 'a>>` — compiler enforces all elements valid for `'a` |
| Interface with borrowed state | GC manages — no distinction | `dyn Trait + 'a` explicitly declares the borrow scope of the trait object |
| Storing callbacks | First-class functions, closures — GC | `Box<dyn Fn() + 'static>` for stored closures; `Box<dyn Fn() + 'a>` if they capture borrowed data |
| `'static` requirement | No equivalent — GC prevents dangling | `Box<dyn Trait>` defaults to `'static`; must relax with `+ 'a` for borrowed objects |
