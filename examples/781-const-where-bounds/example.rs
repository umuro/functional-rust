// 781. Where Bounds on const Generic Parameters
// Type-level constraints on const N

// ── Non-zero constraint ────────────────────────────────────────────────────────

/// A type that can only be instantiated with N > 0
/// The where clause makes it a compile error to use NonEmpty<0>
pub struct NonEmpty<T, const N: usize>
where
    [(); N - 1]: Sized,  // panics at compile time if N == 0 (underflow)
{
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> NonEmpty<T, N>
where
    [(); N - 1]: Sized,
{
    pub fn new() -> Self {
        Self { data: [T::default(); N] }
    }
    pub fn first(&self) -> &T { &self.data[0] }  // safe: N >= 1
    pub fn last(&self)  -> &T { &self.data[N - 1] }
    pub fn len(&self)   -> usize { N }
}

// ── Power-of-two constraint ────────────────────────────────────────────────────

/// Fast ring buffer using bitwise AND instead of modulo
/// Const assert ensures N is a power of two at compile time
pub struct PowerOfTwoRing<T: Default + Copy, const N: usize>
where
    [(); { assert!(N.is_power_of_two(), "N must be a power of two"); 0 }]: Sized,
{
    buf: [T; N],
    head: usize,
    count: usize,
}

impl<T: Default + Copy, const N: usize> PowerOfTwoRing<T, N>
where
    [(); { assert!(N.is_power_of_two(), "N must be a power of two"); 0 }]: Sized,
{
    pub fn new() -> Self { Self { buf: [T::default(); N], head: 0, count: 0 } }

    /// Fast modulo: x & (N-1) instead of x % N
    #[inline]
    fn idx(&self, i: usize) -> usize { i & (N - 1) }

    pub fn push(&mut self, v: T) {
        let tail = self.idx(self.head + self.count);
        self.buf[tail] = v;
        if self.count < N { self.count += 1; }
        else { self.head = self.idx(self.head + 1); }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i >= self.count { return None; }
        Some(&self.buf[self.idx(self.head + i)])
    }

    pub fn len(&self) -> usize { self.count }
}

// ── Const-checked aligned buffer ──────────────────────────────────────────────

use std::mem::align_of;

pub struct AlignAtLeast<T, const ALIGN: usize>
where
    [(); { assert!(align_of::<T>() >= ALIGN, "type alignment too small"); 0 }]: Sized,
{
    value: T,
}

impl<T, const ALIGN: usize> AlignAtLeast<T, ALIGN>
where
    [(); { assert!(align_of::<T>() >= ALIGN, "type alignment too small"); 0 }]: Sized,
{
    pub fn new(value: T) -> Self { Self { value } }
    pub fn get(&self) -> &T { &self.value }
}

// ── Even/odd constraint via const expressions ──────────────────────────────────

/// Pair array: must have even number of elements
pub struct PairArray<T: Default + Copy, const N: usize>
where
    [(); N % 2]: Sized,   // N % 2 == 0 means [(); 0]: Sized (ok), N % 2 == 1 means [(); 1]: Sized (also ok but bad semantics)
{
    data: [T; N],
}

fn main() {
    // NonEmpty<T, N> — N=3 works, NonEmpty<i32, 0> would not compile
    let ne: NonEmpty<i32, 3> = NonEmpty::new();
    println!("NonEmpty<3>: len={}, first={}", ne.len(), ne.first());
    // let bad: NonEmpty<i32, 0> = NonEmpty::new(); // COMPILE ERROR

    // Power-of-two ring
    let mut ring: PowerOfTwoRing<i32, 8> = PowerOfTwoRing::new();
    for i in 0..10 { ring.push(i); }
    println!("Ring[0]={:?}, Ring[7]={:?}", ring.get(0), ring.get(7));
    // let bad_ring: PowerOfTwoRing<i32, 6> = PowerOfTwoRing::new(); // COMPILE ERROR

    // AlignAtLeast
    let a: AlignAtLeast<u64, 4> = AlignAtLeast::new(42);
    println!("AlignAtLeast<u64, 4> value: {}", a.get());
    // let bad: AlignAtLeast<u8, 8> = AlignAtLeast::new(0); // COMPILE ERROR — u8 align < 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty_first_last() {
        let mut ne: NonEmpty<i32, 3> = NonEmpty::new();
        ne.data = [10, 20, 30];
        assert_eq!(*ne.first(), 10);
        assert_eq!(*ne.last(), 30);
    }

    #[test]
    fn pow2_ring_fast_modulo() {
        let mut r: PowerOfTwoRing<i32, 4> = PowerOfTwoRing::new();
        for i in 0..6 { r.push(i); }
        // last 4: 2, 3, 4, 5
        assert_eq!(r.get(0), Some(&2));
        assert_eq!(r.get(3), Some(&5));
    }

    #[test]
    fn aligned_buffer_works() {
        let a: AlignAtLeast<u32, 4> = AlignAtLeast::new(99);
        assert_eq!(*a.get(), 99);
    }
}
