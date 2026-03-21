📖 **[View on hightechmind.io →](https://hightechmind.io/rust/700-unsafe-block)**

---

# Unsafe Blocks
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

`unsafe` blocks are Rust's explicit opt-out from the borrow checker for a specific scope. The fundamental principle: minimize the unsafe footprint. Only the code that genuinely requires unsafe operations should be inside `unsafe { }`. Safe code (error handling, logging, computation) belongs outside. This discipline makes auditing easier — reviewers can focus on the narrow `unsafe` region rather than an entire function. It is a core practice in systems programming with Rust.

## Learning Outcomes

- How `unsafe { }` scopes the suspension of safety guarantees to a minimal region
- Why safe code before and after the unsafe block is still fully checked
- How `static mut` requires unsafe access and why `AtomicU64` is usually better
- How the size of the unsafe region affects auditability
- The Rust convention: document every `unsafe` block with a `// SAFETY:` comment

## Rust Application

`GLOBAL_COUNTER: static mut u64` requires `unsafe` to read or write. `increment()` puts only `GLOBAL_COUNTER += 1` inside `unsafe { }` — the SAFETY comment explains the single-threaded guarantee. `get()` similarly wraps only the read in `unsafe`. The source demonstrates how safe operations (like computing a log message) belong outside the unsafe region even if they logically follow an unsafe operation.

Key patterns:
- Smallest possible `unsafe { }` block around only the unsound operation
- `// SAFETY: <invariant>` comment inside every unsafe block
- Safe wrapping: public function is safe, calls private unsafe block
- Static mut is almost always wrong — prefer `AtomicU64`

## OCaml Approach

OCaml has no `unsafe` blocks — all code is uniformly safe (with the exception of `Obj.magic` and direct C FFI):

```ocaml
let global_counter = ref 0
let increment () = incr global_counter
let get () = !global_counter
(* For thread safety: use Mutex.t or Atomic.t *)
```

## Key Differences

1. **Explicit opt-out**: Rust makes unsafety visible at the call site with `unsafe { }`; OCaml's safety violations (Obj.magic) are equally visible but less common.
2. **Audit surface**: Minimizing `unsafe` blocks makes security audits tractable — tools like `cargo geiger` count unsafe lines; OCaml has no equivalent metric.
3. **`static mut`**: Rust's `static mut` is inherently unsafe; OCaml's global `ref` is always safe (GC-managed, no data races in single-threaded mode).
4. **Documentation requirement**: The Rust community expects SAFETY comments in all unsafe blocks; this is a code review standard, not a compiler requirement.

## Exercises

1. **Minimize unsafe**: Take a function that wraps raw pointer access and refactor it to move all safe operations (validation, error creation, logging) outside the unsafe block.
2. **AtomicU64 replacement**: Rewrite the global counter example using `std::sync::atomic::AtomicU64` — compare the code size and verify thread safety.
3. **SAFETY documentation**: For each unsafe block in the source, write the complete SAFETY comment explaining: what invariant is required, why it holds here, and what would break if it violated.
