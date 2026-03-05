# OCaml vs Rust: Address of a Variable

## Side-by-Side Code

### OCaml

```ocaml
(* OCaml: only boxed (heap) values have an observable address *)
let address_of (x : 'a) : nativeint =
  if Obj.is_block (Obj.repr x) then
    Nativeint.shift_left (Nativeint.of_int (Obj.magic x)) 1
  else
    invalid_arg "Can only find address of boxed values."

let () =
  let a = 3.14 in
  Printf.printf "%nx\n" (address_of a);   (* ok — float is boxed  *)
  let b = ref 42 in
  Printf.printf "%nx\n" (address_of b);   (* ok — ref is boxed    *)
  let c = 17 in
  Printf.printf "%nx\n" (address_of c)    (* raises Invalid_argument — int is immediate *)
```

### Rust (idiomatic)

```rust
// Rust: every value — stack or heap — has a real memory address
pub fn address_of<T>(x: &T) -> usize {
    std::ptr::addr_of!(*x) as usize
}

fn main() {
    let a: f64 = 3.14;
    println!("0x{:x}", address_of(&a));   // ok — stack

    let b: Box<i32> = Box::new(42);
    println!("0x{:x}", address_of(&*b));  // ok — heap data

    let c: i32 = 17;
    println!("0x{:x}", address_of(&c));   // ok — plain i32 also addressable!
}
```

### Rust (functional / helper style)

```rust
// Predicate: are two values stored at different memory locations?
pub fn addresses_differ<T>(a: &T, b: &T) -> bool {
    (std::ptr::addr_of!(*a) as usize) != (std::ptr::addr_of!(*b) as usize)
}

fn demonstrate() {
    let x: i32 = 1;
    let y: i32 = 2;
    assert!(addresses_differ(&x, &y));

    let r = &x;           // reborrow — same address
    assert!(!addresses_differ(&x, r));
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Address function | `val address_of : 'a -> nativeint` | `fn address_of<T>(x: &T) -> usize` |
| Address type | `nativeint` | `usize` (or `*const T`) |
| Input type | any value (runtime check) | `&T` (reference; borrow checked) |
| Heap pointer | `Obj.magic x` shifted left | `std::ptr::addr_of!(*x) as usize` |
| Safety | runtime `invalid_arg` for immediates | compile-time: all `&T` are valid |

## Key Insights

1. **Immediate vs. boxed is an OCaml-only concern.** OCaml's GC tags low bits of
   word-size values to distinguish heap pointers from unboxed integers. Rust has
   no such distinction — every `i32`, `f64`, or struct lives at a real address.

2. **`addr_of!` vs. `&` then cast.** `&x as *const T as usize` works but
   momentarily creates a reference (which requires `x` to be aligned and
   initialised). `std::ptr::addr_of!(x)` skips the reference, making it safe for
   `#[repr(packed)]` fields where creating a reference would be UB.

3. **GC mobility vs. lifetime pinning.** OCaml's minor GC can *move* heap objects
   during a collection, so an address obtained before a GC cycle may be stale.
   Rust values are pinned to their location for the duration of the borrow: the
   compiler guarantees the address remains valid for `'lifetime`.

4. **`usize` is the idiomatic address integer.** Rust uses `usize` (pointer-width
   unsigned integer) rather than OCaml's `nativeint`, which maps directly to C's
   `intptr_t`. Both are pointer-sized, but `usize` is unsigned by convention.

5. **No `unsafe` needed to *print* an address.** Converting `&T → *const T → usize`
   is safe Rust. `unsafe` is only required to *dereference* a raw pointer, not to
   inspect or compare its numeric value.

## When to Use Each Style

**Use idiomatic Rust (`address_of`)** when you need to log, assert, or compare
memory locations in debugging, test infrastructure, or safety-critical code that
verifies aliasing invariants.

**Use the `addresses_differ` helper** when writing tests that assert two references
do not alias — for example, verifying that a copy-on-write implementation actually
produced a fresh allocation.
