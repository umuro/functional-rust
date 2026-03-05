# OCaml vs Rust: `std::mem::transmute` — Reinterpreting Bytes

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: Obj.magic is the transmute equivalent — equally dangerous.
   Always prefer typed conversions. *)

(** Float to bits — idiomatic safe way. *)
let float_to_bits (f : float) : int64 = Int64.bits_of_float f
let bits_to_float (b : int64) : float = Int64.float_of_bits b

(** int32 byte view via Bytes — safe byte manipulation. *)
let int32_to_bytes_le (n : int32) : bytes =
  let b = Bytes.create 4 in
  Bytes.set_int32_le b 0 n;
  b

let () =
  let pi = Float.pi in
  Printf.printf "pi bits: 0x%Lx\n" (float_to_bits pi);
  let bytes = int32_to_bytes_le 0x12345678l in
  Bytes.iter (fun c -> Printf.printf "%02x " (Char.code c)) bytes
```

### Rust (safe — always prefer these)
```rust
// IEEE-754 bits of f32 — named safe API, no unsafe needed
fn f32_bits(f: f32) -> u32 { f.to_bits() }
fn f32_from_bits(bits: u32) -> f32 { f32::from_bits(bits) }

// &str byte view — named safe API
fn str_bytes(s: &str) -> &[u8] { s.as_bytes() }

// Explicit byte serialisation — portable and endian-aware
fn u32_to_le(n: u32) -> [u8; 4] { n.to_le_bytes() }
```

### Rust (transmute — only where no safe API exists)
```rust
use std::mem;

// [f32; 4] → [u32; 4]: no std API for whole-array reinterpretation
fn f32x4_to_bits_transmute(arr: [f32; 4]) -> [u32; 4] {
    // SAFETY: [f32; 4] and [u32; 4] have identical size (16 bytes) and
    // alignment (4). Every [u32; 4] bit pattern is valid.
    unsafe { mem::transmute::<[f32; 4], [u32; 4]>(arr) }
}

// #[repr(C)] struct ↔ [u8; 4]: sound because layout is fully specified
fn rgba_to_bytes_transmute(c: Rgba) -> [u8; 4] {
    // SAFETY: Rgba is #[repr(C)] with four u8 fields — size 4, align 1.
    // [u8; 4] has no validity invariants.
    unsafe { mem::transmute::<Rgba, [u8; 4]>(c) }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Float → bits (safe) | `Int64.bits_of_float : float -> int64` | `f32::to_bits : f32 -> u32` |
| Bits → float (safe) | `Int64.float_of_bits : int64 -> float` | `f32::from_bits : u32 -> f32` |
| Byte reinterpretation (unsafe) | `Obj.magic : 'a -> 'b` | `mem::transmute<T, U>(val: T) -> U` |
| Compile-time size check | runtime mismatch → silent corruption | compile error if `size_of::<T>() != size_of::<U>()` |
| Array element reinterpretation | `Array.map Int64.bits_of_float` | `arr.map(f32::to_bits)` (safe) or transmute (whole array) |
| Struct → bytes | `Bytes.create` + field writes | `rgba_to_bytes_safe` (fields) or transmute with `#[repr(C)]` |

## Key Insights

1. **Same nuclear option, different safety models.** Both `Obj.magic` (OCaml) and `mem::transmute` (Rust) bypass the type system entirely. OCaml surfaces this as a runtime risk with no compile-time guardrails; Rust gates it behind `unsafe {}`, forcing an explicit proof at the call site.

2. **Rust enforces size equality at compile time.** `transmute::<f32, u64>` is a compile error because the sizes differ (4 vs 8 bytes). OCaml's `Obj.magic` has no such check — silent heap corruption is possible.

3. **Clippy correctly warns when a safe API already exists.** `transmute::<f32, u32>` triggers `unnecessary_transmutes` because `f32::to_bits()` does the same thing safely. The examples here therefore use transmute only where no named safe alternative exists: whole-array type changes (`[f32; 4]` → `[u32; 4]`) and `#[repr(C)]` struct↔bytes conversions.

4. **`// SAFETY:` comments are mandatory.** Every `unsafe { mem::transmute(...) }` must be accompanied by a comment proving that alignment, size, validity invariants, and lifetimes are upheld. This is the programmer's written proof obligation — it makes code review and audits tractable.

5. **`#[repr(C)]` is what makes struct transmutes sound.** Without it, the compiler may reorder or pad fields freely; the transmute could read uninitialised padding bytes. OCaml records have runtime metadata making `Obj.magic` even more hazardous — the GC can misinterpret a transmuted value's tag and corrupt the heap.

## When to Use Each Style

**Use safe Rust APIs when:** inspecting float bit patterns (`to_bits`/`from_bits`), converting slices (`as_bytes`), serialising integers (`to_le_bytes`/`to_be_bytes`) — these cover 99 % of real use cases and are zero-cost at compile time.

**Use `transmute` when:** reinterpreting a whole fixed-size array of a primitive type (e.g. for SIMD preparation), working with a `#[repr(C)]` struct in an FFI context, or implementing a low-level runtime primitive where no safe abstraction yet exists — always document with a `// SAFETY:` proof and benchmark whether the safe alternative is actually slower.
