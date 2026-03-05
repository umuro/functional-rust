# OCaml vs Rust: FFI Callbacks — Passing Functions to C

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: higher-order functions are first-class; no trampoline needed.
   Closures capture freely across all call sites. *)

let c_for_each (arr : int array) (f : int -> unit) : unit =
  Array.iter f arr

let c_reduce (arr : int array) (init : int) (f : int -> int -> int) : int =
  Array.fold_left f init arr

let () =
  let data = [| 1; 2; 3; 4; 5 |] in
  c_for_each data (fun x -> Printf.printf "%d " x);
  let sum = c_reduce data 0 ( + ) in
  Printf.printf "\nSum: %d\n" sum;
  let offset = 10 in
  let shifted_sum = c_reduce data 0 (fun acc v -> acc + v + offset) in
  Printf.printf "Shifted sum: %d\n" shifted_sum
```

### Rust — Pattern 1: plain function pointer (no captures)
```rust
use std::os::raw::c_void;

// A plain Rust function declared `extern "C"` coerces to the C function
// pointer type automatically — no unsafe, no boxing, no overhead.
pub extern "C" fn add(acc: i32, v: i32) -> i32 { acc + v }

pub fn sim_reduce(data: &[i32], init: i32, f: extern "C" fn(i32, i32) -> i32) -> i32 {
    data.iter().fold(init, |acc, &v| f(acc, v))
}

let sum = sim_reduce(&[1, 2, 3, 4, 5], 0, add); // 15
```

### Rust — Pattern 2: trampoline for closures with captures
```rust
// The C API accepts (callback, user_data): the function pointer is a stable
// address; user_data carries the closure's captured state as *mut c_void.
pub fn for_each_with_closure<F: FnMut(i32)>(data: &[i32], mut f: F) {
    extern "C" fn trampoline<F: FnMut(i32)>(user_data: *mut c_void, v: i32) {
        // SAFETY: user_data is &mut f from the enclosing stack frame,
        // alive for the duration of this call.
        let closure = unsafe { &mut *user_data.cast::<F>() };
        closure(v);
    }
    let user_data = (&raw mut f).cast::<c_void>();
    sim_for_each_ctx(data, trampoline::<F>, user_data);
}

let mut collected = Vec::new();
for_each_with_closure(&[10, 20, 30], |v| collected.push(v));
// collected == [10, 20, 30]
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Higher-order function | `('a -> 'b) -> 'a list -> 'b list` | `extern "C" fn(i32) -> i32` |
| Closure with captures | `fun x -> x + offset` (first-class) | `\|v\| v + offset` (trampoline needed) |
| Opaque state pointer | N/A (GC manages closure structs) | `*mut c_void` (raw, manually managed) |
| Safe wrapper | N/A (no boundary to wrap) | `pub fn for_each_with_closure<F: FnMut(i32)>` |

## Key Insights

1. **OCaml closures are always first-class**: The runtime represents every closure as a heap-allocated record with a code pointer and an environment. Any `'a -> 'b` value can be passed to any higher-order function without special syntax. There is no concept of a "plain function vs. capturing closure" split.

2. **Rust closures have no stable ABI**: A Rust closure is a compiler-generated struct whose layout depends on what it captures. C cannot call into it because C has no way to know the struct layout at compile time. Only a plain function pointer — a bare address — is C-compatible.

3. **The trampoline is the idiomatic bridge**: Instead of boxing the closure and using a vtable (expensive), the trampoline pattern passes the closure by stack address as `*mut c_void`. The thin `extern "C"` trampoline wrapper holds the code pointer; the user-data holds the state. This is zero-cost compared to virtual dispatch.

4. **`&raw mut` eliminates an intermediate reference**: `&raw mut f` creates a raw pointer without creating an intermediate Rust reference, avoiding potential aliasing UB. It is the modern preferred form over `&mut f as *mut F`.

5. **Safety is quarantined at one point**: All `unsafe` lives inside the trampoline body behind a `// SAFETY:` comment. The public API (`for_each_with_closure`) is entirely safe, and callers never see raw pointers or `unsafe` blocks. This mirrors how standard library functions like `sort_by` hide unsafe internals behind a safe interface.

## When to Use Each Style

**Use plain `extern "C" fn`** when the callback does not need captured state — comparators, pure transformations, logging hooks. Zero overhead, no `unsafe`, simplest possible FFI integration.

**Use the trampoline pattern** when the callback must accumulate results, mutate external state, or close over configuration values. Use a safe wrapper to hide the raw-pointer mechanics from callers.
