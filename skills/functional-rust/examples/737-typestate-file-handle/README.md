# 737: File Handle Typestate: Open/Closed/ReadOnly

**Difficulty:** 4  **Level:** Expert

Encode file permissions — Closed, ReadWrite, ReadOnly — in the type system so that writing to a read-only handle or reading from a closed one is a compile error, not a runtime exception.

## The Problem This Solves

File handles have permissions. Writing to a read-only file, reading from a closed handle, or closing a handle that was already closed are all bugs. In most languages these are runtime errors: `EBADF`, `PermissionError`, or `IOException`. You write defensive guards at every call site, and the guards still miss edge cases.

In Rust, you can encode the handle's current permission in its type. A `FileHandle<ReadOnly>` has a `read_to_string()` method but no `write_all()`. A `FileHandle<Closed>` has neither — you can't accidentally read from it because the method simply doesn't exist on that type.

This pattern appears in real Rust APIs: `std::fs::File` distinguishes open/close state; `BufWriter` tracks whether you've flushed; database transaction wrappers distinguish started/committed/rolled-back. Understanding the pattern lets you design APIs that eliminate whole classes of bugs.

## The Intuition

Three marker types — `Closed`, `ReadWrite`, `ReadOnly` — encode the current access mode. `FileHandle<Mode>` is generic over the mode. Methods are defined on specific instantiations:
- `write_all()` only on `FileHandle<ReadWrite>`
- `into_readonly()` — `ReadWrite` → `ReadOnly` (permission reduction, irreversible)
- `close()` on both `ReadWrite` and `ReadOnly` → returns `FileHandle<Closed>`
- `read_to_string()` on both `ReadWrite` and `ReadOnly` (both can read)

The downgrade from `ReadWrite` to `ReadOnly` is a consuming operation: you can't write anymore, and the type makes that permanent. No runtime flag, no mutex, no option — the permission is structurally encoded.

## How It Works in Rust

```rust
use std::marker::PhantomData;

pub struct Closed;
pub struct ReadWrite;
pub struct ReadOnly;

pub struct FileHandle<Mode> {
    path:    String,
    content: Vec<u8>,
    pos:     usize,
    _mode:   PhantomData<Mode>,  // zero bytes — only affects type checking
}

impl FileHandle<Closed> {
    pub fn new(path: impl Into<String>) -> Self { /* ... */ }

    // Open read-write — Closed → ReadWrite
    pub fn open_rw(self) -> io::Result<FileHandle<ReadWrite>> { /* ... */ }

    // Open read-only with initial content — Closed → ReadOnly
    pub fn open_ro(self, initial: Vec<u8>) -> io::Result<FileHandle<ReadOnly>> { /* ... */ }
}

impl FileHandle<ReadWrite> {
    pub fn write_all(&mut self, data: &[u8]) -> io::Result<()> { /* ... */ }
    pub fn read_to_string(&mut self) -> io::Result<String> { /* ... */ }

    // Permission downgrade: ReadWrite → ReadOnly (consuming, irreversible)
    pub fn into_readonly(self) -> FileHandle<ReadOnly> { /* ... */ }

    // Close: ReadWrite → Closed
    pub fn close(self) -> FileHandle<Closed> { /* ... */ }
}

impl FileHandle<ReadOnly> {
    pub fn read_to_string(&mut self) -> io::Result<String> { /* ... */ }
    // No write_all() here — ReadOnly genuinely cannot write
    pub fn close(self) -> FileHandle<Closed> { /* ... */ }
}

// ── Valid lifecycle ────────────────────────────────────────────────────────────
let handle = FileHandle::<Closed>::new("notes.txt");
let mut rw = handle.open_rw()?;
rw.write_all(b"Hello, typestate!")?;
let content = rw.read_to_string()?;

let mut ro = rw.into_readonly();     // type changes; write_all() vanishes
let _re_read = ro.read_to_string()?; // still readable
let _closed = ro.close();

// ── Compile errors for permission violations ────────────────────────────────
// ro.write_all(b"forbidden");  // error: no method `write_all` on FileHandle<ReadOnly>
// _closed.read_to_string();    // error: no method `read_to_string` on FileHandle<Closed>
```

## What This Unlocks

- **Permission-safe file APIs** — impossible to write to a read-only handle or read from a closed one; the error is in the IDE before you even compile.
- **Capability downgrade** — `into_readonly()` is a permanent one-way transition; once downgraded, write access is structurally gone, not just runtime-checked.
- **Generalises beyond files** — database connections with `Disconnected`/`InTransaction`/`Committed` states, network sockets, hardware peripherals — any resource with mode-restricted operations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| File permission mode | Runtime `open_flag` (Unix); runtime exception on violation | Compile-time phantom type — wrong-mode method doesn't exist |
| Read-only enforcement | `In_channel` vs `Out_channel` separate types | Unified `FileHandle<Mode>` with mode-specific methods |
| Permission downgrade | Change runtime flags; risk of misuse | `into_readonly()` — consuming; write methods disappear from type |
| Close enforcement | No enforcement; double-close is silent | `Closed` type has no read/write methods; compiler prevents misuse |
