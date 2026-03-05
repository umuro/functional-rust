# OCaml vs Rust: Stack Allocation Patterns

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: local integers and floats are unboxed on the stack.
   Arrays and most compound values are heap-allocated by the GC. *)

let stack_int_ops () =
  let a = 42 in
  let b = 100 in
  a + b  (* no allocation — unboxed stack ints *)

(* Float arrays get a special unboxed optimisation in OCaml *)
let unboxed_float_array () =
  let arr = Array.init 8 (fun i -> float_of_int i *. 1.5) in
  Array.fold_left ( +. ) 0.0 arr

(* No language-level fixed-capacity stack vector exists;
   the closest is a plain array with a manual length counter. *)
let arrayvec_sim () =
  let buf = Array.make 4 0 in
  let len = ref 0 in
  buf.(!len) <- 10; incr len;
  buf.(!len) <- 20; incr len;
  (buf, !len)
```

### Rust (idiomatic — fixed-size array)
```rust
// [T; N] lives entirely in the stack frame — zero allocator overhead.
pub fn sum_stack_array() -> f64 {
    let data: [f64; 16] = [
        1.0, 2.0, 3.0, 4.0,  5.0,  6.0,  7.0,  8.0,
        9.0,10.0,11.0,12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    data.iter().copied().sum()
}

// 4×4 matrix multiply — 96 bytes on the stack, fits in L1 cache.
pub fn matmul4(a: &[[f32;4];4], b: &[[f32;4];4]) -> [[f32;4];4] {
    let mut c = [[0.0f32; 4]; 4];
    for i in 0..4 { for k in 0..4 { for j in 0..4 {
        c[i][j] += a[i][k] * b[k][j];
    }}}
    c
}
```

### Rust (functional — inline string buffer, no heap)
```rust
pub struct InlineStr<const CAP: usize> {
    buf: [u8; CAP],
    len: usize,
}

impl<const CAP: usize> InlineStr<CAP> {
    pub const fn new() -> Self { Self { buf: [0u8; CAP], len: 0 } }

    pub fn push_str(&mut self, s: &str) -> bool {
        let bytes = s.as_bytes();
        if self.len + bytes.len() > CAP { return false; }
        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        true
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buf[..self.len]).expect("valid UTF-8")
    }
}
```

### Rust (ArrayVec — stack-backed push/pop)
```rust
pub struct ArrayVec<T, const CAP: usize> {
    data: [std::mem::MaybeUninit<T>; CAP],
    len: usize,
}

impl<T, const CAP: usize> ArrayVec<T, CAP> {
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == CAP { return Err(value); }
        self.data[self.len].write(value);
        self.len += 1;
        Ok(())
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        Some(unsafe { self.data[self.len].assume_init_read() })
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Fixed-size array | `int array` (heap via GC) | `[T; N]` (stack frame) |
| Inline string | no native type | `InlineStr<CAP>` (`[u8; CAP]` + `usize`) |
| Stack vector | manual array + ref counter | `ArrayVec<T, CAP>` with `MaybeUninit` |
| Matrix | `float array array` (boxed) | `[[f32; 4]; 4]` (flat stack block) |
| Capacity failure | runtime exception | `Err(T)` return value |

## Key Insights

1. **Stack vs GC heap.** OCaml allocates almost everything on its GC heap; the compiler only keeps small unboxed values (int, bool, float in local position) on the stack. Rust's `[T; N]` is *always* stack-allocated when used as a local variable, giving the programmer explicit control.

2. **`MaybeUninit` enables safe, un-defaulted stack storage.** `ArrayVec` uses `[MaybeUninit<T>; CAP]` so `T` need not implement `Default`. OCaml arrays always require an initial fill value (`Array.make n v`), which forces a heap write per element.

3. **Const generics encode capacity in the type.** `InlineStr<32>` and `ArrayVec<i32, 8>` carry their capacity as a compile-time constant, enabling zero-cost fixed-size storage while preserving type safety. OCaml has no equivalent — array sizes are runtime values.

4. **`Drop` is deterministic.** Rust's `ArrayVec::drop` runs immediately when the variable goes out of scope, calling each element's destructor in turn. OCaml relies on GC finalisation, which may never run or may run much later.

5. **Cache locality.** A 4×4 `[[f32; 4]; 4]` is 64 contiguous bytes — a single cache line. The equivalent OCaml `float array array` is a boxed array of pointers to boxed inner arrays, scattering data across the heap and causing multiple cache misses per row access.

## When to Use Each Style

**Use `[T; N]` (stack array) when:** the size is known at compile time and small (rule of thumb: under 4 KB). Examples: fixed-size buffers, small matrices, SIMD-friendly data.

**Use `ArrayVec<T, CAP>` when:** you need push/pop semantics but the maximum number of elements is statically bounded and small. Examples: accumulating results in an inner loop without heap allocation, building small lists in embedded or latency-sensitive code.

**Use `Vec<T>` / `String` (heap) when:** size is dynamic, potentially large, or you need to return the collection from a function without copying — the heap is the right tool for unbounded growth.
