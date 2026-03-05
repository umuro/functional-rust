//! # Const Array Size
//!
//! Using const generics for array sizes.

/// Stack-allocated vector with compile-time capacity
#[derive(Debug, Clone)]
pub struct StackVec<T: Copy + Default, const CAP: usize> {
    data: [T; CAP],
    len: usize,
}

impl<T: Copy + Default, const CAP: usize> StackVec<T, CAP> {
    pub const fn new() -> Self {
        StackVec {
            data: [T::default(); CAP],
            len: 0,
        }
    }

    pub const fn capacity(&self) -> usize {
        CAP
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn is_full(&self) -> bool {
        self.len >= CAP
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len < CAP {
            self.data[self.len] = value;
            self.len += 1;
            Ok(())
        } else {
            Err(value)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            Some(self.data[self.len])
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data[..self.len]
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<T: Copy + Default, const CAP: usize> Default for StackVec<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

/// Create an array from a function
pub const fn array_from_fn<T: Copy, const N: usize>(mut f: impl FnMut(usize) -> T) -> [T; N] {
    let mut arr = [f(0); N]; // Initialize with first value
    let mut i = 1;
    while i < N {
        arr[i] = f(i);
        i += 1;
    }
    arr
}

/// Sum of array elements
pub fn sum<const N: usize>(arr: &[i32; N]) -> i32 {
    arr.iter().sum()
}

/// Product of array elements
pub fn product<const N: usize>(arr: &[i32; N]) -> i32 {
    arr.iter().product()
}

/// Zip two arrays
pub fn zip_arrays<T: Copy, U: Copy, const N: usize>(
    a: &[T; N],
    b: &[U; N],
) -> [(T, U); N] {
    let mut result = [(a[0], b[0]); N];
    for i in 0..N {
        result[i] = (a[i], b[i]);
    }
    result
}

/// Map over array
pub fn map_array<T: Copy, U: Copy + Default, const N: usize>(
    arr: &[T; N],
    f: impl Fn(T) -> U,
) -> [U; N] {
    let mut result = [U::default(); N];
    for i in 0..N {
        result[i] = f(arr[i]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_vec() {
        let mut v: StackVec<i32, 4> = StackVec::new();
        assert_eq!(v.capacity(), 4);
        assert!(v.is_empty());

        v.push(1).unwrap();
        v.push(2).unwrap();
        assert_eq!(v.len(), 2);
        assert_eq!(v.as_slice(), &[1, 2]);
    }

    #[test]
    fn test_stack_vec_full() {
        let mut v: StackVec<i32, 2> = StackVec::new();
        assert!(v.push(1).is_ok());
        assert!(v.push(2).is_ok());
        assert!(v.push(3).is_err());
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_product() {
        assert_eq!(product(&[1, 2, 3, 4]), 24);
    }

    #[test]
    fn test_zip_arrays() {
        let a = [1, 2, 3];
        let b = ['a', 'b', 'c'];
        let zipped = zip_arrays(&a, &b);
        assert_eq!(zipped, [(1, 'a'), (2, 'b'), (3, 'c')]);
    }

    #[test]
    fn test_map_array() {
        let arr = [1, 2, 3, 4];
        let doubled = map_array(&arr, |x| x * 2);
        assert_eq!(doubled, [2, 4, 6, 8]);
    }
}
