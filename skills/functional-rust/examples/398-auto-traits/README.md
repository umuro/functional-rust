# 398: Auto Traits and Negative Impls

**Difficulty:** 4  **Level:** Expert

Traits the compiler implements automatically based on a type's fields — and how to override that decision.

## The Problem This Solves

`Send` and `Sync` need to propagate through composed types. If `Arc<T>` is `Send`, that should only be true when `T: Send`. If a struct contains an `Rc<i32>`, the struct should automatically be `!Send`. Writing all these implications by hand would be impossible — there are infinite combinations of types.

Auto traits solve this with a simple rule: a type automatically implements an auto trait if and only if all its fields do. The compiler computes this bottom-up: primitives have known properties, composed types inherit them. No user code required.

The flip side is that `unsafe` code sometimes manages its own synchronization. A type with raw pointers is `!Send` by default (raw pointers are not auto-`Send`), even if the implementation is actually thread-safe. Negative impls and unsafe positive impls let you override the compiler's conservative default.

## The Intuition

Think of auto traits as properties that propagate structurally: a container is thread-safe if and only if its contents are thread-safe. The compiler does this analysis automatically — you never write `impl Send for Vec<T> where T: Send`; it just works.

Negative impls (`impl !Send for Rc<T>`) let a type explicitly opt out of an auto trait, even if the structural rule would have allowed it. `Rc<T>` has a `*mut u8` internally — raw pointers are `!Send` by default, so `Rc` would already be `!Send`. The explicit `!Send` impl is documentation that reinforces the intent.

`unsafe impl Send` goes the other direction: "the structural rule says I'm `!Send`, but I've manually verified my synchronization is correct, and I'm taking responsibility."

## How It Works in Rust

```rust
use std::cell::Cell;
use std::sync::Arc;
use std::rc::Rc;

fn require_send<T: Send>(_: T) {}
fn require_sync<T: Sync>(_: &T) {}

// These compile — primitives and Arc are Send + Sync:
require_send(42i32);
require_sync(&42i32);
let arc = Arc::new(42i32);
require_send(arc.clone());  // Arc<i32>: Send because i32: Send

// This would FAIL to compile — Rc is !Send:
// let rc = Rc::new(42i32);
// require_send(rc);  // ERROR: Rc<i32> cannot be sent between threads safely

// Propagation: struct is Send iff ALL fields are Send
struct AllSend { x: i32, y: String, z: Arc<f64> }
// AllSend: automatically Send (all fields are Send)

// This struct would be !Send because Rc is !Send:
// struct HasRc { x: i32, rc: Rc<i32> }

// Cell<T> is !Sync (allows interior mutability without locking)
struct NonSyncType {
    data: Cell<i32>,  // Cell<i32>: !Sync
}
// NonSyncType is automatically !Sync

// Unsafe override: raw pointer makes MyBuffer !Send by default
// but we know our usage is thread-safe
struct MyRawBuffer {
    ptr: *mut u8,
    len: usize,
}
unsafe impl Send for MyRawBuffer {}
unsafe impl Sync for MyRawBuffer {}
```

**The three cases:**

| Case | What you write | When to use |
|------|---------------|-------------|
| Auto (positive) | Nothing | Type's fields are all Send/Sync — just works |
| Unsafe positive | `unsafe impl Send for T {}` | Fields aren't Send (raw pointers) but logic is thread-safe |
| Negative | `impl !Send for T {}` (nightly) | Opt out despite structural rule allowing it |

## What This Unlocks

- **Zero-cost thread safety** — every composite type automatically inherits thread-safety from its fields; no boilerplate, no runtime overhead.
- **Sound unsafe abstractions** — FFI wrappers and custom allocators can assert their own thread safety guarantees with `unsafe impl Send/Sync`.
- **Compiler-enforced API contracts** — library types like `Rc`, `Cell`, `MutexGuard` use auto-trait opt-outs to prevent misuse at compile time.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Thread safety propagation | Manual — developer ensures correctness | Auto traits propagate structurally; compiler enforces |
| Override mechanism | N/A (no auto traits) | `unsafe impl` (positive override), `impl !Trait` (negative, nightly) |
| Cell/Rc safety | GC manages memory; OCaml 5 domains have separate heaps | `Cell`→`!Sync`, `Rc`→`!Send` — type system enforces at compile time |
| Cost | Runtime GC handles sharing | Zero cost — all decisions at compile time |
