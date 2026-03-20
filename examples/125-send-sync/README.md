📖 **[View on hightechmind.io →](https://hightechmind.io/rust/125-send-sync)**

---

# Send and Sync Marker Traits

## Problem Statement

Data races are a class of concurrency bugs where two threads access the same memory simultaneously with at least one write, with no synchronization. They cause undefined behavior in C/C++ and subtle bugs in GC languages. Rust eliminates them at compile time using two marker traits: `Send` (a type's ownership can cross thread boundaries) and `Sync` (shared references to the type can be accessed from multiple threads). Violating these rules is a compile error, not a runtime crash.

## Learning Outcomes

- Understand what `Send` and `Sync` mean and how they prevent data races at compile time
- Learn why `Rc<T>` is neither `Send` nor `Sync`, but `Arc<T>` is both (when `T: Send + Sync`)
- See `Arc<Mutex<T>>` as the canonical pattern for shared mutable state across threads
- Understand channel-based concurrency (`mpsc`) as an alternative to shared state

## Rust Application

`parallel_sum` uses `Arc<Mutex<i32>>` — `Arc` enables shared ownership across threads, `Mutex` ensures exclusive mutable access. The compiler verifies that the closure moved into `thread::spawn` is `Send`: all captured types must be `Send`. The channel-based `channel_sum` avoids shared state entirely: each thread sends its partial result down an `mpsc::channel`, and the main thread aggregates. Both patterns are data-race-free by construction.

## OCaml Approach

OCaml (before Domain-based parallelism in OCaml 5) used a Global Interpreter Lock — only one thread ran OCaml code at a time, so data races on GC-managed values were impossible. OCaml 5 introduces `Domain`s and requires careful use of atomic operations and mutexes for shared mutable state. OCaml has no compile-time equivalents of `Send`/`Sync`; safety is the programmer's responsibility.

## Key Differences

1. **Compile-time vs. runtime**: Rust catches data races at compile time via `Send`/`Sync`; OCaml relies on runtime locking or the GIL (pre-5) to prevent them.
2. **`Rc` vs. `Arc`**: Rust provides both non-atomic (`Rc`, not `Send`) and atomic (`Arc`, `Send + Sync`) reference counting; OCaml has one GC-managed reference type.
3. **Automatic derivation**: `Send` and `Sync` are auto-implemented for all types whose fields are `Send`/`Sync`; adding a non-`Send` field (like raw pointer) automatically breaks thread safety.
4. **Channels**: Both Rust's `mpsc` and OCaml's `Event` module provide channel primitives; Rust's are typed and checked at compile time.

## Exercises

1. Try sharing an `Rc<i32>` across threads — observe the compile error explaining why `Rc` is not `Send`.
2. Implement a parallel map using `thread::spawn` and channels: split a `Vec<i32>` into chunks, process each in a thread, gather results.
3. Write a custom type with a raw pointer and manually implement `Send` (with `unsafe`), explaining what invariant you are promising to uphold.
