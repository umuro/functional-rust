# 104: Mutable References (&mut T)

**Difficulty:** 2  **Level:** Intermediate

Exactly one writer can hold a mutable reference at a time — and while it exists, no readers are allowed — eliminating data races at compile time.

## The Problem This Solves

Data races are among the hardest bugs to debug. In C or Java, two threads can simultaneously read and write the same memory. The result depends on timing — it might work 99% of the time and fail mysteriously under load. Mutexes help, but they're manually applied: forget one, and the race is back. Valgrind and ThreadSanitizer catch some of these at runtime, but only if that code path runs during testing.

Even single-threaded code suffers from aliasing bugs. In C, you can have two pointers to the same memory location — one through a struct field, one through a direct pointer — and modifying through one invalidates assumptions you made through the other. The C standard calls this "undefined behavior." Rust calls it "a compile error."

Rust's mutable reference rule — "one writer, zero readers" — is a compile-time proof that no two pieces of code can observe an inconsistent state in memory. There's only ever one mutable reference at a time, and while it exists, no one else can read or write the value. Data races become structurally impossible.

## The Intuition

A mutable reference (`&mut T`) is an exclusive lock checked at compile time: exactly one piece of code can write to a value at a time, and while it's writing, no readers exist — making data races and aliasing bugs impossible by construction.

## How It Works in Rust

```rust
fn append_exclamation(s: &mut String) {
    s.push_str("!"); // exclusive write access — safe
}

fn demo_exclusive_borrow() {
    let mut text = String::from("hello");
    append_exclamation(&mut text);
    println!("{}", text); // "hello!"
}

// ERROR: can't have two mutable references at once
fn broken() {
    let mut x = 5;
    let r1 = &mut x;
    let r2 = &mut x; // ERROR: cannot borrow `x` as mutable more than once
    println!("{} {}", r1, r2);
}

// ERROR: can't mix shared and mutable references
fn also_broken() {
    let mut x = 5;
    let r1 = &x;      // shared borrow
    let r2 = &mut x;  // ERROR: cannot borrow `x` as mutable
                      // because it is also borrowed as immutable
    println!("{} {}", r1, r2);
}

// FIX: use references in separate scopes
fn fixed() {
    let mut x = 5;
    {
        let r1 = &mut x;
        *r1 += 1; // exclusive write
    } // r1 dropped here
    let r2 = &x; // now a shared borrow is fine
    println!("{}", r2); // 6
}

fn modify_vec(v: &mut Vec<i32>) {
    v.push(42);        // fine — exclusive access
    v.retain(|&n| n > 0); // also fine
}
```

## What This Unlocks

- **Data race freedom** — the compiler proves that no two paths can simultaneously read and write the same data; concurrent bugs become compile errors.
- **Aliasing elimination** — two mutable references to the same data can't coexist; no pointer aliasing bugs, no undefined behavior from overlapping mutations.
- **Safe in-place mutation** — you can freely modify data through `&mut T` knowing nothing else is observing it, enabling optimizations that alias-prone languages can't make.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable variables | `ref` cells, mutable record fields | `let mut` + `&mut T` |
| Multiple writers | Possible (runtime races in threads) | Compile error — only one `&mut T` |
| Read while writing | Possible | Compile error — `&T` and `&mut T` can't coexist |
| Data race prevention | Manual (Mutex, careful code) | Structural — compiler enforces it |
| Exclusivity enforcement | None (GC manages) | Compile time, zero runtime cost |
