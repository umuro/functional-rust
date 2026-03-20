📖 **[View on hightechmind.io →](https://hightechmind.io/rust/780-const-generic-struct)**

---

# 780-const-generic-struct — Const Generic Struct

## Problem Statement

Ring buffers (circular buffers) are fundamental data structures in operating systems, audio processing, and producer-consumer patterns. A fixed-capacity ring buffer avoids heap allocation and provides O(1) push and pop with power-of-two sizes enabling fast modulo via bitmask. Using a const generic capacity `RingBuffer<T, CAP>` embeds the capacity in the type, enabling different capacities to be different types with no runtime dispatch.

## Learning Outcomes

- Implement `RingBuffer<T: Copy, const CAP: usize>` with head, tail, and len tracking
- Write `push(item) -> Result<(), T>` (returns item on overflow) and `pop() -> Option<T>`
- Use modulo arithmetic to wrap head and tail indices: `(self.tail + 1) % CAP`
- Implement `peek()`, `is_empty()`, `is_full()`, and `iter()` for a complete API
- Verify that `RingBuffer<u32, 8>` is exactly `(8 * 4 + overhead)` bytes with no heap pointer

## Rust Application

`RingBuffer<T, CAP>` stores `data: [Option<T>; CAP]`, `head: usize`, `tail: usize`, and `len: usize`. `push` checks `is_full()`, writes to `data[tail]`, wraps tail, and increments len. `pop` reads `data[head]`, clears it, wraps head, decrements len. The `CAP` parameter is used directly in array size and modulo. Tests verify FIFO ordering, capacity enforcement, and wrap-around behavior.

## OCaml Approach

OCaml's `Queue.t` is a standard doubly-linked-list queue — heap-allocated, no fixed capacity. For fixed-capacity ring buffers, OCaml uses `Array.make cap None` with mutable `head`, `tail`, and `len` references. Libraries like `Faraday` use ring buffers internally for I/O buffering. OCaml's `Bigarray` provides C-allocated ring buffers for zero-copy I/O in `capnp-rpc` and `mirage`.

## Key Differences

1. **Type-level capacity**: Rust's `RingBuffer<u32, 8>` and `RingBuffer<u32, 16>` are different types; OCaml's equivalent would be the same `int array ref record` type at different sizes.
2. **Stack allocation**: Rust's small ring buffers can live on the stack; OCaml arrays are always heap-allocated.
3. **Option overhead**: `[Option<T>; CAP]` uses one byte per slot for the discriminant; a production implementation uses `MaybeUninit<T>` to avoid this.
4. **Power-of-two optimization**: Rust can specialize for power-of-two `CAP` using `& (CAP - 1)` instead of `% CAP`; this is a zero-cost const-generic specialization.

## Exercises

1. Optimize for power-of-two capacity: replace `% CAP` with `& (CAP - 1)` and add a `const_assert!(CAP.is_power_of_two())` in `new()`.
2. Replace `[Option<T>; CAP]` with `[MaybeUninit<T>; CAP]` to eliminate the Option discriminant overhead — add a safety comment explaining the invariant.
3. Add an `Iterator` implementation for `RingBuffer` that reads elements from head to tail without modifying the buffer.
