📖 **[View on hightechmind.io →](https://hightechmind.io/rust/346-runtime-context)**

---

# 346: Runtime Context
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Distributed systems attach request IDs, trace spans, and user session tokens to every operation without threading those values through every function call. Thread-local storage solves this: each thread has its own copy of the context variable, invisible to other threads. This enables "ambient context" — logging, metrics, and tracing libraries can read the current request ID without the application code explicitly passing it. Java's `ThreadLocal`, Python's `contextvars.ContextVar`, and Go's `context.Context` (passed explicitly) all address the same problem. Rust uses `thread_local!` for synchronous code and task-local variables (via `tokio::task_local!`) for async code.

## Learning Outcomes

- Declare thread-local storage with `thread_local!` and `RefCell<Option<T>>`
- Read and write thread-local values with the `.with()` closure API
- Implement `with_context` as a scoped setter that restores the previous value
- Understand why thread-local values are not automatically propagated to spawned threads
- Recognize the difference between thread-local (per OS thread) and task-local (per async task)
- Use this pattern for request ID propagation, logging context, and tracing spans

## Rust Application

```rust
use std::cell::RefCell;

thread_local! {
    static CONTEXT: RefCell<Option<String>> = RefCell::new(None);
}

pub fn set_context(ctx: String) {
    CONTEXT.with(|c| *c.borrow_mut() = Some(ctx));
}

pub fn get_context() -> Option<String> {
    CONTEXT.with(|c| c.borrow().clone())
}

pub fn with_context<R>(ctx: String, f: impl FnOnce() -> R) -> R {
    let old = get_context();
    set_context(ctx);
    let result = f();
    match old {
        Some(c) => set_context(c),
        None => CONTEXT.with(|c| *c.borrow_mut() = None),
    }
    result
}
```

`thread_local!` creates a static that has a separate instance per OS thread. The `RefCell` allows interior mutability within the single-threaded context (no lock needed since only one thread accesses its own local). `with_context` saves and restores the previous value, enabling nested context scopes.

## OCaml Approach

OCaml 5 uses `Domain.DLS` (Domain-Local Storage) for the equivalent:

```ocaml
let key = Domain.DLS.new_key (fun () -> None)

let set_context ctx =
  Domain.DLS.set key (Some ctx)

let get_context () =
  Domain.DLS.get key

let with_context ctx f =
  let old = get_context () in
  set_context (Some ctx);
  let result = f () in
  Domain.DLS.set key old;
  result
```

In OCaml 4, plain `ref` values suffice since threads share a GIL — each "thread-local" is just a per-thread mutable reference without synchronization concerns. `Effect`-based frameworks (OCaml 5) can propagate context automatically across continuations.

## Key Differences

| Aspect | Rust `thread_local!` | OCaml `Domain.DLS` |
|--------|---------------------|---------------------|
| Scope | Per OS thread | Per domain (OCaml 5) |
| Async task scope | Requires `tokio::task_local!` | Requires effect-based propagation |
| Initialization | Lazy, per thread | Via `new_key` factory |
| Interior mutability | `RefCell` needed | Implicit (mutable by default) |
| Cross-thread visibility | None (by design) | None (by design) |

## Exercises

1. **Request ID logger**: Implement a logging function `log(msg: &str)` that prepends the current context (request ID) to every message; demonstrate nested contexts with different IDs.
2. **Thread propagation**: Spawn a thread inside a `with_context` block; observe that the spawned thread does NOT inherit the context; manually copy it by capturing the value before spawning.
3. **Async task-local**: Using `tokio::task_local!`, implement the same `with_context` pattern for async tasks; verify that concurrent tasks each see their own context independently.
