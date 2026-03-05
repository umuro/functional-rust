# 775: Fixed-Size Arrays with const N Parameter

**Difficulty:** 3  **Level:** Intermediate

Build stack-allocated data structures — ring buffers, fixed stacks, sliding-window statistics — where the capacity is a type parameter, not a runtime value.

## The Problem This Solves

A ring buffer that allocates a `Vec<T>` internally pays for heap allocation, pointer indirection, and dynamic resizing. In many contexts — embedded systems, real-time audio processing, network packet buffering, sensor data windowing — you know the maximum capacity at compile time and want it on the stack. No allocation, no bounds-check overhead, no fragmentation.

The challenge before const generics was that `struct RingBuffer<T>` couldn't encode the capacity in its type. You'd hard-code it (`struct RingBuffer8<T>([T; 8], ...)`) or use a heap `Vec`. Const generics solve this: `RingBuffer<f32, 16>` and `RingBuffer<f32, 1024>` are different types with different stack footprints, and you choose the right one at the call site.

Sliding-window statistics (for sensor smoothing, rate calculation, moving averages) are a particularly common use case: a `RingBuffer<f64, 32>` holding the last 32 sensor readings, with zero dynamic allocation.

## The Intuition

Think of C's `float buf[N]` where `N` is a `#define` constant — except Rust's version is generic and type-safe. The type `RingBuffer<f32, 16>` carries `16` as part of its type signature, so the compiler knows the array is exactly 16 elements at compile time.

For Python developers, this feels like NumPy arrays where shape is part of the type. For JavaScript developers, there's no direct equivalent — TypeScript's fixed-tuple types are the closest approximation.

## How It Works in Rust

```rust
// The capacity N is part of the type — RingBuffer<f32, 16> vs RingBuffer<f32, 1024>
#[derive(Debug)]
pub struct RingBuffer<T: Copy + Default, const N: usize> {
    data: [T; N],   // stack-allocated array — no heap
    head: usize,    // index of oldest element
    count: usize,   // how many valid elements
}

impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        // [T::default(); N] — zero-initialize the array at compile-known size
        Self { data: [T::default(); N], head: 0, count: 0 }
    }

    pub fn push(&mut self, val: T) {
        let tail = (self.head + self.count) % N;
        self.data[tail] = val;
        if self.count < N {
            self.count += 1;          // not full yet — just grow
        } else {
            self.head = (self.head + 1) % N;  // full — overwrite oldest
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.count).map(|i| &self.data[(self.head + i) % N])
    }

    pub fn is_full(&self) -> bool { self.count == N }
}

// Fixed-capacity stack — N is the maximum depth
pub struct FixedStack<T: Copy + Default, const N: usize> {
    data: [T; N],
    top: usize,
}

impl<T: Copy + Default, const N: usize> FixedStack<T, N> {
    pub fn push(&mut self, v: T) -> bool {
        if self.top >= N { return false; }  // stack full — no panic
        self.data[self.top] = v;
        self.top += 1;
        true
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 { return None; }
        self.top -= 1;
        Some(self.data[self.top])
    }
}

// Sliding-window moving average — no heap allocation
let mut window: RingBuffer<f64, 8> = RingBuffer::new();
for reading in sensor_data {
    window.push(reading);
    if window.is_full() {
        let avg: f64 = window.iter().sum::<f64>() / 8.0;
        println!("Moving avg: {avg:.2}");
    }
}

// size_of confirms stack allocation
println!("{} bytes", std::mem::size_of::<RingBuffer<f64, 8>>());
// 8 × 8 bytes (f64) + 2 × 8 bytes (usize) = 80 bytes — on the stack
```

Key points:
- `[T::default(); N]` requires `T: Default + Copy` — `Default` for the initial value, `Copy` because arrays use `Copy` for initialization
- The `% N` modular arithmetic keeps `head` and `tail` within bounds without bounds-check panics
- Returning `false` from `push` on a full stack is better than panicking — let callers decide
- `std::mem::size_of::<RingBuffer<f64, 8>>()` is computed at compile time — no surprises
- `iter()` returning `impl Iterator` hides the modular arithmetic from callers

## What This Unlocks

- **Real-time embedded code**: no heap allocator required — all memory is in the type's stack footprint; the linker can compute total memory usage statically
- **Sensor smoothing and signal processing**: sliding-window average, variance, min/max over the last N samples — allocate once, never GC
- **Capacity-checked data structures**: `FixedStack<u32, 64>` — the capacity is part of the public API type signature, not documentation

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Fixed-size collection | `Array.make n default` (heap) | `[T; N]` — stack-allocated, size in type |
| Capacity in type | Bigarray with kind | `struct RingBuffer<T, const N: usize>` |
| Default initialization | `Array.make n 0` | `[T::default(); N]` — requires `T: Default + Copy` |
| Overflow on push | Exception or `None` | Return `bool` or `Option` |
| Memory layout | Heap pointer + length | Inline — part of the struct's stack frame |
| Moving average | Mutable array + index arithmetic | Same — `% N` modular arithmetic |
