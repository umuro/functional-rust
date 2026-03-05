//! # Const Where Bounds
//!
//! Constraining const generic parameters.

/// Only allow non-zero sizes
pub struct NonEmptyArray<T, const N: usize>
where
    [(); N - 1]: Sized, // N >= 1
{
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> NonEmptyArray<T, N>
where
    [(); N - 1]: Sized,
{
    pub fn new() -> Self {
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

impl<T: Default + Copy, const N: usize> Default for NonEmptyArray<T, N>
where
    [(); N - 1]: Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Power of two only
pub struct PowerOfTwoBuffer<const SIZE: usize>
where
    [(); (SIZE & (SIZE - 1))]: Sized, // SIZE is power of 2 (fails if not)
{
    data: [u8; SIZE],
}

impl<const SIZE: usize> PowerOfTwoBuffer<SIZE>
where
    [(); (SIZE & (SIZE - 1))]: Sized,
{
    pub const fn new() -> Self {
        PowerOfTwoBuffer { data: [0; SIZE] }
    }

    pub const fn size(&self) -> usize {
        SIZE
    }

    /// Fast modulo using bit mask
    pub const fn wrap_index(&self, idx: usize) -> usize {
        idx & (SIZE - 1)
    }
}

/// Ensure N divides M evenly
pub struct AlignedChunks<T, const N: usize, const M: usize>
where
    [(); M % N]: Sized, // M must be divisible by N (fails otherwise)
{
    data: [[T; N]; M / N],
}

impl<T: Default + Copy, const N: usize, const M: usize> AlignedChunks<T, N, M>
where
    [(); M % N]: Sized,
{
    pub fn new() -> Self {
        AlignedChunks {
            data: [[T::default(); N]; M / N],
        }
    }

    pub const fn chunk_size(&self) -> usize {
        N
    }

    pub const fn num_chunks(&self) -> usize {
        M / N
    }

    pub fn get_chunk(&self, idx: usize) -> Option<&[T; N]> {
        self.data.get(idx)
    }
}

impl<T: Default + Copy, const N: usize, const M: usize> Default for AlignedChunks<T, N, M>
where
    [(); M % N]: Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Minimum size constraint
pub struct MinSizeBuffer<const N: usize>
where
    [(); N.saturating_sub(64)]: Sized, // N >= 64
{
    data: [u8; N],
}

impl<const N: usize> MinSizeBuffer<N>
where
    [(); N.saturating_sub(64)]: Sized,
{
    pub const fn new() -> Self {
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

    // This won't compile:
    // let bad: NonEmptyArray<i32, 0> = NonEmptyArray::new();

    #[test]
    fn test_power_of_two_buffer() {
        let buf: PowerOfTwoBuffer<16> = PowerOfTwoBuffer::new();
        assert_eq!(buf.size(), 16);
        assert_eq!(buf.wrap_index(17), 1); // 17 % 16 = 1
    }

    // This won't compile (not power of 2):
    // let bad: PowerOfTwoBuffer<15> = PowerOfTwoBuffer::new();

    #[test]
    fn test_aligned_chunks() {
        let chunks: AlignedChunks<i32, 4, 12> = AlignedChunks::new();
        assert_eq!(chunks.chunk_size(), 4);
        assert_eq!(chunks.num_chunks(), 3);
    }

    // This won't compile (12 % 5 != 0):
    // let bad: AlignedChunks<i32, 5, 12> = AlignedChunks::new();

    #[test]
    fn test_min_size() {
        let buf: MinSizeBuffer<128> = MinSizeBuffer::new();
        assert_eq!(buf.data.len(), 128);
    }
}
