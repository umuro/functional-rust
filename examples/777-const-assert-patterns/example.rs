// 777. Compile-Time Assertions with const
// Build fails if invariants are violated — zero runtime overhead

use std::mem::{size_of, align_of};

// ── Basic const assertions ─────────────────────────────────────────────────────

// These run at compile time — if false, the build fails
const _: () = assert!(size_of::<u64>() == 8,  "u64 must be 8 bytes");
const _: () = assert!(size_of::<u32>() == 4,  "u32 must be 4 bytes");
const _: () = assert!(size_of::<bool>() == 1, "bool must be 1 byte");
const _: () = assert!(usize::BITS >= 32,      "need at least 32-bit usize");

// ── Protocol / domain invariants ──────────────────────────────────────────────

const MAGIC: u32  = 0xCAFEBABE;
const VERSION: u8 = 2;
const MAX_PAYLOAD: usize = 65536;
const HEADER_SIZE: usize = 16;

// Ensure header fits in one cache line (≤ 64 bytes)
const _: () = assert!(HEADER_SIZE <= 64, "header must fit in a cache line");
// Ensure max payload is a power of two
const _: () = assert!(MAX_PAYLOAD.is_power_of_two(), "MAX_PAYLOAD must be power of 2");
// Version must be non-zero
const _: () = assert!(VERSION > 0, "version must be > 0");

// ── Struct size assertions ─────────────────────────────────────────────────────

#[repr(C)]
#[derive(Debug)]
pub struct PacketHeader {
    pub magic:   u32,
    pub version: u8,
    pub flags:   u8,
    pub length:  u16,
    pub seq:     u64,
}

// Wire format guarantee: header is exactly 16 bytes
const _: () = assert!(
    size_of::<PacketHeader>() == 16,
    "PacketHeader must be exactly 16 bytes for wire compatibility"
);
const _: () = assert!(
    align_of::<PacketHeader>() >= 4,
    "PacketHeader alignment must be at least 4"
);

// ── Generic const assertions ───────────────────────────────────────────────────

/// A type that is only usable if its alignment is at least A
pub struct AlignedBuffer<T, const A: usize>
where
    [(); { assert!(align_of::<T>() >= A, "alignment requirement not met"); 0 }]: Sized,
{
    data: T,
}

impl<T, const A: usize> AlignedBuffer<T, A>
where
    [(); { assert!(align_of::<T>() >= A, "alignment requirement not met"); 0 }]: Sized,
{
    pub fn new(data: T) -> Self { Self { data } }
    pub fn get(&self) -> &T { &self.data }
}

// ── const fn that panics on bad input ─────────────────────────────────────────

const fn must_be_power_of_two(n: usize) -> usize {
    assert!(n.is_power_of_two(), "N must be a power of two");
    n
}

const CACHE_SIZE: usize = must_be_power_of_two(1024); // fine
// const BAD_CACHE: usize = must_be_power_of_two(1000); // compile error!

// ── Capacity bounds check ──────────────────────────────────────────────────────

pub struct BoundedVec<T, const MAX: usize> {
    data: Vec<T>,
}

impl<T, const MAX: usize> BoundedVec<T, MAX> {
    const _CHECK: () = assert!(MAX > 0 && MAX <= 1_000_000, "MAX out of range");

    pub fn new() -> Self {
        let _ = Self::_CHECK;
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, v: T) -> Result<(), &'static str> {
        if self.data.len() >= MAX { return Err("capacity exceeded"); }
        self.data.push(v);
        Ok(())
    }
    pub fn len(&self) -> usize { self.data.len() }
}

fn main() {
    println!("All compile-time assertions passed!");
    println!("PacketHeader size: {} bytes", size_of::<PacketHeader>());
    println!("CACHE_SIZE: {CACHE_SIZE}");

    let header = PacketHeader {
        magic: MAGIC, version: VERSION, flags: 0, length: 64, seq: 1,
    };
    println!("Header: {header:?}");

    let mut bvec: BoundedVec<i32, 3> = BoundedVec::new();
    for i in 0..3 { bvec.push(i).unwrap(); }
    println!("BoundedVec len: {}", bvec.len());
    println!("BoundedVec push(3): {:?}", bvec.push(99));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_header_size_is_16() {
        assert_eq!(size_of::<PacketHeader>(), 16);
    }

    #[test]
    fn bounded_vec_enforces_capacity() {
        let mut bv: BoundedVec<i32, 2> = BoundedVec::new();
        assert!(bv.push(1).is_ok());
        assert!(bv.push(2).is_ok());
        assert!(bv.push(3).is_err());
    }

    #[test]
    fn cache_size_is_correct() {
        assert_eq!(CACHE_SIZE, 1024);
        assert!(CACHE_SIZE.is_power_of_two());
    }
}
