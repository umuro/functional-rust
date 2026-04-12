# Stack Allocation Patterns
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



> **Functional Rust** · [hightechmind.io](https://hightechmind.io)

## Problem Statement

Heap allocation via `malloc`/`free` (or `Box`/`Vec` in Rust) requires a system call or
at minimum a lock-protected free-list traversal, typically costing 50–200 ns per
allocation. On the stack, allocation is a single sub-instruction decrement of the stack
pointer: effectively free. For small, fixed-size buffers known at compile time, stack
allocation is 10–100× faster and produces data already in L1 cache.

Stack allocation matters in three domains: (1) embedded and real-time systems where
heap allocators may be absent or non-deterministic, (2) hot loops where allocation cost
accumulates (parsing, rendering, physics), (3) cryptographic primitives where
heap-allocated secrets leave traces in deallocated memory. The cost of stack allocation
is inflexibility: the size must be known at compile time and the data lives only for
the current stack frame.

## Learning Outcomes

- Understand the stack frame model and why stack allocation is free
- Use fixed-size arrays `[T; N]` and `const N: usize` generics for stack buffers
- Implement matrix operations over `[f64; N*M]` without heap allocation
- Apply stack allocation in hot loops by avoiding `Vec::new()` inside loops
- Recognize stack overflow risk and safe maximum stack sizes (~1–8 MB on most systems)

## Rust Application

```rust
// Fixed-size matrix on the stack — no heap allocation
struct Matrix<const ROWS: usize, const COLS: usize> {
    data: [f64; ROWS * COLS],   // const generic: size known at compile time
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    fn new() -> Self {
        Matrix { data: [0.0; R * C] }
    }

    fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r * C + c]
    }

    fn set(&mut self, r: usize, c: usize, val: f64) {
        self.data[r * C + c] = val;
    }
}

// Stack buffer for small string operations
fn format_small(n: u32) -> &'static str {
    // For truly zero-allocation formatting, use a stack buffer
    // itoa crate writes directly to [u8; 20] on the stack
    todo!("use itoa::Buffer::new()")
}

// Stack-allocated temporary in a hot loop
fn process_events(events: &[u32]) -> u64 {
    let mut acc = 0u64;
    for &e in events {
        // Small scratch buffer on stack, not heap
        let mut scratch = [0u32; 16];
        scratch[0] = e;
        scratch[1] = e.wrapping_mul(0x9e3779b9);
        // ... process without allocating
        acc += scratch.iter().map(|&x| x as u64).sum::<u64>();
    }
    acc
}

// Inline small vector — stack up to N, spills to heap
// (see smallvec / arrayvec crates)
fn median_of_small_slice(data: &[f32]) -> f32 {
    assert!(data.len() <= 16, "use heap for larger slices");
    let mut buf = [0f32; 16];
    buf[..data.len()].copy_from_slice(data);
    let slice = &mut buf[..data.len()];
    slice.sort_by(|a, b| a.partial_cmp(b).unwrap());
    slice[slice.len() / 2]
}
```

Const generics (`const N: usize`) allow generic stack-allocated arrays whose size is
checked at compile time, catching `[T; 0]` or oversized allocations.

## OCaml Approach

OCaml allocates everything (tuples, records, closures, arrays) on its minor heap by
default. The minor GC is fast (copying collector, typically < 1 µs), but it still
costs more than stack. There is no direct equivalent of stack allocation except for
`int`, `bool`, `unit`, and unboxed floats in float-only records:

```ocaml
(* OCaml: even a small array goes on the heap *)
let process_events events =
  List.fold_left (fun acc e ->
    (* This allocates a 16-element array on the minor heap *)
    let scratch = Array.make 16 0 in
    scratch.(0) <- e;
    acc + Array.fold_left (+) 0 scratch
  ) 0 events

(* Reuse a buffer to reduce allocation pressure *)
let scratch = Array.make 16 0
let process_events_reuse events =
  List.fold_left (fun acc e ->
    Array.fill scratch 0 16 0;
    scratch.(0) <- e;
    acc + Array.fold_left (+) 0 scratch
  ) 0 events
```

OCaml 5 adds unboxed arrays for numeric types (experimental), which would enable
true stack-style allocation.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Fixed-size arrays | `[T; N]` on stack, zero cost | `Array.make n x` on heap |
| Const generics | `const N: usize` | Not available |
| Small inline vector | `arrayvec`, `smallvec` crates | Manual buffer reuse |
| GC pressure | None (no GC) | Minor heap allocation per call |
| Stack overflow safety | Detectable via OS guard page | Same; default stack 8 MB |

## Exercises

1. Implement `Matrix<3, 3>` multiplication using only stack-allocated `[f64; 9]`
   arrays and benchmark it vs `Vec<f64>`-backed multiplication for 1M iterations.
2. Use the `arrayvec` crate to implement a `StackVec<T, 16>` that stores up to 16
   elements on the stack and panics on overflow. Write unit tests for push/pop/iter.
3. Implement a stack-allocated median filter for a `&[f32]` window of size up to 32
   using `[f32; 32]` and insertion sort.
4. Profile `process_events` with heap vs stack scratch buffer using `heaptrack`.
   Quantify the allocation reduction and wall-clock improvement.
5. Investigate `stacker::maybe_grow` for recursive algorithms that risk stack overflow,
   and implement a recursive Fibonacci with dynamic stack growth.
