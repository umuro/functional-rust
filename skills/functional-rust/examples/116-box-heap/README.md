# 116: Box\<T\> — Heap Allocation

**Difficulty:** 2  **Level:** Intermediate

Explicitly allocate a value on the heap with single ownership and zero overhead.

## The Problem This Solves

In Rust, almost everything lives on the stack by default. The stack is fast, but it has two hard limits: size must be known at compile time, and data can't outlive the scope that owns it. When you try to put a large array there, the stack can overflow. When you try to store a recursive type like `enum Expr { Add(Expr, Expr) }`, the compiler refuses — it can't compute the size of a type that contains itself.

`Box<T>` solves both. It allocates `T` on the heap and keeps a thin pointer (8 bytes, always) on the stack. The compiler now knows the size of any `Box<T>`: it's always pointer-sized. The heap allocation lives until `Box` is dropped, which follows Rust's normal ownership rules — no garbage collector needed.

A third use case: heterogeneous collections. If you want a `Vec` that holds different concrete types (a `Circle` and a `Square` side by side), you need a uniform size. `Box<dyn Shape>` gives you that — each box is pointer-sized, and the heap holds whatever shape it points to.

## The Intuition

`Box<T>` is a pointer to a heap-allocated `T` that Rust's ownership system manages automatically — no `free()` required, no GC.

## How It Works in Rust

```rust
// 1. Heap-allocate a value
let b: Box<i32> = Box::new(42);
assert_eq!(*b, 42);  // Deref to access the inner value

// 2. Recursive types: without Box this won't compile
//    ("recursive type has infinite size")
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),  // Box breaks the infinite size loop
}

fn eval(e: &Expr) -> i32 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
    }
}

// 3. Trait objects: mix concrete types in one collection
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Square { side: 4.0 }),
];
for s in &shapes {
    println!("{}: {:.2}", s.name(), s.area());
}
// Each Box<dyn Shape> is pointer-sized; the heap holds the real struct
```

## What This Unlocks

- **Recursive data structures** — trees, linked lists, expression ASTs: anything that contains itself as a field.
- **Trait objects** — `Box<dyn Trait>` lets you store heterogeneous types in a single `Vec` or return different concrete types from a function.
- **Large data** — moving a `Box<[i32; 100_000]>` is always cheap (pointer copy), even if the array itself is huge.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Default storage | Heap (GC manages) | Stack (you decide) |
| Recursive types | Works without annotation | Requires `Box` |
| Trait objects | First-class modules | `Box<dyn Trait>` |
| Cleanup | Garbage collected | Dropped when owner goes out of scope |
| Overhead | GC metadata per value | Zero — one thin pointer |
