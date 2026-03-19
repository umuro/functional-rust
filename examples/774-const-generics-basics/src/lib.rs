#![allow(clippy::all)]
//! # Const Generics Basics
//!
//! Using compile-time constant parameters.

/// Array wrapper with const generic size
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Array<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> Array<T, N> {
    /// Create array filled with default values
    pub fn new() -> Self {
        Array {
            data: [T::default(); N],
        }
    }

    /// Get the length (known at compile time)
    pub const fn len(&self) -> usize {
        N
    }

    /// Check if empty
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Get element by index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Set element by index
    pub fn set(&mut self, index: usize, value: T) -> bool {
        if index < N {
            self.data[index] = value;
            true
        } else {
            false
        }
    }
}

impl<T: Default + Copy, const N: usize> Default for Array<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Fixed-size buffer
#[derive(Debug)]
pub struct Buffer<const SIZE: usize> {
    data: [u8; SIZE],
    len: usize,
}

impl<const SIZE: usize> Buffer<SIZE> {
    pub const fn new() -> Self {
        Buffer {
            data: [0; SIZE],
            len: 0,
        }
    }

    pub const fn capacity(&self) -> usize {
        SIZE
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, byte: u8) -> bool {
        if self.len < SIZE {
            self.data[self.len] = byte;
            self.len += 1;
            true
        } else {
            false
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<const SIZE: usize> Default for Buffer<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

/// Matrix with compile-time dimensions
#[derive(Debug, Clone)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn new() -> Self {
        Matrix {
            data: [[T::default(); COLS]; ROWS],
        }
    }

    pub const fn rows(&self) -> usize {
        ROWS
    }

    pub const fn cols(&self) -> usize {
        COLS
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row).and_then(|r| r.get(col))
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if row < ROWS && col < COLS {
            self.data[row][col] = value;
        }
    }
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS> {
    fn default() -> Self {
        Self::new()
    }
}

/// Function with const generic parameter
pub fn repeat<const N: usize>(value: char) -> [char; N] {
    [value; N]
}

/// Sum array elements
pub fn sum_array<const N: usize>(arr: &[i32; N]) -> i32 {
    arr.iter().sum()
}

/// Compare array lengths at compile time
pub fn arrays_same_size<T, U, const N: usize, const M: usize>(_a: &[T; N], _b: &[U; M]) -> bool {
    N == M
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_basic() {
        let mut arr: Array<i32, 5> = Array::new();
        assert_eq!(arr.len(), 5);
        arr.set(0, 42);
        assert_eq!(arr.get(0), Some(&42));
    }

    #[test]
    fn test_buffer() {
        let mut buf: Buffer<16> = Buffer::new();
        assert_eq!(buf.capacity(), 16);

        buf.push(b'H');
        buf.push(b'i');
        assert_eq!(buf.as_slice(), b"Hi");
    }

    #[test]
    fn test_buffer_overflow() {
        let mut buf: Buffer<2> = Buffer::new();
        assert!(buf.push(b'a'));
        assert!(buf.push(b'b'));
        assert!(!buf.push(b'c')); // Full
    }

    #[test]
    fn test_matrix() {
        let mut m: Matrix<i32, 3, 4> = Matrix::new();
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 4);

        m.set(1, 2, 42);
        assert_eq!(m.get(1, 2), Some(&42));
    }

    #[test]
    fn test_repeat() {
        let arr: [char; 5] = repeat('x');
        assert_eq!(arr, ['x', 'x', 'x', 'x', 'x']);
    }

    #[test]
    fn test_sum_array() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_array(&arr), 15);
    }

    #[test]
    fn test_same_size() {
        let a = [1, 2, 3];
        let b = ['a', 'b', 'c'];
        let c = [1.0, 2.0];

        assert!(arrays_same_size(&a, &b));
        assert!(!arrays_same_size(&a, &c));
    }
}
