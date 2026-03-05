# OCaml vs Rust: `#[repr(C)]` Structs for FFI Interop

## Side-by-Side Code

### OCaml (via Ctypes — conceptual)

```ocaml
(* OCaml structs have no guaranteed memory layout by default.
   The Ctypes library adds explicit C-compatible layout at runtime. *)
type point2d = { x : float; y : float }
type rect = { origin : point2d; width : float; height : float }

let area (r : rect) : float = r.width *. r.height
let perimeter (r : rect) : float = 2.0 *. (r.width +. r.height)

let () =
  let r = { origin = { x = 1.0; y = 2.0 }; width = 10.0; height = 5.0 } in
  Printf.printf "Area:      %.1f\n" (area r);
  Printf.printf "Perimeter: %.1f\n" (perimeter r)
```

### Rust (idiomatic — `#[repr(C)]` layout guarantee)

```rust
use std::mem;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D { pub x: f64, pub y: f64 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect { pub origin: Point2D, pub width: f64, pub height: f64 }

pub fn rect_area(r: Rect) -> f64 { r.width * r.height }
pub fn rect_perimeter(r: Rect) -> f64 { 2.0 * (r.width + r.height) }

// Verified at compile time: size_of::<Point2D>() == 16, align_of == 8
// Matches C's: sizeof(struct { double x; double y; }) == 16
```

### Rust (FFI export surface with `extern "C"`)

```rust
/// Callable directly from C with no marshalling overhead.
#[no_mangle]
pub extern "C" fn ffi_rect_area(r: Rect) -> f64 { r.width * r.height }

/// Round-trip through raw bytes — simulates writing to / reading from C.
pub fn point2d_as_bytes(p: &Point2D) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            (p as *const Point2D).cast::<u8>(),
            mem::size_of::<Point2D>(),
        )
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| 2-D point type | `type point2d = { x : float; y : float }` | `#[repr(C)] pub struct Point2D { pub x: f64, pub y: f64 }` |
| Rectangle type | `type rect = { origin : point2d; width : float; height : float }` | `#[repr(C)] pub struct Rect { pub origin: Point2D, pub width: f64, pub height: f64 }` |
| Area function | `val area : rect -> float` | `fn rect_area(r: Rect) -> f64` |
| FFI export | Ctypes `foreign` binding | `#[no_mangle] pub extern "C" fn ...` |
| Size guarantee | Ctypes `sizeof` query at runtime | `mem::size_of::<Point2D>()` — checked at compile time |
| Memory layout | Opaque OCaml GC heap layout; Ctypes wraps it | Exact C ABI layout baked in by `#[repr(C)]` |

## Key Insights

1. **Layout is opt-in in Rust.** Without `#[repr(C)]` the compiler may reorder
   fields and insert arbitrary padding. The annotation is a public contract: the
   struct's binary shape is now fixed and documented.

2. **OCaml needs a library; Rust has it built in.** OCaml structs live on the GC
   heap with their own internal layout. FFI interop requires the `Ctypes` library
   to build shadow C-layout representations at runtime. Rust bakes the C layout
   directly into the type via an attribute — no runtime overhead, no extra
   dependency.

3. **`unsafe` is surgically scoped.** The `unsafe` block in `point2d_as_bytes`
   is narrow and justified: `Point2D` is `#[repr(C)]` with no padding, so the
   raw byte reinterpretation is sound. All safe Rust code — geometry functions,
   tests — stays entirely safe.

4. **`size_of` and field offsets are testable.** Because the layout is
   guaranteed, you can write deterministic unit tests asserting
   `mem::size_of::<Point2D>() == 16` and `offset_of x == 0, y == 8`. These
   tests catch accidental removal of `#[repr(C)]` during refactoring.

5. **`extern "C"` fixes the calling convention.** `#[repr(C)]` fixes the data
   layout; `extern "C"` fixes how arguments are passed in registers/stack; and
   `#[no_mangle]` fixes the symbol name. All three are needed for a complete,
   correct FFI boundary — each solves a different part of the ABI contract.

## When to Use Each Style

**Use `#[repr(C)]` Rust structs when:** you are building a shared library that
C/C++ code will link against, reading/writing binary file formats defined in C
headers, using `mmap`-backed structures, or wrapping a C library with
`bindgen`-generated bindings that pass structs by value.

**Use plain Rust structs (no `repr`) when:** the struct is purely internal to
your Rust codebase. The compiler's freedom to optimise layout gives better
packing, smaller stack frames, and potentially better cache behaviour — benefits
you only surrender when an external C ABI contract actually requires it.
