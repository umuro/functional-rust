//! 699 — Raw Pointer Basics
//! *const T and *mut T: creation, casting, and safe wrapping.


/// Read `idx` from a slice via raw pointer, returning None for out-of-bounds.
fn safe_read(slice: &[u32], idx: usize) -> Option<u32> {
    if idx >= slice.len() {
        return None;
    }
    let ptr: *const u32 = slice.as_ptr();
    Some(unsafe {
        // SAFETY: idx < slice.len() checked above; ptr is valid for
        // slice.len() elements; alignment guaranteed by slice invariant.
        *ptr.add(idx)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_read_in_bounds() {
        let arr = [10u32, 20, 30];
        assert_eq!(safe_read(&arr, 0), Some(10));
        assert_eq!(safe_read(&arr, 2), Some(30));
    }

    #[test]
    fn test_safe_read_out_of_bounds() {
        let arr = [10u32, 20, 30];
        assert_eq!(safe_read(&arr, 3), None);
        assert_eq!(safe_read(&arr, usize::MAX), None);
    }

    #[test]
    fn test_raw_ptr_mut_write() {
        let mut val: i64 = 7;
        let p: *mut i64 = &mut val;
        unsafe {
            // SAFETY: p derived from `val` still live; no other references.
            *p = 42;
        }
        assert_eq!(val, 42);
    }

    #[test]
    fn test_const_ptr_read() {
        let data = [100u8, 200, 150];
        let ptr: *const u8 = data.as_ptr();
        let second = unsafe {
            // SAFETY: offset 1 < data.len() = 3; valid, aligned.
            *ptr.add(1)
        };
        assert_eq!(second, 200);
    }
}
