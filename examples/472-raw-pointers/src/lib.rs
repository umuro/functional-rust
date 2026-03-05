//! # Raw Pointers — *const T and *mut T
//!
//! Working with raw pointers in Rust.

use std::ptr;

/// Create raw pointers from references
pub fn pointers_from_references() {
    let x = 42;
    let ptr: *const i32 = &x;

    let mut y = 100;
    let ptr_mut: *mut i32 = &mut y;

    unsafe {
        println!("x via ptr: {}", *ptr);
        *ptr_mut = 200;
        println!("y after modification: {}", y);
    }
}

/// Pointer arithmetic
pub fn pointer_arithmetic(data: &[i32]) -> i32 {
    let ptr = data.as_ptr();
    let mut sum = 0;

    for i in 0..data.len() {
        unsafe {
            sum += *ptr.add(i);
        }
    }

    sum
}

/// Null pointers
pub fn null_handling() -> bool {
    let null: *const i32 = ptr::null();
    let null_mut: *mut i32 = ptr::null_mut();

    null.is_null() && null_mut.is_null()
}

/// Cast between pointer types
pub fn pointer_casting() {
    let x: u32 = 0x12345678;
    let ptr_u32: *const u32 = &x;

    // Cast to byte pointer
    let ptr_u8: *const u8 = ptr_u32 as *const u8;

    unsafe {
        // Read individual bytes (little endian)
        let b0 = *ptr_u8;
        let b1 = *ptr_u8.add(1);
        let b2 = *ptr_u8.add(2);
        let b3 = *ptr_u8.add(3);

        println!("Bytes: {:02x} {:02x} {:02x} {:02x}", b0, b1, b2, b3);
    }
}

/// Copy memory
pub fn copy_memory(src: &[u8], dst: &mut [u8], count: usize) {
    assert!(count <= src.len());
    assert!(count <= dst.len());

    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), count);
    }
}

/// Write to pointer
pub fn write_via_pointer(value: i32) -> i32 {
    let mut x: i32 = 0;
    let ptr: *mut i32 = &mut x;

    unsafe {
        ptr::write(ptr, value);
        ptr::read(ptr)
    }
}

/// Compare pointers
pub fn pointers_equal<T>(a: *const T, b: *const T) -> bool {
    ptr::eq(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_arithmetic() {
        let data = [1, 2, 3, 4, 5];
        assert_eq!(pointer_arithmetic(&data), 15);
    }

    #[test]
    fn test_null_handling() {
        assert!(null_handling());
    }

    #[test]
    fn test_copy_memory() {
        let src = [1u8, 2, 3, 4, 5];
        let mut dst = [0u8; 5];

        copy_memory(&src, &mut dst, 3);
        assert_eq!(dst, [1, 2, 3, 0, 0]);
    }

    #[test]
    fn test_write_via_pointer() {
        assert_eq!(write_via_pointer(42), 42);
    }

    #[test]
    fn test_pointers_equal() {
        let x = 42;
        let ptr1: *const i32 = &x;
        let ptr2: *const i32 = &x;
        let y = 42;
        let ptr3: *const i32 = &y;

        assert!(pointers_equal(ptr1, ptr2));
        assert!(!pointers_equal(ptr1, ptr3));
    }

    #[test]
    fn test_pointer_offset() {
        let arr = [10, 20, 30, 40];
        let ptr = arr.as_ptr();

        unsafe {
            assert_eq!(*ptr, 10);
            assert_eq!(*ptr.add(1), 20);
            assert_eq!(*ptr.add(2), 30);
            assert_eq!(*ptr.offset(3), 40);
        }
    }

    #[test]
    fn test_pointer_from_int() {
        let x = 42;
        let ptr = &x as *const i32;
        let addr = ptr as usize;
        let restored = addr as *const i32;

        unsafe {
            assert_eq!(*restored, 42);
        }
    }
}
