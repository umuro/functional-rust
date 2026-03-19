//! 701 — Unsafe Functions
//! unsafe fn declarations and the safe-wrapper idiom.

/// Copy `n` bytes from `src` to `dst` without bounds checking.
///
/// # Safety
/// - `src` must be valid for `n` bytes of reads.
/// - `dst` must be valid for `n` bytes of writes.
/// - The two regions must not overlap.
/// - Both pointers must be aligned for `u8` (alignment 1 — always satisfied).
unsafe fn raw_copy(src: *const u8, dst: *mut u8, n: usize) {
    for i in 0..n {
        // SAFETY: Caller guarantees src and dst are valid for n bytes,
        // non-overlapping, and aligned.
        *dst.add(i) = *src.add(i);
    }
}

/// Safe wrapper: validates slice lengths, then calls `raw_copy`.
pub fn safe_copy(src: &[u8], dst: &mut [u8]) -> Result<(), String> {
    if src.len() != dst.len() {
        return Err(format!(
            "length mismatch: src={} dst={}",
            src.len(),
            dst.len()
        ));
    }
    unsafe {
        // SAFETY: Both slices are valid for their full length.
        // Rust's borrow rules guarantee &[u8] and &mut [u8] cannot alias.
        raw_copy(src.as_ptr(), dst.as_mut_ptr(), src.len());
    }
    Ok(())
}

/// Get the element at index without bounds check.
///
/// # Safety
/// `idx` must be less than `slice.len()`.
unsafe fn get_unchecked<T: Copy>(slice: &[T], idx: usize) -> T {
    // SAFETY: Caller guarantees idx < slice.len().
    *slice.as_ptr().add(idx)
}

/// Safe wrapper: bounds-checks before calling get_unchecked.
pub fn safe_get<T: Copy>(slice: &[T], idx: usize) -> Option<T> {
    if idx < slice.len() {
        Some(unsafe {
            // SAFETY: idx < slice.len() confirmed above.
            get_unchecked(slice, idx)
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_copy() {
        let src = b"abcde";
        let mut dst = vec![0u8; 5];
        assert!(safe_copy(src, &mut dst).is_ok());
        assert_eq!(&dst, b"abcde");
    }

    #[test]
    fn test_safe_copy_mismatch() {
        assert!(safe_copy(b"abc", &mut vec![0u8; 5]).is_err());
    }

    #[test]
    fn test_safe_get() {
        let v = [1i32, 2, 3];
        assert_eq!(safe_get(&v, 0), Some(1));
        assert_eq!(safe_get(&v, 2), Some(3));
        assert_eq!(safe_get(&v, 3), None);
    }
}
