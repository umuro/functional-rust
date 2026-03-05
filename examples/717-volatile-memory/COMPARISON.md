# OCaml vs Rust: Volatile Memory Reads and Writes

## Side-by-Side Code

### OCaml (Bigarray — closest analog to volatile MMIO)

```ocaml
open Bigarray

type reg32 = (int32, int32_elt, c_layout) Array1.t

let make_mmio_region size_words : reg32 =
  Array1.create int32 c_layout size_words

let status_reg = 0
let data_reg   = 1
let ctrl_reg   = 2

let tx_ready = Int32.of_int 0x01
let rx_ready = Int32.of_int 0x02

(* Bigarray read — compiler does not cache across accesses *)
let mmio_read (regs : reg32) offset = regs.{offset}

(* Bigarray write — side-effecting, not optimised away *)
let mmio_write (regs : reg32) offset value = regs.{offset} <- value

(* Set bits: read-modify-write *)
let set_bits regs offset mask =
  let current = mmio_read regs offset in
  mmio_write regs offset (Int32.logor current mask)

let () =
  let regs = make_mmio_region 8 in
  mmio_write regs data_reg 0xDEADBEEFl;
  assert (mmio_read regs data_reg = 0xDEADBEEFl);
  set_bits regs status_reg tx_ready;
  assert (Int32.logand (mmio_read regs status_reg) tx_ready = tx_ready);
  print_endline "ok"
```

### Rust (idiomatic — safe wrapper around `write_volatile` / `read_volatile`)

```rust
use std::ptr;

pub struct MmioDevice { regs: [u32; 8] }

impl MmioDevice {
    pub fn new() -> Self { Self { regs: [0u32; 8] } }

    pub fn write(&mut self, reg: usize, val: u32) {
        assert!(reg < self.regs.len());
        unsafe { ptr::write_volatile(self.regs.as_mut_ptr().add(reg), val); }
    }

    pub fn read(&self, reg: usize) -> u32 {
        assert!(reg < self.regs.len());
        unsafe { ptr::read_volatile(self.regs.as_ptr().add(reg)) }
    }

    pub fn set_bits(&mut self, reg: usize, mask: u32) {
        let v = self.read(reg);
        self.write(reg, v | mask);
    }
}
```

### Rust (functional/recursive — standalone volatile helpers)

```rust
use std::ptr;

/// Write to an MMIO address without a wrapper type.
pub unsafe fn mmio_write(ptr: *mut u32, val: u32) {
    ptr::write_volatile(ptr, val);
}

/// Read from an MMIO address without caching.
pub unsafe fn mmio_read(ptr: *const u32) -> u32 {
    ptr::read_volatile(ptr)
}

// Poll-loop pattern: reads must not be hoisted by the optimiser.
pub fn poll_until_ready(ptr: *const u32, mask: u32) {
    loop {
        if unsafe { ptr::read_volatile(ptr) } & mask != 0 { break; }
    }
}
```

---

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| MMIO region type | `(int32, int32_elt, c_layout) Array1.t` | `[u32; 8]` (or `*mut u32`) |
| Volatile write | `regs.{offset} <- value` (Bigarray) | `ptr::write_volatile(ptr, val)` |
| Volatile read | `regs.{offset}` (Bigarray) | `ptr::read_volatile(ptr)` |
| Bit manipulation | `Int32.logor`, `Int32.logand` | `\|`, `&`, `!` (native operators) |
| Safety boundary | Runtime (GC manages all memory) | `unsafe` block with explicit invariant |

---

## Key Insights

1. **OCaml has no `volatile` keyword.** The closest analog is `Bigarray`, whose
   element accesses are backed by C-level memory and are not subject to the same
   reordering that the OCaml GC might introduce for ordinary heap values. This is
   a convention rather than a language guarantee.

2. **Rust `write_volatile` / `read_volatile` are first-class language primitives.**
   They emit a barrier to the compiler (not the CPU) that prevents the access from
   being eliminated, merged, or reordered with adjacent accesses to the same
   address. This is exactly the guarantee MMIO drivers require.

3. **`volatile` ≠ `atomic`.** Volatile suppresses compiler optimisations; atomics
   additionally provide CPU-level ordering guarantees visible to other threads.
   For single-core MMIO with no DMA, volatile alone is sufficient.

4. **Safe wrapper pattern.** The `MmioDevice` struct encapsulates the `unsafe` raw
   pointer operations behind a checked, bounds-safe API. Users call `dev.write()`
   and `dev.read()` — idiomatic Rust that is impossible to misuse at the call site.

5. **Every access must be volatile.** A single non-volatile read in a polling loop
   lets the compiler cache the value in a register and loop forever. In Rust this
   is explicit: if you forget `read_volatile` and use `*ptr` instead, the bug is
   visible in the source. In C, `volatile` is a type qualifier that propagates
   automatically through the type system — Rust makes the choice per call-site.

---

## When to Use Each Style

**Use the safe wrapper (`MmioDevice`):** for any production driver or HAL crate
where you want the `unsafe` isolated in one place and the public API to be
entirely safe.

**Use the standalone `mmio_read` / `mmio_write` helpers:** when writing low-level
board support code where you work directly with linker-provided addresses and do
not want the overhead of a struct abstraction.

**Use Bigarray (OCaml):** when prototyping driver logic in OCaml or writing tools
that parse firmware memory dumps — not for real MMIO access in production embedded
software, which is almost exclusively done in C or Rust.
