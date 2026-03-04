# 501: Closure Capture Rules

**Difficulty:** 2  **Level:** Beginner-Intermediate

Rust closures borrow what they need — understanding the three capture modes unlocks the borrow checker.

## The Problem This Solves

When you write `|x| x + offset` in Rust, the compiler has to decide: does the closure *own* `offset`, borrow it immutably, or borrow it mutably? Get this wrong and you hit errors like `closure may outlive the current function`, `cannot move out of borrowed content`, or `cannot borrow as mutable because it is borrowed as immutable`.

Without understanding capture rules, you'll spend hours fighting the borrow checker on code that should be simple. You'll reach for `.clone()` everywhere or sprinkle `move` blindly hoping things compile.

The three modes — `&T` (shared borrow), `&mut T` (exclusive borrow), and ownership via `move` — map directly to Rust's ownership model. Closures follow the same rules as variables; they're just anonymous structs that implement `Fn`/`FnMut`/`FnOnce`.

## The Intuition

Think of a closure as a small struct that stores snapshots of the variables it touches. The compiler picks the *least restrictive* snapshot possible:

- Just reads a variable? Store a `&T` (shared reference).
- Modifies a variable? Store a `&mut T` (exclusive reference).
- Needs to outlive the variable's scope? Use `move` to take ownership.

In Python, `lambda x: x + offset` silently captures `offset` by reference into the enclosing scope — Python's GC keeps it alive. In JavaScript, arrow functions close over the entire lexical scope. Rust instead makes the capture mode explicit and enforced by the compiler.

The `move` keyword is your escape hatch: it forces the closure to *own* all its captures, cutting the lifetime tie to the outer scope.

## How It Works in Rust

```rust
// Mode 1: Capture by shared reference (&T)
// Compiler picks this when the closure only reads
let x = 10;
let add_x = |n| n + x;   // borrows x as &i32
println!("{}", add_x(5)); // x still usable here — shared borrow

// Mode 2: Capture by mutable reference (&mut T)
// Compiler picks this when the closure writes
let mut count = 0;
let mut increment = || count += 1; // borrows count as &mut i32
increment(); increment();
drop(increment);           // release &mut borrow before reading count again
println!("{}", count);     // 2

// Mode 3: Capture by value (move)
// Required when closure must outlive the captured variable's scope
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |n| n + x   // x is *moved* into the closure; closure can outlive this fn
}

// String moves — non-Copy types
let greeting = String::from("hello");
let greet = move |name: &str| format!("{}, {}!", greeting, name);
// greeting is MOVED — can't use it here anymore
```

The compiler infers the mode from usage. You only write `move` explicitly — everything else is automatic.

## What This Unlocks

- **Return closures from functions** — `move` is required because the closure outlives the function's stack frame.
- **Thread safety** — `thread::spawn(move || ...)` requires the closure to own its data (`'static` bound).
- **Reasoning about the borrow checker** — understanding captures explains 80% of borrow-checker errors involving closures.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Capture mode | Automatic (GC manages lifetime) | Inferred: `&T`, `&mut T`, or `move` |
| Outliving scope | GC handles it | Requires `move` to own the capture |
| Thread closure | GC-safe by default | `move ||` needed for `Send + 'static` |
| Explicit ownership transfer | No concept | `move` keyword |
| Lifetime of closure | GC keeps alive | Bounded by captures unless `move` |
