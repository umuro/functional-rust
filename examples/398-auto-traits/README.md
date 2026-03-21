📖 **[View on hightechmind.io →](https://hightechmind.io/rust/398-auto-traits)**

---

# 398: Auto Traits (`Send`, `Sync`, `Unpin`)
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Thread safety cannot be determined by looking at a type in isolation — it depends on all the types it contains. Rust's auto traits (`Send`, `Sync`, `Unpin`) are automatically implemented by the compiler for any type whose fields all satisfy the same trait. If a struct contains only `Send` fields, it is automatically `Send`. If it contains a `Rc<T>` (which is `!Send`), it is automatically `!Send`. This compositional property means you never have to manually declare thread safety for most types — the compiler tracks it transitively.

`Send` and `Sync` are the foundation of Rust's fearless concurrency: `Arc<T>` requires `T: Send + Sync`, `thread::spawn` requires the closure to be `Send`, and channel endpoints require `Send` values.

## Learning Outcomes

- Understand how auto traits propagate automatically through composite types
- Learn the semantic difference: `Send` means ownership can transfer between threads; `Sync` means shared references can be accessed from multiple threads
- See why `Rc<T>` is `!Send`/`!Sync` (non-atomic reference counting) while `Arc<T>` is `Send + Sync`
- Understand when `unsafe impl Send for T` is needed and what safety invariant it asserts
- Learn about `Unpin` and its role in pinning for async code

## Rust Application

In `src/lib.rs`, the `check_auto_traits` function demonstrates auto trait propagation: `i32`, `String`, and `Arc<i32>` are all `Send + Sync` because their internals are safe for concurrent access. `Rc<i32>` would fail `is_send::<Rc<i32>>()` because `Rc` uses non-atomic reference counting. `MySendSync` wraps `Arc<String>` and uses `unsafe impl Send` and `unsafe impl Sync` to manually assert thread safety.

## OCaml Approach

OCaml's runtime is single-threaded by default (the Global Interpreter Lock in OCaml 4.x), so thread safety is not enforced by the type system. OCaml 5.x with effects introduces parallel domains, but uses a different concurrency model without `Send`/`Sync` equivalents. Thread safety in OCaml is achieved through `Mutex.t` and `Atomic.t` modules, but there is no compile-time enforcement.

## Key Differences

1. **Compile-time vs. runtime**: Rust enforces thread safety at compile time via auto traits; OCaml enforces it at runtime via locks (OCaml 4.x) or domain isolation (OCaml 5.x).
2. **Opt-in vs. opt-out**: Rust types are `Send`/`Sync` by default (if fields are); opting out requires `!Send`/`!Sync` via `PhantomData`. OCaml has no type-level thread safety.
3. **`unsafe` escape hatch**: Rust allows `unsafe impl Send/Sync` for manually verified correctness; OCaml trusts the programmer entirely without a special annotation.
4. **`Unpin`**: Rust's `Unpin` enables safe movement of pinned values (key for async); OCaml has no pinning concept since values don't have stable addresses.

## Exercises

1. **Thread safety test**: Write a test that spawns 4 threads, each calling a function `fn use_it<T: Send + Sync>(val: Arc<T>)`. Pass it `Arc<i32>`, `Arc<String>`, and `Arc<Vec<u8>>`. Then try passing `Arc<RefCell<i32>>` and observe the compile error.
2. **Custom Send type**: Create a `SafeCounter` struct wrapping `AtomicU64`. Manually implement `unsafe impl Send for SafeCounter` and `unsafe impl Sync for SafeCounter` with a code comment explaining the safety invariant you're asserting.
3. **!Send propagation**: Create a `NotSendStruct { rc: Rc<i32> }` and write a test that verifies (using a compile-fail test or `static_assertions` crate) that it is not `Send`. Explain why the `Rc` field causes this.
