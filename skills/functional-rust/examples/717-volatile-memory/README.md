# 717: Volatile Memory Reads and Writes

**Difficulty:** 4  **Level:** Expert

Use `read_volatile`/`write_volatile` to prevent the compiler from optimising away memory-mapped I/O register accesses.

## The Problem This Solves

Modern compilers are aggressive optimisers. If you write to a memory address and then write to it again without reading, the compiler may eliminate the first write — it looks dead. If you read the same address twice in a loop and the value hasn't changed from the compiler's perspective, it may cache the value in a register and skip the second read. For normal memory, these optimisations are correct. For memory-mapped I/O registers, they are catastrophically wrong.

An MMIO register is a hardware register masquerading as a memory address. Writing to the UART data register sends a byte to the serial port — even if you never read the result. Reading the status register queries the hardware — even if the compiler thinks the value is unchanged. Every access to an MMIO address has observable side effects the compiler cannot see. The compiler must not reorder, merge, or eliminate these accesses.

`std::ptr::read_volatile` and `std::ptr::write_volatile` are the solution. They tell the compiler: "this read/write is observable; treat it as if the entire outside world can see it." This is not the same as atomic operations — volatile says nothing about thread ordering. It's purely about suppressing compiler optimisations on a specific memory location.

## The Intuition

Think of volatile access as a "do not touch" label for the compiler. Normally, the compiler silently rearranges your code for efficiency, confident that programs can't tell the difference. Volatile says "I can tell the difference — every one of these reads and writes matters, in order, exactly as written." Hardware registers, `mmap`-ed files, and shared memory with other processes all have this property.

The canonical abstraction is a `VolatileCell<T>` wrapper that exposes only `read()` and `write()` methods, both implemented with the volatile intrinsics. Users of the wrapper can't accidentally use regular field access.

## How It Works in Rust

```rust
use std::ptr;

pub struct VolatileCell<T>(std::cell::UnsafeCell<T>);

impl<T: Copy> VolatileCell<T> {
    pub fn read(&self) -> T {
        unsafe {
            // SAFETY: The pointer is valid and aligned; T: Copy.
            // Volatile prevents the compiler from caching or eliminating this read.
            ptr::read_volatile(self.0.get())
        }
    }

    pub fn write(&self, val: T) {
        unsafe {
            // SAFETY: Same as above; write is not eliminated even if val is "unused".
            ptr::write_volatile(self.0.get(), val);
        }
    }
}

// MMIO register map — would live at a fixed physical address in real firmware.
pub struct UartRegisters {
    pub status: VolatileCell<u32>,
    pub data:   VolatileCell<u32>,
}
```

In real embedded code, the register map is at a fixed linker-defined address: `let uart = unsafe { &*(0x4000_1000 as *const UartRegisters) }`.

## What This Unlocks

- **Embedded firmware**: Access UART, GPIO, SPI, and timer registers correctly — the hardware sees every write, in order, with no compiler-invented omissions.
- **`mmap` file I/O**: Volatile reads ensure you see updated file contents when another process has written to the mapped region.
- **Correct spin-loops**: Polling a status register in a loop requires volatile reads — otherwise the compiler hoists the read out of the loop and spins forever on a cached value.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Volatile reads | `Bigarray` (partial workaround) | `ptr::read_volatile` |
| Volatile writes | Not available directly | `ptr::write_volatile` |
| Memory-mapped regions | `Unix.map_file` | `mmap` via libc or `/dev/mem` |
| Register abstraction | Record of functions | `struct VolatileCell<T>` |
| Compiler reordering | Not controllable | `core::sync::atomic::compiler_fence` |
| Embedded bare-metal | Not typical | `#![no_std]` + linker sections |
