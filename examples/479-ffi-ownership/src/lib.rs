//! # FFI Ownership — Managing Memory Across FFI
//!
//! Patterns for transferring ownership across the FFI boundary.

use std::ffi::c_void;
use std::os::raw::c_int;

/// Opaque type for FFI
pub struct OpaqueData {
    values: Vec<i32>,
    name: String,
}

/// Create and transfer ownership to C
#[no_mangle]
pub extern "C" fn opaque_create() -> *mut OpaqueData {
    let data = Box::new(OpaqueData {
        values: Vec::new(),
        name: String::from("default"),
    });
    Box::into_raw(data)
}

/// Borrow without taking ownership
#[no_mangle]
pub extern "C" fn opaque_get_count(ptr: *const OpaqueData) -> c_int {
    if ptr.is_null() {
        return -1;
    }
    unsafe { (*ptr).values.len() as c_int }
}

/// Mutably borrow
#[no_mangle]
pub extern "C" fn opaque_push(ptr: *mut OpaqueData, value: c_int) {
    if !ptr.is_null() {
        unsafe {
            (*ptr).values.push(value);
        }
    }
}

/// Take ownership back and free
#[no_mangle]
pub extern "C" fn opaque_free(ptr: *mut OpaqueData) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

/// Pattern: Allocate, fill, return
#[no_mangle]
pub extern "C" fn create_array(len: usize) -> *mut c_int {
    let mut vec = vec![0c_int; len];
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // Prevent deallocation
    ptr
}

#[no_mangle]
pub extern "C" fn free_array(ptr: *mut c_int, len: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, len, len);
        }
    }
}

/// Pattern: Caller allocates, Rust fills
#[no_mangle]
pub extern "C" fn fill_buffer(buf: *mut u8, len: usize) -> usize {
    if buf.is_null() {
        return 0;
    }

    let data = b"Hello from Rust!";
    let copy_len = data.len().min(len);

    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), buf, copy_len);
    }

    copy_len
}

/// Pattern: Return struct by value (copied)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SmallResult {
    pub success: bool,
    pub value: c_int,
}

#[no_mangle]
pub extern "C" fn compute_result(input: c_int) -> SmallResult {
    SmallResult {
        success: input >= 0,
        value: input * 2,
    }
}

/// Pattern: Out parameter
#[no_mangle]
pub extern "C" fn get_values(a: *mut c_int, b: *mut c_int) -> bool {
    if a.is_null() || b.is_null() {
        return false;
    }

    unsafe {
        *a = 42;
        *b = 100;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opaque_lifecycle() {
        let ptr = opaque_create();
        assert!(!ptr.is_null());

        opaque_push(ptr, 10);
        opaque_push(ptr, 20);
        assert_eq!(opaque_get_count(ptr), 2);

        opaque_free(ptr);
    }

    #[test]
    fn test_opaque_null_safety() {
        assert_eq!(opaque_get_count(std::ptr::null()), -1);
        opaque_push(std::ptr::null_mut(), 10); // Should not crash
        opaque_free(std::ptr::null_mut()); // Should not crash
    }

    #[test]
    fn test_array_allocation() {
        let ptr = create_array(5);
        assert!(!ptr.is_null());

        unsafe {
            *ptr = 1;
            *ptr.add(1) = 2;
            assert_eq!(*ptr, 1);
        }

        free_array(ptr, 5);
    }

    #[test]
    fn test_fill_buffer() {
        let mut buf = [0u8; 32];
        let len = fill_buffer(buf.as_mut_ptr(), buf.len());

        assert_eq!(len, 16);
        assert_eq!(&buf[..len], b"Hello from Rust!");
    }

    #[test]
    fn test_return_struct() {
        let result = compute_result(10);
        assert!(result.success);
        assert_eq!(result.value, 20);

        let result = compute_result(-5);
        assert!(!result.success);
    }

    #[test]
    fn test_out_parameters() {
        let mut a = 0;
        let mut b = 0;

        let success = get_values(&mut a, &mut b);
        assert!(success);
        assert_eq!(a, 42);
        assert_eq!(b, 100);

        let fail = get_values(std::ptr::null_mut(), &mut b);
        assert!(!fail);
    }
}
