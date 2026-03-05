# Example 1111: Address of a Variable

**Difficulty:** ⭐
**Category:** General / Memory Model
**OCaml Source:** Rosetta Code — "Address of a variable"

## Problem Statement

Print (or return) the memory address of a variable. For heap-allocated values this
is the pointer to the data; for stack values it is the frame address.

## Learning Outcomes

- Rust gives every value a stable memory address for its entire lifetime — stack or heap.
- `std::ptr::addr_of!(expr)` obtains a raw pointer without forming an intermediate reference, avoiding undefined behaviour with packed structs.
- Unlike OCaml, Rust treats unboxed integers exactly like boxed values with respect to addressing: they all live in real memory.
- Casting a raw pointer to `usize` is a safe, common pattern for inspecting or comparing addresses.

## OCaml Approach

OCaml's GC distinguishes *immediate* values (unboxed integers, booleans) from
*boxed* heap values (floats, tuples, custom blocks). `Obj.repr` exposes the
internal representation: for immediates it returns a tagged integer, not a pointer.
The Rosetta Code task therefore raises `Invalid_argument` when asked for the address
of a plain `int`.

## Rust Approach

Rust has no garbage collector and no distinction between immediate and boxed values
at the language level. Every binding occupies a location in memory (stack frame or
heap allocation), and `&x` yields a reference to that location. Converting a
reference to a raw pointer (`*const T`) and then to `usize` gives the numeric
address. `std::ptr::addr_of!` is the canonical way to do this without constructing
an intermediate reference.

## Key Differences

1. **All values are addressable:** OCaml unboxed integers have no heap address;
   Rust `i32` / `bool` / etc. always live at a concrete address.
2. **Safety model:** Rust raw-pointer arithmetic is `unsafe`; merely printing the
   address (usize) is entirely safe.
3. **GC vs. ownership:** OCaml addresses can change as the GC moves objects; Rust
   addresses are stable for the lifetime of the binding.
4. **Syntax:** `Obj.magic` + `Nativeint` in OCaml vs. `addr_of!` + `as usize` in Rust.
