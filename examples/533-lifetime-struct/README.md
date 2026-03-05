📖 **[View on hightechmind.io →](https://hightechmind.io/rust/533-lifetime-struct)**

---

# 533: Lifetimes in Structs

**Difficulty:** 3  **Level:** Intermediate

When a struct holds a reference, it must declare a lifetime — because the struct can't be valid longer than the data it borrows.

## The Problem This Solves

Suppose you build a view struct that borrows from some source string. Without a lifetime, the compiler has no way to prevent this:

```rust
struct ParseResult {
    value: &str,     // error: missing lifetime specifier
    remaining: &str,
}
```

Even if the compiler allowed it, the danger is real. If you create a `ParseResult` that borrows from a `String`, then drop the `String`, the struct's pointers now point at freed memory. In C this is a use-after-free. In Rust, the compiler prevents the struct from compiling unless you prove the lifetime relationship.

The lifetime annotation on the struct is a *contract*: "this struct cannot outlive the data it borrows from." The compiler enforces it everywhere the struct is used.

## The Intuition

A struct with `<'a>` is saying: "I have a borrowing relationship, and `'a` names how long that relationship must stay valid." The struct cannot outlive `'a`. When the source data drops, any attempt to use the struct becomes a compile error.

This is the difference between a *view* and an *owner*. A view (struct with `&'a T`) is valid only while the source is alive. An owner (struct with `String`, `Vec`, etc.) lives on its own. Lifetime annotations on structs mark them as views.

## How It Works in Rust

**The annotation:**

```rust
// 'a declares: this struct borrows from some data that lives for 'a
struct Highlight<'a> {
    text: &'a str,   // a view into the source string
    start: usize,    // owned data — no lifetime needed
    end: usize,      // owned data — no lifetime needed
}
```

**Constructor and methods:**

```rust
impl<'a> Highlight<'a> {
    // 'a on the impl block means: same 'a as the struct
    fn new(source: &'a str, start: usize, end: usize) -> Option<Self> {
        if end <= source.len() && start <= end {
            Some(Highlight { text: &source[start..end], start, end })
        } else {
            None
        }
    }

    fn content(&self) -> &str { self.text }
    // ^^^ Elision rule 3: &self method — return tied to self's lifetime
    // Same as: fn content(&'a self) -> &'a str
}
```

**The compiler enforcement:**

```rust
let h2;
{
    let source = String::from("Hello, World!");
    h2 = Highlight::new(&source, 0, 5).unwrap();
    println!("{}", h2.content()); // fine — source alive
}
// source dropped here
println!("{}", h2.content()); // ERROR: source doesn't live long enough
```

**Multiple lifetime params in a struct:**

```rust
// Parser result: both fields borrow from the same input string
struct ParseResult<'a> {
    value: &'a str,      // slice of what was parsed
    remaining: &'a str,  // slice of what's left
}

// Or from different sources:
struct PairView<'a, 'b> {
    left: &'a str,
    right: &'b str,
}
```

## What This Unlocks

- **Zero-copy parsing** — build a parser that returns `ParseResult<'a>` structs pointing into the input buffer. No allocation on the hot path.
- **View types** — create lightweight, cheap-to-copy views into larger structures (config parsers, protocol decoders, text analyzers) that are provably safe.
- **Config from raw strings** — parse a config file into a struct that borrows from the file contents, avoiding any intermediate `String` allocations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Struct with references | Records can contain any value including references; GC manages validity | Structs with references need `<'a>` — lifetime ensures struct never outlives source |
| String slice views | `String.sub` allocates; views require care | `&'a str` in a struct is a first-class zero-copy view with compiler-enforced validity |
| Dangling pointers | Impossible — GC prevents | Impossible — lifetime annotations and borrow checker prevent at compile time |
| Parser result types | Typically return `string * string` pairs | Return `ParseResult<'a>` borrowing from input — zero allocation |
| Struct validity | Always valid while any reference exists | Valid only within lifetime `'a` — scope-checked by compiler |
