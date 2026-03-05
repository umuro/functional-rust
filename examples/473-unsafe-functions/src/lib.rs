//! # Unsafe Functions — Declaring and Calling
//!
//! How to declare and safely wrap unsafe functions.

/// An unsafe function that requires caller to ensure safety
///
/// # Safety
/// - `ptr` must be valid and properly aligned
/// - `ptr` must point to initialized memory
/// - The pointee must not be mutated during the read
pub unsafe fn read_unchecked<T: Copy>(ptr: *const T) -> T {
    *ptr
}

/// Unsafe function with bounds that caller must verify
///
/// # Safety
/// - `index` must be less than `slice.len()`
pub unsafe fn get_unchecked<T>(slice: &[T], index: usize) -> &T {
    slice.get_unchecked(index)
}

/// Unsafe mutable access
///
/// # Safety
/// - Same as `get_unchecked`, plus no other references exist
pub unsafe fn get_unchecked_mut<T>(slice: &mut [T], index: usize) -> &mut T {
    slice.get_unchecked_mut(index)
}

/// Safe wrapper that validates input
pub fn get_safe<T>(slice: &[T], index: usize) -> Option<&T> {
    if index < slice.len() {
        Some(unsafe { get_unchecked(slice, index) })
    } else {
        None
    }
}

/// Safe wrapper that panics on invalid input
pub fn get_or_panic<T>(slice: &[T], index: usize) -> &T {
    assert!(index < slice.len(), "index out of bounds");
    unsafe { get_unchecked(slice, index) }
}

/// Unsafe trait - implementer must ensure invariants
pub unsafe trait UnsafeTrait {
    fn do_something(&self);
}

/// Safe trait wrapping unsafe operations
pub trait SafeOperations {
    fn safe_operation(&self) -> i32;
}

/// Example struct implementing safe wrapper
pub struct SafeWrapper {
    data: Vec<i32>,
}

impl SafeWrapper {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    /// Safe interface using unsafe internally
    pub fn sum(&self) -> i32 {
        let mut total = 0;
        let ptr = self.data.as_ptr();

        for i in 0..self.data.len() {
            unsafe {
                total += *ptr.add(i);
            }
        }

        total
    }

    /// Checked access - safe
    pub fn get(&self, index: usize) -> Option<i32> {
        self.data.get(index).copied()
    }

    /// Unchecked access - unsafe
    ///
    /// # Safety
    /// Index must be in bounds
    pub unsafe fn get_unchecked(&self, index: usize) -> i32 {
        *self.data.get_unchecked(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_unchecked() {
        let x = 42;
        let ptr = &x as *const i32;

        unsafe {
            assert_eq!(read_unchecked(ptr), 42);
        }
    }

    #[test]
    fn test_get_unchecked() {
        let data = [1, 2, 3, 4, 5];

        unsafe {
            assert_eq!(*get_unchecked(&data, 0), 1);
            assert_eq!(*get_unchecked(&data, 4), 5);
        }
    }

    #[test]
    fn test_get_safe() {
        let data = [1, 2, 3];

        assert_eq!(get_safe(&data, 0), Some(&1));
        assert_eq!(get_safe(&data, 2), Some(&3));
        assert_eq!(get_safe(&data, 3), None);
    }

    #[test]
    fn test_safe_wrapper() {
        let wrapper = SafeWrapper::new(vec![1, 2, 3, 4, 5]);

        assert_eq!(wrapper.sum(), 15);
        assert_eq!(wrapper.get(2), Some(3));
        assert_eq!(wrapper.get(10), None);

        unsafe {
            assert_eq!(wrapper.get_unchecked(3), 4);
        }
    }

    #[test]
    #[should_panic]
    fn test_get_or_panic() {
        let data = [1, 2, 3];
        let _ = get_or_panic(&data, 5);
    }

    #[test]
    fn test_get_unchecked_mut() {
        let mut data = [1, 2, 3];

        unsafe {
            *get_unchecked_mut(&mut data, 1) = 20;
        }

        assert_eq!(data, [1, 20, 3]);
    }
}
