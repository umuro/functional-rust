# 725: Stack Allocation Patterns

**Difficulty:** 3  **Level:** Expert

Keep small, fixed-size data on the stack — zero allocator overhead, automatic cleanup, L1-cache locality.

## The Problem This Solves

Every call to `Box::new`, `Vec::new`, `String::from`, or `HashMap::new` goes through the heap allocator. Heap allocation is not free: the allocator traverses free lists or calls into the OS, memory may not be cache-hot, and every allocation eventually needs a `dealloc`. For large or dynamically-sized data this cost is unavoidable. For small, fixed-size data, it's unnecessary.

Rust gives you a tool that most languages don't: fixed-size arrays `[T; N]` live in the stack frame. No allocator, no pointer indirection, automatic cleanup when the frame returns. A 4×4 matrix of `f32` is 64 bytes on the stack — you can allocate and compute with it entirely in L1 cache. An inline string buffer for a 23-byte string needs no heap at all.

The discipline is knowing when to use which. Use `[T; N]` when the size is known at compile time and small enough to stack (rule of thumb: under 4KB). Use `Vec<T>` when the size is dynamic or potentially large. The `ArrayVec` pattern — a fixed-capacity array with a runtime length counter — gives you push/pop semantics without heap allocation, useful for accumulating a small number of results.

## The Intuition

The stack is your fastest memory. It's just a pointer decrement — no allocator, no OS call, no cache miss. Data on the stack is in the same memory region as your local variables and function arguments, so it's almost certainly already in L1. The cost is that you must know the size at compile time, and the stack is typically limited to a few MB. For buffers, matrices, short strings, and small collections, it's the right choice.

## How It Works in Rust

```rust
// A 4×4 matrix — 64 bytes, entirely on the stack.
fn matmul4(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut c = [[0.0f32; 4]; 4];  // zero init, on stack
    for i in 0..4 { for k in 0..4 { for j in 0..4 {
        c[i][j] += a[i][k] * b[k][j];
    }}}
    c  // returned by value — compiler applies NRVO, no copy
}

// Inline string buffer — no heap for short strings.
struct StackStr {
    buf: [u8; 64],
    len: usize,
}

impl StackStr {
    fn new(s: &str) -> Option<Self> {
        if s.len() > 64 { return None; }
        let mut buf = [0u8; 64];
        buf[..s.len()].copy_from_slice(s.as_bytes());
        Some(StackStr { buf, len: s.len() })
    }
    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buf[..self.len]).unwrap()
    }
}

// ArrayVec pattern — fixed-capacity, no heap.
struct ArrayVec<T, const N: usize> {
    data: [std::mem::MaybeUninit<T>; N],
    len: usize,
}
```

Pass large stack arrays by reference (`&[T; N]` or `&mut [T; N]`) to avoid copies across function calls. The compiler's NRVO (Named Return Value Optimisation) often eliminates copies at return boundaries.

## What This Unlocks

- **Zero-allocation hot paths**: Process sensor readings, audio frames, or render commands in a tight loop with no allocator involvement.
- **Predictable latency**: No GC pauses, no allocator stalls. Stack allocation cost is a handful of CPU cycles at frame entry.
- **Cache-friendly numerics**: 4×4 matrix, SIMD buffer, or small DSP filter coefficients fit in a single cache line — no pointer chasing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Stack allocation | Only integers/floats — records are boxed | `let x: [u8; 64] = [0; 64]` — truly on stack |
| Fixed-size array | Not available (OCaml always boxes arrays) | `[T; N]` — inline in stack frame |
| Small string without alloc | Not available | `[u8; 32]` + length counter |
| Inline growth with cap | Not idiomatic | `ArrayVec<T, N>` pattern |
| Return large value by value | Boxed on heap | NRVO often eliminates the copy |
| Array size must be compile-time | Not applicable | `const N: usize` generic parameter |
