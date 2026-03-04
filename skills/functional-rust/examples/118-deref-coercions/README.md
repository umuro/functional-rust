# 118: Deref Coercions

**Difficulty:** 2  **Level:** Intermediate

The compiler auto-dereferences smart pointers and string types so you rarely need to convert manually.

## The Problem This Solves

Rust has multiple string types (`String`, `str`, `Box<String>`), multiple slice types (`Vec<T>`, `[T]`, `Box<Vec<T>>`), and various smart pointers (`Box<T>`, `Arc<T>`, `Rc<T>`). Writing conversion code every time you pass these to a function would be exhausting and noisy.

The solution is `Deref` coercions. When you pass `&String` where `&str` is expected, the compiler automatically inserts a call to `String::deref()` and produces `&str`. This happens transitively: `&Box<String>` becomes `&String` which becomes `&str` — a two-step chain the compiler figures out for you.

Without this, every function that accepts string data would have to choose one representation, forcing callers to convert. With it, you write `fn greet(name: &str)` and callers can pass `&String`, `&Box<String>`, or `&str` freely. Same for slices: write `fn sum(data: &[i32])` and callers pass `&Vec<i32>` or `&[i32]` without thinking about it.

## The Intuition

When you pass `&SomeSmartPointer` where `&InnerType` is expected, the compiler silently calls `.deref()` as many times as needed to make the types match.

## How It Works in Rust

```rust
fn greet(name: &str) {          // expects &str
    println!("Hello, {}!", name);
}

fn sum(data: &[i32]) -> i32 {   // expects &[i32]
    data.iter().sum()
}

let owned: String = "World".to_string();
greet(&owned);                  // &String → &str (one deref step)

let boxed: Box<String> = Box::new("Box".to_string());
greet(&boxed);                  // &Box<String> → &String → &str (two steps!)

let v = vec![1, 2, 3];
sum(&v);                        // &Vec<i32> → &[i32]

// You can also implement Deref for your own wrapper types:
use std::ops::Deref;

struct Wrapper<T>(T);

impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

let w = Wrapper(String::from("wrapped"));
greet(&w);   // &Wrapper<String> → &String → &str (two steps again)
```

## What This Unlocks

- **Ergonomic APIs** — functions take `&str` and `&[T]` rather than forcing callers to pick one concrete type.
- **Smart pointer transparency** — code written for `&T` works unchanged with `Box<T>`, `Arc<T>`, or `Rc<T>` behind the reference.
- **Custom wrapper types** — implement `Deref` once and your wrapper gains all the methods of the inner type automatically.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Auto type conversion | Almost none (`:>` for polymorphic variants only) | Via `Deref` trait — transitively |
| String types | One (`string`) | `String` (owned) and `&str` (borrowed slice) |
| Slice types | Arrays and `Bigarray` | `Vec<T>` (owned) and `&[T]` (borrowed slice) |
| Custom coercions | No mechanism | Implement `Deref` |
