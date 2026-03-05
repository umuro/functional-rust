//! # FFI Callbacks — Passing Functions Across FFI Boundary
//!
//! How to use callbacks with FFI safely.

use std::ffi::c_void;
use std::os::raw::c_int;

/// Simple C-style callback type
pub type SimpleCallback = extern "C" fn(c_int) -> c_int;

/// Callback with user data
pub type CallbackWithData = extern "C" fn(c_int, *mut c_void) -> c_int;

/// Apply a simple callback
pub fn apply_simple_callback(value: c_int, callback: SimpleCallback) -> c_int {
    callback(value)
}

/// Apply callback with user data
pub fn apply_callback_with_data(
    value: c_int,
    callback: CallbackWithData,
    user_data: *mut c_void,
) -> c_int {
    callback(value, user_data)
}

/// Rust closure wrapper for FFI callback
pub struct CallbackWrapper<F> {
    callback: F,
}

impl<F: Fn(c_int) -> c_int> CallbackWrapper<F> {
    pub fn new(callback: F) -> Self {
        Self { callback }
    }

    pub fn call(&self, value: c_int) -> c_int {
        (self.callback)(value)
    }

    /// Get a C-compatible function pointer and data pointer
    pub fn to_c_callback(&mut self) -> (CallbackWithData, *mut c_void) {
        extern "C" fn trampoline<F: Fn(c_int) -> c_int>(
            value: c_int,
            user_data: *mut c_void,
        ) -> c_int {
            let wrapper = unsafe { &*(user_data as *const CallbackWrapper<F>) };
            wrapper.call(value)
        }

        (trampoline::<F>, self as *mut Self as *mut c_void)
    }
}

/// C-style iteration callback
pub type IterCallback = extern "C" fn(c_int, *mut c_void) -> bool;

/// Iterate over data, calling callback for each element
pub fn iterate_with_callback(
    data: &[c_int],
    callback: IterCallback,
    user_data: *mut c_void,
) -> bool {
    for &item in data {
        if !callback(item, user_data) {
            return false; // Callback requested stop
        }
    }
    true
}

/// Event handler pattern
#[repr(C)]
pub struct EventHandler {
    pub on_start: Option<extern "C" fn(*mut c_void)>,
    pub on_data: Option<extern "C" fn(c_int, *mut c_void)>,
    pub on_end: Option<extern "C" fn(*mut c_void)>,
    pub user_data: *mut c_void,
}

impl EventHandler {
    pub fn trigger_start(&self) {
        if let Some(f) = self.on_start {
            f(self.user_data);
        }
    }

    pub fn trigger_data(&self, data: c_int) {
        if let Some(f) = self.on_data {
            f(data, self.user_data);
        }
    }

    pub fn trigger_end(&self) {
        if let Some(f) = self.on_end {
            f(self.user_data);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern "C" fn double_it(x: c_int) -> c_int {
        x * 2
    }

    extern "C" fn add_context(x: c_int, ctx: *mut c_void) -> c_int {
        let offset = unsafe { *(ctx as *const c_int) };
        x + offset
    }

    #[test]
    fn test_simple_callback() {
        assert_eq!(apply_simple_callback(21, double_it), 42);
    }

    #[test]
    fn test_callback_with_data() {
        let offset: c_int = 10;
        let result = apply_callback_with_data(
            5,
            add_context,
            &offset as *const c_int as *mut c_void,
        );
        assert_eq!(result, 15);
    }

    #[test]
    fn test_callback_wrapper() {
        let mut wrapper = CallbackWrapper::new(|x| x * 3);
        assert_eq!(wrapper.call(10), 30);
    }

    extern "C" fn sum_callback(value: c_int, ctx: *mut c_void) -> bool {
        unsafe {
            let sum = ctx as *mut c_int;
            *sum += value;
        }
        true
    }

    #[test]
    fn test_iteration() {
        let data = [1, 2, 3, 4, 5];
        let mut sum: c_int = 0;

        iterate_with_callback(&data, sum_callback, &mut sum as *mut c_int as *mut c_void);

        assert_eq!(sum, 15);
    }

    extern "C" fn stop_at_three(value: c_int, ctx: *mut c_void) -> bool {
        unsafe {
            let count = ctx as *mut c_int;
            *count += 1;
        }
        value != 3
    }

    #[test]
    fn test_early_stop() {
        let data = [1, 2, 3, 4, 5];
        let mut count: c_int = 0;

        let completed = iterate_with_callback(
            &data,
            stop_at_three,
            &mut count as *mut c_int as *mut c_void,
        );

        assert!(!completed);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_event_handler() {
        static mut EVENTS: Vec<&str> = Vec::new();

        extern "C" fn on_start(_: *mut c_void) {
            unsafe { EVENTS.push("start"); }
        }

        extern "C" fn on_data(v: c_int, _: *mut c_void) {
            unsafe { EVENTS.push(if v > 0 { "positive" } else { "non-positive" }); }
        }

        extern "C" fn on_end(_: *mut c_void) {
            unsafe { EVENTS.push("end"); }
        }

        unsafe { EVENTS.clear(); }

        let handler = EventHandler {
            on_start: Some(on_start),
            on_data: Some(on_data),
            on_end: Some(on_end),
            user_data: std::ptr::null_mut(),
        };

        handler.trigger_start();
        handler.trigger_data(42);
        handler.trigger_end();

        unsafe {
            assert_eq!(EVENTS, vec!["start", "positive", "end"]);
        }
    }
}
