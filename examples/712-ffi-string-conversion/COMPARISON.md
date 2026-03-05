# OCaml vs Rust: String/CString/CStr Conversion for FFI

## Side-by-Side Code

### OCaml

```ocaml
(* OCaml: manual null-termination using Bytes *)
let to_c_string (s : string) : bytes =
  let n = String.length s in
  let b = Bytes.create (n + 1) in
  Bytes.blit_string s 0 b 0 n;
  Bytes.set b n '\000';
  b

let c_strlen (b : bytes) : int =
  let rec go i =
    if i >= Bytes.length b || Bytes.get b i = '\000' then i else go (i + 1)
  in go 0

let from_c_string (b : bytes) : string =
  Bytes.sub_string b 0 (c_strlen b)

let () =
  let s = "Hello, FFI!" in
  let cs = to_c_string s in
  assert (c_strlen cs = String.length s);
  assert (from_c_string cs = s);
  print_endline "ok"
```

### Rust (idiomatic — using `CString` / `CStr`)

```rust
use std::ffi::{CStr, CString};

// Rust → C: allocate a heap-owned, null-terminated buffer.
fn rust_to_cstring(s: &str) -> Result<CString, std::ffi::NulError> {
    CString::new(s)
}

// C → Rust: borrow the C buffer as a &CStr, then validate UTF-8.
unsafe fn ptr_to_str<'a>(ptr: *const std::os::raw::c_char) -> &'a str {
    CStr::from_ptr(ptr).to_str().expect("not valid UTF-8")
}
```

### Rust (manual / functional — mirrors the OCaml recursive strlen)

```rust
// Recursive C strlen — mirrors OCaml's `go i` accumulator pattern.
unsafe fn manual_strlen(ptr: *const u8) -> usize {
    if *ptr == 0 { 0 } else { 1 + manual_strlen(ptr.add(1)) }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Rust-owned C string | `bytes` (manual) | `CString` |
| Borrowed C string | `bytes` slice | `&CStr` |
| Raw C pointer | `'a Bigarray` / `nativeint` | `*const c_char` |
| Conversion to string | `Bytes.sub_string` | `CStr::to_str() -> Result<&str, Utf8Error>` |
| Interior NUL guard | runtime `String.contains '\000'` (manual) | `CString::new` returns `Err(NulError)` |
| UTF-8 validation | no built-in (OCaml is byte-agnostic) | `CStr::to_str()` enforces UTF-8 |

## Key Insights

1. **Ownership encodes lifetime**: In OCaml, `to_c_string` returns a `bytes` value whose lifetime is managed by the GC — there's no dangling-pointer risk. In Rust, `CString` is a heap-allocated RAII type; calling `.as_ptr()` borrows from the `CString`, so the `CString` must outlive the pointer. The compiler enforces this statically.

2. **Null-termination is a type invariant**: OCaml's `bytes` is just bytes — the programmer manually appends `'\000'`. Rust's `CString` guarantees null-termination by construction; you cannot create one without the terminator, and you cannot get a `&CStr` from bytes that aren't null-terminated.

3. **Interior NUL is a type-level error**: If the Rust string contains `'\0'`, `CString::new` returns `Err(NulError)` instead of silently truncating the C string at the first NUL — a common source of FFI security bugs.

4. **UTF-8 flows in one direction only**: OCaml strings are byte sequences without encoding guarantees. Rust `&str` is always UTF-8. When reading a `*const c_char` from C, `CStr::to_str()` validates UTF-8 and returns `Err(Utf8Error)` rather than producing a corrupted `&str`.

5. **Zero-copy on the C→Rust path**: `CStr::from_ptr` borrows the C buffer directly — no allocation, no copy. OCaml's `from_c_string` always allocates a new `string`. Rust pays for allocation only when you call `.to_owned()` or `.to_string_lossy().into_owned()`.

## When to Use Each Style

**Use `CString` / `CStr` (idiomatic Rust) when:** calling real C libraries (`libc`, system calls, C extensions). These types prevent the null-termination and UTF-8 bugs at compile time and make FFI audits easier.

**Use manual byte manipulation when:** you need precise control over the buffer layout (e.g., fixed-size stack buffers, `MaybeUninit` patterns for output parameters) or when interoperating with non-UTF-8 encodings where `CStr::to_str()` would always fail and `to_string_lossy()` better reflects intent.
