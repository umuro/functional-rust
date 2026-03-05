//! # Unsafe Basics — When and How to Use Unsafe
//!
//! Understanding when unsafe is necessary and how to use it safely.

/// Unsafe allows:
/// 1. Dereference raw pointers
/// 2. Call unsafe functions
/// 3. Access/modify mutable statics
/// 4. Implement unsafe traits
/// 5. Access union fields

/// Example: Safe wrapper around unsafe operation
pub fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/// Swap two elements using raw pointers
pub fn swap<T>(a: &mut T, b: &mut T) {
    unsafe {
        let pa = a as *mut T;
        let pb = b as *mut T;
        std::ptr::swap(pa, pb);
    }
}

/// Read from raw pointer
pub fn read_raw<T: Copy>(ptr: *const T) -> Option<T> {
    if ptr.is_null() {
        None
    } else {
        unsafe { Some(*ptr) }
    }
}

/// Unsafe block for performance
pub fn sum_unchecked(data: &[i32]) -> i32 {
    let mut sum = 0;
    let len = data.len();
    let ptr = data.as_ptr();

    for i in 0..len {
        unsafe {
            sum += *ptr.add(i);
        }
    }

    sum
}

/// Static mutable (requires unsafe to access)
static mut COUNTER: u64 = 0;

pub fn increment_static_counter() -> u64 {
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}

pub fn get_static_counter() -> u64 {
    unsafe { COUNTER }
}

/// Transmute (very dangerous!)
pub fn reinterpret_bytes(n: u32) -> [u8; 4] {
    unsafe { std::mem::transmute(n) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_at_mut() {
        let mut data = vec![1, 2, 3, 4, 5];
        let (left, right) = split_at_mut(&mut data, 2);

        left[0] = 10;
        right[0] = 30;

        assert_eq!(data, vec![10, 2, 30, 4, 5]);
    }

    #[test]
    fn test_swap() {
        let mut a = 1;
        let mut b = 2;
        swap(&mut a, &mut b);
        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }

    #[test]
    fn test_read_raw() {
        let x = 42;
        let ptr = &x as *const i32;
        assert_eq!(read_raw(ptr), Some(42));

        let null: *const i32 = std::ptr::null();
        assert_eq!(read_raw(null), None);
    }

    #[test]
    fn test_sum_unchecked() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_unchecked(&data), 15);
    }

    #[test]
    fn test_static_counter() {
        // Reset for test isolation
        unsafe {
            COUNTER = 0;
        }

        assert_eq!(increment_static_counter(), 1);
        assert_eq!(increment_static_counter(), 2);
        assert_eq!(get_static_counter(), 2);
    }

    #[test]
    fn test_transmute() {
        let n: u32 = 0x01020304;
        let bytes = reinterpret_bytes(n);
        // Little endian
        assert_eq!(bytes, [0x04, 0x03, 0x02, 0x01]);
    }
}
