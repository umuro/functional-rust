📖 **[View on hightechmind.io →](https://hightechmind.io/rust/531-lifetime-basics)**

---

# 531: Lifetime Annotations — 'a Basics

**Difficulty:** 3  **Level:** Intermediate

Name a scope. Tell the compiler which reference determines how long the output is valid.

## The Problem This Solves

Without lifetimes, this function would compile — and explode at runtime:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() >= y.len() { x } else { y }
}
// error[E0106]: missing lifetime specifier
// the compiler doesn't know if the return borrows x or y
```

The bug this prevents: returning a reference to a local variable that's been dropped. In C you'd get a dangling pointer. In Rust, the compiler refuses to compile unless you prove the output is valid.

The danger becomes concrete when lifetimes differ:

```rust
let result;
{
    let short = String::from("xyz");
    result = longest("long string", &short); // borrows short...
}
// result now points to freed memory if short's scope was shorter
println!("{}", result); // undefined behavior in C — compile error in Rust
```

Lifetimes are the proof you give the compiler: "the output is safe for this long."

## The Intuition

`'a` is just a name — a label you stick on references to say "these come from the same scope." It doesn't allocate anything, copy anything, or change runtime behavior in any way. It's a compile-time annotation, like a type — except it describes *duration* rather than *shape*.

When you write `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`, you're saying: "the output borrows from the same region as both inputs — so it lives as long as the *shorter* of the two." You're not setting a fixed duration. You're establishing a *relationship*.

The compiler uses that relationship to verify that wherever you use the output, the source data is still alive.

## How It Works in Rust

**The error first:**

```rust
// error: missing lifetime specifier — two inputs, one output, compiler can't guess
fn longest(x: &str, y: &str) -> &str {
    if x.len() >= y.len() { x } else { y }
}
```

**The fix:**

```rust
// 'a says: output lives as long as the shorter-lived of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //      ^^  ^^^^             ^^^^     ^^^^
    //  declare  annotate inputs          annotate output
    if x.len() >= y.len() { x } else { y }
}
```

**Different lifetimes when output only depends on one input:**

```rust
// Output tied to x only — y can die sooner, that's fine
fn pick_first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x // output borrows x, not y — so only 'a matters
}

// Proof it works:
let x = String::from("hello");
let result;
{
    let y = String::from("world"); // shorter lived
    result = pick_first(&x, &y);   // borrows x (long) and y (short)
}                                  // y dropped — but result only borrowed x!
println!("{}", result);            // fine!
```

**Struct holding a reference — must declare the relationship:**

```rust
struct Excerpt<'a> {
    text: &'a str,  // this struct can't outlive the string it borrows from
}
```

## What This Unlocks

- **Zero-copy string parsing** — return slices of the input without allocating, safe because the compiler tracks their validity.
- **Self-documenting APIs** — `fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str` tells callers exactly when the return value expires.
- **Confidence in scope boundaries** — you can refactor code across scopes without second-guessing which references are still valid; the compiler tells you.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Reference validity | GC guarantees references are always valid | Compile-time lifetime analysis — no GC needed |
| Returning a local reference | Possible (GC keeps it alive) | Compile error — must prove the referent outlives the reference |
| Annotation syntax | No lifetime annotations | `'a` on generic parameters and references |
| Cost | GC pause, heap allocation for closures | Zero runtime cost — annotations compile away |
| String slices | `String.sub` allocates; `Bytes.sub_string` too | `&str` is a zero-copy view — annotated with lifetimes |
