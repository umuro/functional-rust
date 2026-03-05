//! # Unsafe Traits — Manual Safety Guarantees
//!
//! Traits that require manual verification of invariants.

/// Unsafe trait: implementer guarantees thread-safety
pub unsafe trait ThreadSafe {}

// Safe types implement ThreadSafe
unsafe impl ThreadSafe for i32 {}
unsafe impl ThreadSafe for String {}
unsafe impl<T: ThreadSafe> ThreadSafe for Vec<T> {}

/// Unsafe trait for zero-copy parsing
pub unsafe trait ZeroCopy: Sized {
    fn from_bytes(bytes: &[u8]) -> Option<&Self>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub magic: u32,
    pub version: u16,
    pub flags: u16,
}

unsafe impl ZeroCopy for Header {
    fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        if bytes.len() < std::mem::size_of::<Self>() {
            return None;
        }
        let ptr = bytes.as_ptr() as *const Self;
        // Safety: caller ensures bytes are valid Header
        Some(unsafe { &*ptr })
    }
}

/// GlobalAlloc is an unsafe trait from std
use std::alloc::{GlobalAlloc, Layout};

pub struct TrackingAllocator;

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        std::alloc::System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        std::alloc::System.dealloc(ptr, layout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn require_thread_safe<T: ThreadSafe>() {}

    #[test]
    fn test_thread_safe_bounds() {
        require_thread_safe::<i32>();
        require_thread_safe::<String>();
        require_thread_safe::<Vec<i32>>();
    }

    #[test]
    fn test_zero_copy() {
        let bytes = [0x12, 0x34, 0x56, 0x78, 0x01, 0x00, 0x02, 0x00];
        let header = Header::from_bytes(&bytes).unwrap();
        assert_eq!(header.magic, 0x78563412); // Little endian
    }

    #[test]
    fn test_zero_copy_too_small() {
        let bytes = [0x12, 0x34];
        assert!(Header::from_bytes(&bytes).is_none());
    }
}
