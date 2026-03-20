📖 **[View on hightechmind.io →](https://hightechmind.io/rust/737-typestate-file-handle)**

---

# 737-typestate-file-handle — Typestate File Handle
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

File handles have permission semantics: writing to a read-only file, or reading from a write-only file, are runtime errors in most systems. The standard library's `File` type catches these at runtime via OS error codes. The typestate pattern promotes permission violations to compile-time errors: `FileHandle<ReadOnly>` simply does not implement `write_all`, so calling it is caught by the type checker before the program runs. This approach is used in embedded HAL (hardware abstraction layer) crates to prevent writing to read-only hardware registers.

## Learning Outcomes

- Encode file permissions (`Closed`, `ReadWrite`, `ReadOnly`) as phantom type parameters
- Implement `write_all` only for `FileHandle<ReadWrite>` and `read_to_string` for both readable modes
- Transition between modes by consuming the handle: `open_rw` and `open_ro` return different types
- Combine typestate with `io::Result` for error propagation without losing permission information
- See how `close()` consumes any open handle and returns a `FileHandle<Closed>`

## Rust Application

`FileHandle<Mode>` holds a `path`, in-memory `content: Vec<u8>`, and `pos: usize`. `impl FileHandle<Closed>` exposes `open_rw` and `open_ro`. `impl FileHandle<ReadWrite>` exposes `write_all` and `read_all`. Both `ReadWrite` and `ReadOnly` can call `read_to_string`. `close()` is generic over all open modes and returns `FileHandle<Closed>`, preventing any further use. The `_mode: PhantomData<Mode>` field carries no runtime bytes.

## OCaml Approach

OCaml's `open_in` and `open_out` return distinct `in_channel` and `out_channel` types — a simpler but less composable form of permission encoding. More expressive permission systems use GADTs or phantom type variables with abstract module signatures. The `Bos` (Basic OS) library uses a similar approach for file system operations with typed path permissions.

## Key Differences

1. **Standard library**: OCaml's stdlib uses separate `in_channel`/`out_channel` types (a coarser approach); Rust's `File` uses runtime `OpenOptions` flags.
2. **Composability**: Rust's single `FileHandle<Mode>` type with phantom parameter is more composable than OCaml's two separate types when adding new modes.
3. **Upgrades**: Rust typestate prevents upgrading a `ReadOnly` to `ReadWrite` without going through `Closed`; OCaml achieves the same via module abstraction.
4. **Embedded**: Rust's approach is widely used in `embedded-hal` for GPIO pin modes (`Input`, `Output`, `Alternate`); OCaml has no equivalent embedded ecosystem.

## Exercises

1. Add a `WriteOnly` mode that supports `write_all` but not `read_all`, and implement `open_wo` on `FileHandle<Closed>`.
2. Implement `seek(&mut self, pos: usize)` on `FileHandle<ReadWrite>` and `FileHandle<ReadOnly>` using a shared trait `Seekable`.
3. Write a function `copy<Src, Dst>(src: &mut FileHandle<Src>, dst: &mut FileHandle<Dst>)` constrained to only work when `Src: Readable` and `Dst: Writable`.
