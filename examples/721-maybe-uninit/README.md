# MaybeUninit — Safe Uninitialized Memory
**Difficulty:** ⭐  
**Category:** Functional Programming  


> **Functional Rust** · [hightechmind.io](https://hightechmind.io)

## Problem Statement

Rust guarantees that every value of type `T` is valid before it can be read. Enforcing
this guarantee at zero cost requires a mechanism to hold memory that may not yet contain
a valid `T`—without triggering undefined behavior (UB) from reading an uninitialized
value. Before `MaybeUninit<T>` (stabilized in Rust 1.36), the only option was unsafe
pointer tricks that were easy to misuse and triggered UB under Miri.

The problem surfaces in three real scenarios: (1) initializing a large array
incrementally without a default value, (2) reading out-parameters from C FFI where the
callee writes the value, and (3) building collections that grow element-by-element
without requiring `T: Default`. `MaybeUninit<T>` is a union of `T` and `u8`; it
reserves space and alignment for `T` but tells the compiler that the bytes may be
garbage, preventing any optimization that assumes validity.

## Learning Outcomes

- Explain why reading uninitialized memory is undefined behavior in LLVM IR
- Use `MaybeUninit<T>::uninit()`, `write()`, and `assume_init()` correctly
- Initialize arrays element-by-element without `T: Default`
- Model C out-parameter FFI patterns safely with `MaybeUninit`
- Use `MaybeUninit::uninit_array()` for stack-allocated uninitialized arrays

## Rust Application

```rust
use std::mem::MaybeUninit;

// Safe pattern: write before assume_init
fn fill_value<T: Copy>(val: T, n: usize) -> Vec<T> {
    let mut v: Vec<MaybeUninit<T>> = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(MaybeUninit::new(val));
    }
    // SAFETY: every element was initialized by MaybeUninit::new above
    unsafe {
        let ptr = v.as_mut_ptr() as *mut T;
        let len = v.len();
        std::mem::forget(v);
        Vec::from_raw_parts(ptr, len, len)
    }
}

// C out-parameter pattern
extern "C" {
    fn c_get_value(out: *mut i32) -> i32; // returns error code
}

fn safe_get_value() -> Result<i32, i32> {
    let mut out = MaybeUninit::<i32>::uninit();
    // SAFETY: c_get_value writes *out before returning 0
    let err = unsafe { c_get_value(out.as_mut_ptr()) };
    if err == 0 {
        // SAFETY: err == 0 guarantees the callee initialized out
        Ok(unsafe { out.assume_init() })
    } else {
        Err(err)
    }
}

// Stack array without Default
fn init_array() -> [u32; 8] {
    let mut arr: [MaybeUninit<u32>; 8] = MaybeUninit::uninit_array();
    for (i, slot) in arr.iter_mut().enumerate() {
        slot.write(i as u32 * i as u32);
    }
    // SAFETY: all 8 slots written above
    unsafe { MaybeUninit::array_assume_init(arr) }
}
```

The critical discipline: every code path that reaches `assume_init()` must have
previously called `write()` on that slot. Miri will catch violations.

## OCaml Approach

OCaml does not expose uninitialized memory at the language level. The runtime
initializes every allocated block to a valid value (usually `0` or a tagged int).
The closest analog is an `option` array filled with `None` and then populated:

```ocaml
(* Safe but heap-allocates option wrappers *)
let init_partial n f =
  let arr = Array.make n None in
  for i = 0 to n - 1 do
    arr.(i) <- Some (f i)
  done;
  Array.map Option.get arr   (* panics if any slot is still None *)
```

For FFI out-parameters, OCaml uses `Bigarray` or `Bytes.create` to allocate a
writable buffer and passes it to C; the result is read back via safe accessors.
There is no direct equivalent of `assume_init` — OCaml's GC ensures all values
remain tagged.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Uninitialized memory | `MaybeUninit<T>`, explicit init | Not exposed; GC initializes all |
| Array without Default | `uninit_array` + `assume_init` | `Array.make n dummy` or `option` |
| C out-parameters | `MaybeUninit::as_mut_ptr()` | `Bigarray` or `Bytes` buffer |
| Safety enforcement | Miri detects UB at test time | Runtime checks, no UB concept |
| Performance | Zero overhead vs initialized | Small overhead for `None` boxing |

## Exercises

1. Implement `collect_uninit<T, I: Iterator<Item=T>>(iter: I) -> Vec<T>` using
   `MaybeUninit` and `Vec::with_capacity` to avoid the double-initialization that
   `Vec::new()` + `push` performs.
2. Write a safe wrapper around a C function `int compute(double *out_a, double *out_b)`
   that fills two out-parameters. Return `(f64, f64)` without intermediate allocation.
3. Implement a fixed-size ring buffer `RingBuf<T, const N: usize>` backed by
   `[MaybeUninit<T>; N]` with correct `Drop` that only drops initialized slots.
4. Use Miri (`cargo +nightly miri test`) to verify your `assume_init` calls are safe.
   Introduce a deliberate bug (read before write) and confirm Miri catches it.
5. Benchmark `fill_value` above vs `vec![val; n]`. Explain any difference in terms of
   memset vs element-by-element initialization.
