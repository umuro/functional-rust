# 536: 'static Lifetime

**Difficulty:** 3  **Level:** Intermediate

`'static` means "valid for the entire program." String literals are `'static`. Thread-spawned closures must be `'static`. Understanding when to require it — and when *not* to — prevents a common class of beginner frustration.

## The Problem This Solves

When you spawn a thread or store something in a global, the data must be valid for as long as the program runs. The compiler can't prove a borrowed reference from a local scope survives that long:

```rust
fn spawn_worker(data: &str) {
    std::thread::spawn(|| {
        println!("{}", data); // ERROR: data may not live long enough
        // data is borrowed — it could be dropped before the thread finishes
    });
}
```

The fix is either to *own* the data (move it into the thread) or require it to be `'static`. The `'static` bound communicates: "I need this to live forever — don't give me a temporary borrow."

`T: 'static` doesn't mean `T` is a static variable. It means `T` contains no borrowed references that could expire. An owned `String` satisfies `T: 'static` because it owns all its data.

## The Intuition

There are two distinct uses of `'static`:

1. **`&'static str`** — a reference that truly lives forever (string literals, `static` variables). The data is baked into the binary.

2. **`T: 'static` as a bound** — means "T owns its data, or its borrows are `'static`." This doesn't mean T lives forever. It means: *if* T contains references, they're `'static`. An owned `String` satisfies this because it has *no* references with limited scope.

The `'static` bound is really saying "no borrowed data with a limited lifetime" — not "this value will exist forever."

## How It Works in Rust

**String literals are `'static`:**

```rust
// Embedded in binary — valid for entire program duration
let s: &'static str = "I will never be freed";
static APP_NAME: &str = "MyApp"; // also &'static str
```

**Thread spawning requires `'static`:**

```rust
// Works: owned data, moved into thread — satisfies 'static
let data = vec![1, 2, 3];
std::thread::spawn(move || {
    println!("{:?}", data); // data moved — no borrow, safe
});

// Fails: borrowed reference with limited scope
let data = vec![1, 2, 3];
std::thread::spawn(|| {
    println!("{:?}", &data); // ERROR: data doesn't live long enough
});
```

**`T: 'static` bound — what satisfies it:**

```rust
fn store_globally<T: 'static>(value: T) { /* ... */ }

store_globally(String::from("owned"));  // ✓ no borrowed refs
store_globally(42i32);                  // ✓ no refs at all
store_globally(vec!["a", "b"]);         // ✓ &'static str — fine
store_globally("literal");              // ✓ &'static str

// &String would fail — it's a borrow with limited scope
let s = String::from("temp");
// store_globally(&s); // ERROR: &s doesn't satisfy 'static
```

**Lazy global initialization:**

```rust
use std::sync::OnceLock;
static CONFIG: OnceLock<Vec<String>> = OnceLock::new();

fn get_config() -> &'static [String] {
    CONFIG.get_or_init(|| vec!["setting1".to_string()])
    // Returns &'static — valid forever once initialized
}
```

## What This Unlocks

- **Thread-safe data sharing** — move owned data into threads without reference counting. `T: 'static + Send` is the baseline for `thread::spawn`.
- **Global caches and lazy initialization** — `OnceLock<T>` and `static` variables return `&'static T` — references you can store anywhere without worrying about lifetimes.
- **Plugin and trait-object APIs** — `Box<dyn Trait + 'static>` is the default for stored trait objects. Understanding `'static` prevents the common "trait object requires 'static" confusion.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String literals | `string` values on the heap, GC-managed | `&'static str` — embedded in binary, truly immortal |
| Global state | `let` at module level, GC-managed | `static` with type annotation; `OnceLock` for lazy initialization |
| Thread data lifetime | Values kept alive by GC across threads | Must be `'static + Send` — owned or truly immortal |
| `T: 'static` bound | No equivalent — GC handles all | "T owns its data" — no borrowed refs with limited scope |
| Eternal references | All live references are eternal (GC) | Only `&'static T` is eternal — all others have scoped lifetimes |
