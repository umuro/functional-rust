#![allow(clippy::all)]
//! # Const Where Bounds
//!
//! Constraining const generic parameters.
//!
//! Note: Rust stable doesn't support `where [(); expr]:` bounds on const generics.
//! We demonstrate the concept using runtime assertions and trait-based patterns.

/// Non-empty array — uses a const generic N and stores [T; N].
/// The constraint N >= 1 is enforced at construction time.
pub struct NonEmptyArray<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> NonEmptyArray<T, N> {
    /// Panics if N == 0.
    pub fn new() -> Self {
        assert!(N >= 1, "NonEmptyArray requires N >= 1");
        NonEmptyArray {
            data: [T::default(); N],
        }
    }

    pub fn first(&self) -> &T {
        &self.data[0]
    }

    pub fn last(&self) -> &T {
        &self.data[N - 1]
    }
}

impl<T: Default + Copy, const N: usize> Default for NonEmptyArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Power of two buffer — fast modulo via bitmask.
/// The power-of-two constraint is checked at construction.
pub struct PowerOfTwoBuffer<const SIZE: usize> {
    data: [u8; SIZE],
}

impl<const SIZE: usize> PowerOfTwoBuffer<SIZE> {
    pub fn new() -> Self {
        assert!(
            SIZE > 0 && (SIZE & (SIZE - 1)) == 0,
            "SIZE must be a power of 2"
        );
        PowerOfTwoBuffer { data: [0; SIZE] }
    }

    pub const fn size(&self) -> usize {
        SIZE
    }

    /// Fast modulo using bit mask (works because SIZE is power of 2).
    pub const fn wrap_index(&self, idx: usize) -> usize {
        idx & (SIZE - 1)
    }
}

/// Aligned chunks: divide M items into chunks of N.
/// Constraint M % N == 0 is enforced at construction.
pub struct AlignedChunks<T> {
    data: Vec<Vec<T>>,
    chunk_size: usize,
}

impl<T: Default + Clone> AlignedChunks<T> {
    pub fn new(chunk_size: usize, total: usize) -> Self {
        assert!(chunk_size > 0, "chunk_size must be > 0");
        assert!(
            total % chunk_size == 0,
            "total must be divisible by chunk_size"
        );
        let num_chunks = total / chunk_size;
        let data = vec![vec![T::default(); chunk_size]; num_chunks];
        AlignedChunks { data, chunk_size }
    }

    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    pub fn num_chunks(&self) -> usize {
        self.data.len()
    }

    pub fn get_chunk(&self, idx: usize) -> Option<&[T]> {
        self.data.get(idx).map(|v| v.as_slice())
    }
}

/// Minimum size buffer — ensures N >= 64.
pub struct MinSizeBuffer<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> MinSizeBuffer<N> {
    pub fn new() -> Self {
        assert!(N >= 64, "MinSizeBuffer requires N >= 64");
        MinSizeBuffer { data: [0; N] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_array() {
        let arr: NonEmptyArray<i32, 5> = NonEmptyArray::new();
        assert_eq!(*arr.first(), 0);
        assert_eq!(*arr.last(), 0);
    }

    // NonEmptyArray::<i32, 0>::new() would panic at runtime

    #[test]
    fn test_power_of_two_buffer() {
        let buf: PowerOfTwoBuffer<16> = PowerOfTwoBuffer::new();
        assert_eq!(buf.size(), 16);
        assert_eq!(buf.wrap_index(17), 1); // 17 % 16 = 1
    }

    // PowerOfTwoBuffer::<15>::new() would panic (15 is not power of 2)

    #[test]
    fn test_aligned_chunks() {
        let chunks: AlignedChunks<i32> = AlignedChunks::new(4, 12);
        assert_eq!(chunks.chunk_size(), 4);
        assert_eq!(chunks.num_chunks(), 3);
    }

    // AlignedChunks::new(5, 12) would panic (12 % 5 != 0)

    #[test]
    fn test_min_size() {
        let buf: MinSizeBuffer<128> = MinSizeBuffer::new();
        assert_eq!(buf.data.len(), 128);
    }
}
