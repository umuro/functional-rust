#![allow(clippy::all)]
//! 722 — Memory Layout: repr(C), repr(packed), repr(align(N))
//!
//! Controls struct field order, padding, and alignment for FFI,
//! serialisation, and SIMD work. The three attributes let you move from
//! "compiler decides" to "you decide" when byte-exact layout matters.

use std::mem;

// ── repr(Rust) — default, compiler-chosen layout ─────────────────────────────

/// Default layout: the compiler may reorder fields to minimise padding.
///
/// For `{u8, u32, u16}` the compiler typically produces 8 bytes by placing
/// `b: u32` first, then `c: u16`, then `a: u8` with 1 pad byte — instead of
/// the 12-byte C layout. Field order is NOT guaranteed across compiler versions.
///
/// Do NOT use for FFI or when byte layout must be stable.
pub struct DefaultLayout {
    pub a: u8,
    pub b: u32,
    pub c: u16,
}

// ── repr(C) — C-compatible, declaration order ─────────────────────────────────

/// Fields in declaration order with C padding rules.
///
/// Memory map:
/// - `a: u8`  at offset 0 (1 byte)
/// - 3 bytes padding (align b to 4)
/// - `b: u32` at offset 4 (4 bytes)
/// - `c: u16` at offset 8 (2 bytes)
/// - 2 bytes trailing padding (struct align = 4)
/// - Total: 12 bytes, align 4
#[repr(C)]
pub struct CLayout {
    pub a: u8,
    pub b: u32,
    pub c: u16,
}

// ── repr(C, packed) — C order, no padding ────────────────────────────────────

/// All padding stripped; every field is byte-adjacent.
///
/// Memory map:
/// - `a: u8`  at offset 0 (1 byte)
/// - `b: u32` at offset 1 (4 bytes, unaligned!)
/// - `c: u16` at offset 5 (2 bytes, unaligned!)
/// - Total: 7 bytes, align 1
///
/// # Safety invariant
/// Taking a Rust reference (`&h.b`) to an unaligned field is **undefined
/// behaviour**. Always use `std::ptr::addr_of!(h.b)` to obtain a raw pointer,
/// then `ptr::read_unaligned` to copy the value.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PackedLayout {
    pub a: u8,
    pub b: u32,
    pub c: u16,
}

// ── repr(C, align(N)) — forced alignment ─────────────────────────────────────

/// 16-byte aligned struct for SIMD loads (SSE requires 16-byte alignment).
///
/// Four `f32` fields already fill 16 bytes, so `align(16)` only forces the
/// struct's minimum alignment — no extra padding is added here.
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimdVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// 64-byte aligned counter to occupy exactly one cache line.
///
/// Placing hot fields in separate cache lines prevents false sharing between
/// threads: two CPUs writing different counters do not invalidate each other's
/// cache lines.
#[repr(C, align(64))]
pub struct CacheLinePadded {
    pub counter: u64,
    // 56 bytes of compiler-inserted padding bring the size to 64
}

// ── Wire-format packet header (repr(C, packed)) ───────────────────────────────

/// Simulated network packet header: exactly 7 bytes on the wire, no padding.
///
/// This is the canonical use of `repr(C, packed)`: you own the byte layout
/// because it is defined by a protocol, not by the CPU's alignment needs.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketHeader {
    pub magic: u8,
    pub length: u32,
    pub checksum: u16,
}

// ── Safe accessors for packed-struct fields ───────────────────────────────────

/// Read `PacketHeader.length` without creating an unaligned reference.
///
/// `addr_of!` produces a raw pointer without going through a reference, so
/// there is no aliasing rule violation. `read_unaligned` copies the bytes.
pub fn packet_length(h: &PacketHeader) -> u32 {
    // SAFETY: `length` may be at offset 1 (unaligned for u32).
    // `addr_of!` never creates a reference; `read_unaligned` handles the copy.
    unsafe { std::ptr::read_unaligned(std::ptr::addr_of!(h.length)) }
}

/// Read `PacketHeader.checksum` without creating an unaligned reference.
pub fn packet_checksum(h: &PacketHeader) -> u16 {
    // SAFETY: same as packet_length — unaligned field read via raw pointer.
    unsafe { std::ptr::read_unaligned(std::ptr::addr_of!(h.checksum)) }
}

// ── Serialise / deserialise ───────────────────────────────────────────────────

/// Serialise a `PacketHeader` to its 7 wire bytes.
pub fn packet_to_bytes(h: &PacketHeader) -> [u8; 7] {
    let mut buf = [0u8; 7];
    // SAFETY: PacketHeader is repr(C, packed); size_of is exactly 7.
    // We copy the raw bytes; no references to unaligned fields are created.
    unsafe {
        std::ptr::copy_nonoverlapping(
            (h as *const PacketHeader).cast::<u8>(),
            buf.as_mut_ptr(),
            mem::size_of::<PacketHeader>(),
        );
    }
    buf
}

/// Deserialise a `PacketHeader` from 7 raw bytes.
pub fn packet_from_bytes(bytes: &[u8; 7]) -> PacketHeader {
    // SAFETY: PacketHeader is repr(C, packed) with no invalid bit patterns
    // for its field types (u8, u32, u16). Length is verified by type system.
    unsafe { std::ptr::read_unaligned(bytes.as_ptr().cast::<PacketHeader>()) }
}

// ── Layout query helpers ──────────────────────────────────────────────────────

/// Returns `(size_of::<T>(), align_of::<T>())`.
pub fn layout_of<T>() -> (usize, usize) {
    (mem::size_of::<T>(), mem::align_of::<T>())
}

// ── SIMD-style dot product (works on aligned data) ───────────────────────────

/// Dot product of two 4-component vectors.
///
/// Operates on `SimdVec4`, which is guaranteed to be 16-byte aligned — a
/// precondition for SSE/NEON SIMD intrinsics in real code.
pub fn dot(a: SimdVec4, b: SimdVec4) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Size and alignment ────────────────────────────────────────────────────

    #[test]
    fn default_layout_is_smaller_than_c_layout() {
        // The compiler reorders fields to avoid padding; C layout cannot.
        let default_size = mem::size_of::<DefaultLayout>();
        let c_size = mem::size_of::<CLayout>();
        assert!(
            default_size <= c_size,
            "default({default_size}) should be ≤ C({c_size})"
        );
    }

    #[test]
    fn c_layout_has_c_padding() {
        // a(1) + pad(3) + b(4) + c(2) + pad(2) = 12 bytes, align 4
        let (size, align) = layout_of::<CLayout>();
        assert_eq!(size, 12, "CLayout must be 12 bytes");
        assert_eq!(align, 4, "CLayout must be 4-byte aligned");
    }

    #[test]
    fn packed_layout_has_no_padding() {
        // 1 + 4 + 2 = 7 bytes, align 1
        let (size, align) = layout_of::<PackedLayout>();
        assert_eq!(size, 7, "PackedLayout must be 7 bytes");
        assert_eq!(align, 1, "PackedLayout must be 1-byte aligned");
    }

    #[test]
    fn simd_vec4_is_16_byte_aligned() {
        let (size, align) = layout_of::<SimdVec4>();
        assert_eq!(size, 16, "SimdVec4 must be 16 bytes (4 × f32)");
        assert_eq!(align, 16, "SimdVec4 must be 16-byte aligned for SIMD");
    }

    #[test]
    fn cache_line_padded_fills_one_cache_line() {
        let (size, align) = layout_of::<CacheLinePadded>();
        assert_eq!(size, 64, "CacheLinePadded must occupy exactly 64 bytes");
        assert_eq!(align, 64, "CacheLinePadded must be 64-byte aligned");
    }

    #[test]
    fn packet_header_size_is_seven() {
        // 1 + 4 + 2 = 7 bytes on the wire
        assert_eq!(mem::size_of::<PacketHeader>(), 7);
        assert_eq!(mem::align_of::<PacketHeader>(), 1);
    }

    // ── Field offsets ─────────────────────────────────────────────────────────

    #[test]
    fn c_layout_field_offsets() {
        // repr(C) guarantees: a@0, b@4 (C padding), c@8
        let v = CLayout { a: 0, b: 0, c: 0 };
        let base = &v as *const CLayout as usize;
        assert_eq!(
            &v.a as *const u8 as usize - base,
            0,
            "a must be at offset 0"
        );
        assert_eq!(
            &v.b as *const u32 as usize - base,
            4,
            "b must be at offset 4"
        );
        assert_eq!(
            &v.c as *const u16 as usize - base,
            8,
            "c must be at offset 8"
        );
    }

    #[test]
    fn packed_layout_field_offsets() {
        // repr(C, packed): a@0, b@1, c@5 — no gaps
        let v = PackedLayout { a: 0, b: 0, c: 0 };
        let base = &v as *const PackedLayout as usize;
        // Use addr_of! — never take a reference to a packed field
        let a_off = std::ptr::addr_of!(v.a) as usize - base;
        let b_off = std::ptr::addr_of!(v.b) as usize - base;
        let c_off = std::ptr::addr_of!(v.c) as usize - base;
        assert_eq!(a_off, 0, "a must be at offset 0");
        assert_eq!(b_off, 1, "b must be at offset 1");
        assert_eq!(c_off, 5, "c must be at offset 5");
    }

    // ── Safe accessors for packed fields ──────────────────────────────────────

    #[test]
    fn packet_accessors_return_correct_values() {
        let h = PacketHeader {
            magic: 0xAB,
            length: 0x0102_0304,
            checksum: 0xBEEF,
        };
        assert_eq!(h.magic, 0xAB);
        assert_eq!(packet_length(&h), 0x0102_0304);
        assert_eq!(packet_checksum(&h), 0xBEEF);
    }

    // ── Serialise / deserialise round-trip ────────────────────────────────────

    #[test]
    fn packet_round_trips_through_bytes() {
        let original = PacketHeader {
            magic: 0xFF,
            length: 1024,
            checksum: 0xCAFE,
        };
        let bytes = packet_to_bytes(&original);
        assert_eq!(bytes.len(), 7);

        let recovered = packet_from_bytes(&bytes);
        // Compare field by field (no PartialEq on packed struct to avoid UB)
        assert_eq!(recovered.magic, original.magic);
        assert_eq!(packet_length(&recovered), packet_length(&original));
        assert_eq!(packet_checksum(&recovered), packet_checksum(&original));
    }

    #[test]
    fn packet_bytes_are_little_endian_layout() {
        // length = 0x0000_0005 → bytes [05, 00, 00, 00] on little-endian
        let h = PacketHeader {
            magic: 1,
            length: 5,
            checksum: 0,
        };
        let bytes = packet_to_bytes(&h);
        assert_eq!(bytes[0], 1); // magic
                                 // bytes[1..5] are length in native byte order
        let reconstructed_length = u32::from_ne_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        assert_eq!(reconstructed_length, 5);
    }

    // ── SIMD vec4 dot product ─────────────────────────────────────────────────

    #[test]
    fn dot_product_orthogonal_vectors() {
        let x_axis = SimdVec4 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        let y_axis = SimdVec4 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        };
        assert_eq!(dot(x_axis, y_axis), 0.0);
    }

    #[test]
    fn dot_product_parallel_vectors() {
        let v = SimdVec4 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        // v·v = 1 + 4 + 9 + 16 = 30
        assert!((dot(v, v) - 30.0).abs() < f32::EPSILON);
    }

    // ── repr(align) instance alignment ───────────────────────────────────────

    #[test]
    fn simd_vec4_instance_is_correctly_aligned() {
        let v = SimdVec4 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let addr = &v as *const SimdVec4 as usize;
        assert_eq!(addr % 16, 0, "SimdVec4 instance must be 16-byte aligned");
    }
}
