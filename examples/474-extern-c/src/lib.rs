//! # Extern C — C ABI Compatibility
//!
//! Using C calling convention and #[repr(C)] for FFI.

use std::ffi::{c_char, c_int, c_void};

/// C-compatible struct layout
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// C-compatible function - can be called from C
#[no_mangle]
pub extern "C" fn add_numbers(a: c_int, b: c_int) -> c_int {
    a + b
}

/// C-compatible function with pointers
#[no_mangle]
pub extern "C" fn sum_array(arr: *const c_int, len: usize) -> c_int {
    if arr.is_null() || len == 0 {
        return 0;
    }

    let mut sum = 0;
    for i in 0..len {
        unsafe {
            sum += *arr.add(i);
        }
    }
    sum
}

/// C-compatible struct operations
#[no_mangle]
pub extern "C" fn point_distance(p1: *const Point, p2: *const Point) -> f64 {
    if p1.is_null() || p2.is_null() {
        return -1.0;
    }

    unsafe { (*p1).distance(&*p2) }
}

/// C-compatible callback
pub type Callback = extern "C" fn(c_int) -> c_int;

#[no_mangle]
pub extern "C" fn apply_callback(value: c_int, callback: Callback) -> c_int {
    callback(value)
}

/// C-style enum
#[repr(C)]
pub enum Status {
    Ok = 0,
    Error = 1,
    NotFound = 2,
}

/// C-compatible string operations (simplified)
#[no_mangle]
pub extern "C" fn string_length(s: *const c_char) -> usize {
    if s.is_null() {
        return 0;
    }

    let mut len = 0;
    unsafe {
        while *s.add(len) != 0 {
            len += 1;
        }
    }
    len
}

/// Opaque type pattern for FFI
pub struct OpaqueHandle {
    data: Vec<i32>,
}

#[no_mangle]
pub extern "C" fn handle_create() -> *mut OpaqueHandle {
    Box::into_raw(Box::new(OpaqueHandle { data: Vec::new() }))
}

#[no_mangle]
pub extern "C" fn handle_push(handle: *mut OpaqueHandle, value: c_int) {
    if !handle.is_null() {
        unsafe {
            (*handle).data.push(value);
        }
    }
}

#[no_mangle]
pub extern "C" fn handle_sum(handle: *const OpaqueHandle) -> c_int {
    if handle.is_null() {
        return 0;
    }

    unsafe { (*handle).data.iter().sum() }
}

#[no_mangle]
pub extern "C" fn handle_free(handle: *mut OpaqueHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
    }

    #[test]
    fn test_sum_array() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_array(arr.as_ptr(), arr.len()), 15);
        assert_eq!(sum_array(std::ptr::null(), 0), 0);
    }

    #[test]
    fn test_point() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);

        assert!((point_distance(&p1, &p2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_callback() {
        extern "C" fn double_it(x: c_int) -> c_int {
            x * 2
        }

        assert_eq!(apply_callback(21, double_it), 42);
    }

    #[test]
    fn test_string_length() {
        let s = b"hello\0";
        assert_eq!(string_length(s.as_ptr() as *const c_char), 5);
    }

    #[test]
    fn test_opaque_handle() {
        let handle = handle_create();
        handle_push(handle, 10);
        handle_push(handle, 20);
        handle_push(handle, 30);

        assert_eq!(handle_sum(handle), 60);

        handle_free(handle);
    }

    #[test]
    fn test_repr_c_size() {
        // Point has predictable size with repr(C)
        assert_eq!(std::mem::size_of::<Point>(), 16);
    }
}
