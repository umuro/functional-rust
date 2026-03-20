📖 **[View on hightechmind.io →](https://hightechmind.io/rust/460-send-sync-bounds)**

---

# 460: `Send` and `Sync` Bounds

## Problem Statement

Rust's type system prevents data races at compile time through two auto traits. `Send` means a type's ownership can be transferred to another thread — its memory can be safely moved across thread boundaries. `Sync` means a shared reference `&T` can be safely used from multiple threads simultaneously. These are auto-derived compositionally: a struct is `Send` if all fields are `Send`. `Rc<T>` is `!Send` because its reference counter isn't atomic. `Cell<T>` is `!Sync` because interior mutability without synchronization is unsafe across threads.

`Send` and `Sync` are the compile-time foundation of Rust's fearless concurrency — every thread spawn, channel send, and `Arc` clone depends on them.

## Learning Outcomes

- Understand the semantic difference: `Send` = ownership transfer, `Sync` = shared reference sharing
- Learn which standard types are `!Send`/`!Sync` and why: `Rc`, `Cell`, `RefCell`, raw pointers
- See how `PhantomData<T>` marks a type as `!Send` or `!Sync` without holding a value
- Understand when `unsafe impl Send/Sync` is needed and the safety invariant it asserts
- Learn how `Arc<Mutex<T>>` achieves `Send + Sync` by composing individually safe primitives

## Rust Application

In `src/lib.rs`, `ThreadSafe { data: i32 }` is automatically `Send + Sync` since `i32` is both. `SendNotSync { data: Cell<i32> }` is `Send` (can be moved to another thread) but not `Sync` (can't be shared by reference, since `Cell` lacks synchronization). `NotSendNotSync` uses `PhantomData<Rc<()>>` to mark the type as `!Send + !Sync`. The helper functions `assert_send::<T>()` and `assert_sync::<T>()` verify bounds at compile time.

## OCaml Approach

OCaml has no equivalent of `Send`/`Sync` — the type system doesn't track thread safety. In OCaml 4.x, the GIL ensures only one thread runs OCaml code at a time, making data races impossible. In OCaml 5.x, data races on mutable values are possible and the programmer is responsible for synchronization. There is no compile-time enforcement — OCaml 5.x's safety relies on programming discipline and tools like ThreadSanitizer.

## Key Differences

1. **Compile-time vs. discipline**: Rust enforces thread safety at compile time; OCaml 5.x relies on programmer discipline and runtime tools.
2. **Auto-derivation**: Rust automatically derives `Send`/`Sync` based on fields; OCaml has no equivalent automatic annotation.
3. **Unsafe override**: Rust allows `unsafe impl Send for T` for manually verified safety; OCaml has no such mechanism.
4. **GC interaction**: OCaml's GC makes some values inherently thread-unsafe (mutable values shared across domains in OCaml 5.x); Rust's ownership system eliminates this class of bug.

## Exercises

1. **Send wrapper**: Implement `struct ForceSend<T>(T)` with `unsafe impl Send for ForceSend<T>`. Write a function that wraps an `Rc<i32>` (which is `!Send`) in `ForceSend` and sends it to another thread. In a comment, explain the safety invariant you're asserting and why it's actually unsafe.
2. **Sync assertion test**: Use `static_assertions::assert_impl_all!(T: Send + Sync)` (or write your own `fn assert_send_sync<T: Send + Sync>()`) to verify at compile time that your custom types satisfy the expected bounds. Add tests for `Arc<i32>`, `Mutex<i32>`, `Arc<Mutex<i32>>`, `Rc<i32>`.
3. **Custom concurrent type**: Design `struct ConcurrentBag<T: Send>` wrapping `Arc<Mutex<Vec<T>>>`. Manually implement `Send` and `Sync` with safety proof in comments. Verify it can be shared across threads using `Arc::clone`.
