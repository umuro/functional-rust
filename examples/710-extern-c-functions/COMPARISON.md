# OCaml vs Rust: Calling C Functions with `extern "C"`

## Side-by-Side Code

### OCaml — external declaration
```ocaml
(* OCaml binds to C symbols with `external`. *)
external c_add : int -> int -> int = "c_add"
external c_abs : int -> int        = "c_abs"
external c_max : int -> int -> int = "c_max"

let () =
  Printf.printf "c_add(3, 4)   = %d\n" (c_add 3 4);
  Printf.printf "c_abs(-7)     = %d\n" (c_abs (-7));
  Printf.printf "c_max(10, 20) = %d\n" (c_max 10 20)
```

### Rust — `extern "C"` declaration
```rust
use std::os::raw::c_int;

extern "C" {
    fn c_add(a: c_int, b: c_int) -> c_int;
    fn c_abs(n: c_int) -> c_int;
    fn c_max(a: c_int, b: c_int) -> c_int;
}
```

### Rust — safe wrappers (idiomatic boundary pattern)
```rust
pub fn safe_add(a: i32, b: i32) -> i32 {
    // SAFETY: no pointers, no aliasing, no UB for any i32 pair.
    unsafe { ffi::c_add(a, b) }
}

pub fn safe_clamp(n: i32, lo: i32, hi: i32) -> Option<i32> {
    if lo > hi { return None; }
    // SAFETY: lo <= hi validated above.
    Some(unsafe { ffi::c_clamp(n, lo, hi) })
}
```

### Rust — C-side simulation (`#[no_mangle]`)
```rust
#[no_mangle]
pub extern "C" fn c_add(a: c_int, b: c_int) -> c_int { a + b }

#[no_mangle]
pub extern "C" fn c_abs(n: c_int) -> c_int { n.abs() }

#[no_mangle]
pub extern "C" fn c_max(a: c_int, b: c_int) -> c_int { a.max(b) }
```

---

## Type Signatures

| Concept              | OCaml                                  | Rust                                      |
|----------------------|----------------------------------------|-------------------------------------------|
| FFI declaration      | `external c_add : int -> int -> int = "c_add"` | `extern "C" { fn c_add(a: c_int, b: c_int) -> c_int; }` |
| C integer type       | `int` (OCaml int, not C int!)          | `c_int` = `i32` on all common platforms   |
| Unsafe marker        | Implicit — OCaml trusts the stub       | Explicit `unsafe {}` block at every call  |
| Safe wrapper         | Optional (OCaml has no `unsafe`)       | Idiomatic — isolate `unsafe` in one place |
| Export for C         | Automatic via C stub generator         | `#[no_mangle] pub extern "C" fn`          |

---

## Key Insights

1. **Declaration syntax**: OCaml uses `external name : type = "c_symbol"` — a single line that names the OCaml identifier, gives its type, and maps it to the C symbol name. Rust separates these concerns: `extern "C" { fn name(...); }` declares the import, and the linker resolves the symbol name.

2. **Safety model**: OCaml has no `unsafe` keyword — calling C via `external` is syntactically identical to calling OCaml code, leaving safety verification entirely to the programmer. Rust makes the danger explicit: every call through an `extern "C"` declaration requires an `unsafe {}` block, forcing you to document why each call is sound.

3. **Type mismatch risk**: OCaml's `int` is a 63-bit tagged integer (not a C `int`), so naive type-pun between OCaml `int` and C `int` is a bug; OCaml's C stub system handles the conversion automatically. Rust's `c_int` is exactly C's `int` (an alias for `i32` on all mainstream platforms), so the mapping is direct and explicit.

4. **Safe wrapper pattern**: Rust idiom is to wrap every `extern "C"` call in a safe public function that validates preconditions before crossing the boundary. OCaml achieves the same with regular functions that check arguments, but there is no language-level distinction between the unsafe FFI call and the validation wrapper.

5. **Self-contained testing**: This example uses `#[no_mangle] pub extern "C" fn` to implement the "C library" side in Rust with C calling convention, so no C compiler is needed. The linker resolves the `extern "C"` declarations to the `#[no_mangle]` definitions within the same binary — the same mechanism that resolves them to a real `.so` in production.

---

## When to Use Each Style

**Use `extern "C"` with safe wrappers when:** integrating OS APIs, database drivers, cryptographic libraries, or any existing C/C++ codebase — the safe wrapper is the idiomatic Rust boundary.

**Use `#[no_mangle] pub extern "C" fn` when:** you need Rust code to be *callable* from C, Python (via ctypes), or other languages — you are the library author, not the consumer.
