//! # FFI Safety — Defensive FFI Programming
//!
//! Patterns for making FFI code safer.

use std::ffi::{c_void, CStr};
use std::os::raw::{c_char, c_int};
use std::panic;

/// Error codes for FFI
#[repr(C)]
pub enum ErrorCode {
    Success = 0,
    NullPointer = 1,
    InvalidArgument = 2,
    OutOfMemory = 3,
    Panic = 4,
    Unknown = 99,
}

/// Catch panics at FFI boundary
pub fn catch_panic_ffi<F, T>(f: F, default: T) -> (T, ErrorCode)
where
    F: FnOnce() -> T + panic::UnwindSafe,
{
    match panic::catch_unwind(f) {
        Ok(result) => (result, ErrorCode::Success),
        Err(_) => (default, ErrorCode::Panic),
    }
}

/// Safe wrapper for FFI functions
#[no_mangle]
pub extern "C" fn safe_divide(a: c_int, b: c_int, result: *mut c_int) -> c_int {
    if result.is_null() {
        return ErrorCode::NullPointer as c_int;
    }

    if b == 0 {
        return ErrorCode::InvalidArgument as c_int;
    }

    unsafe {
        *result = a / b;
    }

    ErrorCode::Success as c_int
}

/// Validate string input
#[no_mangle]
pub extern "C" fn safe_strlen(s: *const c_char) -> c_int {
    if s.is_null() {
        return -1;
    }

    let (len, code) = catch_panic_ffi(
        || unsafe { CStr::from_ptr(s).to_bytes().len() as c_int },
        -1,
    );

    match code {
        ErrorCode::Success => len,
        _ => -1,
    }
}

/// Defensive bounds checking
#[no_mangle]
pub extern "C" fn safe_array_get(
    arr: *const c_int,
    len: usize,
    index: usize,
    result: *mut c_int,
) -> c_int {
    if arr.is_null() || result.is_null() {
        return ErrorCode::NullPointer as c_int;
    }

    if index >= len {
        return ErrorCode::InvalidArgument as c_int;
    }

    unsafe {
        *result = *arr.add(index);
    }

    ErrorCode::Success as c_int
}

/// Version and capability checking
#[repr(C)]
pub struct LibraryInfo {
    pub major: c_int,
    pub minor: c_int,
    pub patch: c_int,
}

#[no_mangle]
pub extern "C" fn get_library_info() -> LibraryInfo {
    LibraryInfo {
        major: 1,
        minor: 0,
        patch: 0,
    }
}

/// Thread-safe initialization
use std::sync::Once;

static INIT: Once = Once::new();
static mut INITIALIZED: bool = false;

#[no_mangle]
pub extern "C" fn library_init() -> c_int {
    INIT.call_once(|| {
        // Initialization code here
        unsafe {
            INITIALIZED = true;
        }
    });

    ErrorCode::Success as c_int
}

#[no_mangle]
pub extern "C" fn is_initialized() -> bool {
    unsafe { INITIALIZED }
}

/// Safe handle pattern with validation
pub struct HandleRegistry {
    handles: std::sync::Mutex<Vec<Option<Box<dyn std::any::Any + Send>>>>,
}

impl HandleRegistry {
    pub fn new() -> Self {
        Self {
            handles: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn register<T: std::any::Any + Send + 'static>(&self, value: T) -> usize {
        let mut handles = self.handles.lock().unwrap();
        let id = handles.len();
        handles.push(Some(Box::new(value)));
        id
    }

    pub fn get<T: 'static>(&self, id: usize) -> Option<&T> {
        // This is simplified - real impl would need different approach
        None
    }

    pub fn remove(&self, id: usize) -> bool {
        let mut handles = self.handles.lock().unwrap();
        if id < handles.len() && handles[id].is_some() {
            handles[id] = None;
            true
        } else {
            false
        }
    }
}

impl Default for HandleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide() {
        let mut result = 0;

        let code = safe_divide(10, 2, &mut result);
        assert_eq!(code, ErrorCode::Success as c_int);
        assert_eq!(result, 5);

        let code = safe_divide(10, 0, &mut result);
        assert_eq!(code, ErrorCode::InvalidArgument as c_int);

        let code = safe_divide(10, 2, std::ptr::null_mut());
        assert_eq!(code, ErrorCode::NullPointer as c_int);
    }

    #[test]
    fn test_safe_strlen() {
        use std::ffi::CString;

        let s = CString::new("hello").unwrap();
        assert_eq!(safe_strlen(s.as_ptr()), 5);

        assert_eq!(safe_strlen(std::ptr::null()), -1);
    }

    #[test]
    fn test_safe_array_get() {
        let arr = [10, 20, 30, 40, 50];
        let mut result = 0;

        let code = safe_array_get(arr.as_ptr(), arr.len(), 2, &mut result);
        assert_eq!(code, ErrorCode::Success as c_int);
        assert_eq!(result, 30);

        let code = safe_array_get(arr.as_ptr(), arr.len(), 10, &mut result);
        assert_eq!(code, ErrorCode::InvalidArgument as c_int);
    }

    #[test]
    fn test_library_info() {
        let info = get_library_info();
        assert_eq!(info.major, 1);
        assert_eq!(info.minor, 0);
    }

    #[test]
    fn test_initialization() {
        library_init();
        assert!(is_initialized());
    }

    #[test]
    fn test_catch_panic() {
        let (result, code) = catch_panic_ffi(|| 42, 0);
        assert_eq!(result, 42);
        assert!(matches!(code, ErrorCode::Success));

        let (result, code) = catch_panic_ffi(
            || {
                panic!("test panic");
                #[allow(unreachable_code)]
                0
            },
            -1,
        );
        assert_eq!(result, -1);
        assert!(matches!(code, ErrorCode::Panic));
    }
}
