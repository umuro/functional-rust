# 107: Lifetimes in Structs

**Difficulty:** 3  **Level:** Advanced

When a struct holds a reference, it must carry a lifetime parameter — proving the struct can't outlive the data it points into.

## The Problem This Solves

Storing a reference in a struct is a trap in C. A struct that holds a `char*` pointing into a local variable will become a dangling pointer the moment that local variable goes out of scope — but nothing stops you from storing the struct globally or passing it to another function. The struct looks valid; reading it returns garbage or crashes.

In Java, this can't happen — every reference keeps the pointed-to object alive. But that means objects that "should" be freed stay in memory as long as any reference exists, causing memory leaks in long-lived caches.

Rust handles this exactly right: a struct holding a reference is only valid as long as the referenced data is alive. The lifetime parameter on the struct makes this relationship explicit. The compiler then tracks every instance of the struct and ensures it never outlives the data it points into. No dangling pointers, no memory leaks, no surprises.

## The Intuition

A struct that holds a reference must declare its lifetime (`'a`) to let the compiler enforce: this struct instance cannot outlive the data it's borrowing — the struct is valid for exactly as long as the reference is valid.

## How It Works in Rust

```rust
// ERROR: struct holds reference but has no lifetime parameter
struct Excerpt {
    text: &str, // ERROR: missing lifetime specifier
}

// FIX: declare the lifetime on the struct
struct Excerpt<'a> {
    text: &'a str, // "this struct lives no longer than the &str it holds"
}

fn demo() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    
    let first_sentence;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = &novel[..i];
    }
    
    let excerpt = Excerpt { text: first_sentence };
    println!("{}", excerpt.text); // fine — novel is still alive
}

// Lifetime on impl blocks mirrors the struct
impl<'a> Excerpt<'a> {
    fn text(&self) -> &str {
        // Return elides to &'a str — the excerpt's lifetime
        self.text
    }
    
    fn announce(&self, announcement: &str) -> &str {
        // Rule 3: output gets self's lifetime
        println!("Attention: {}", announcement);
        self.text
    }
}

// ERROR: struct outlives the data it references
fn dangling_excerpt() -> Excerpt<'_> {
    let text = String::from("temporary");
    Excerpt { text: &text } // ERROR: text dropped when function returns
}

// Multiple references — each with its own lifetime constraint
struct TwoStrings<'a, 'b> {
    first: &'a str,
    second: &'b str,
}
```

## What This Unlocks

- **Zero-copy views into data** — a struct can hold slices and references into large buffers without copying, and the compiler ensures the buffer outlives the view.
- **Explicit ownership documentation** — the lifetime parameter in `Excerpt<'a>` immediately tells readers "this struct borrows from somewhere; it doesn't own its data."
- **Parser and compiler patterns** — AST nodes, parsed tokens, and iterator adaptors that reference underlying data are safely expressible as lifetime-parameterized structs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Struct holding reference | Automatic (GC keeps target alive) | Must declare lifetime parameter |
| Struct outliving data | Impossible (GC prevents) | Compile error (lifetime check) |
| Zero-copy string views | Less common (strings are values) | Idiomatic with `&'a str` fields |
| Lifetime annotation on struct | Not needed | Required when struct holds references |
| Dangling struct reference | Can't happen (GC) | Can't happen (borrow checker) |
