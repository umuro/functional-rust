#![allow(clippy::all)]
//! 703 — Raw Pointer Arithmetic
//!
//! Demonstrates `ptr.add()`, `ptr.sub()`, and `ptr.offset()` with safe wrappers.
//! Each unsafe block carries a `// SAFETY:` comment explaining why the offset is valid.

/// Collect every `stride`-th element using raw pointer arithmetic.
///
/// # Panics
/// Never panics — returns empty vec for empty slice or stride == 0.
pub fn strided_collect(slice: &[i32], stride: usize) -> Vec<i32> {
    if slice.is_empty() || stride == 0 {
        return vec![];
    }
    let mut result = Vec::new();
    let base: *const i32 = slice.as_ptr();
    let len = slice.len();
    let mut offset = 0usize;
    while offset < len {
        // SAFETY: `offset` is always < `len` == `slice.len()`.
        // `base` is valid for `len` elements (derived from a live slice).
        // Alignment is guaranteed by the slice invariant.
        result.push(unsafe { *base.add(offset) });
        offset = offset.saturating_add(stride);
    }
    result
}

/// Reverse a slice in-place using raw pointer swap via converging lo/hi pointers.
pub fn reverse_in_place(slice: &mut [i32]) {
    let len = slice.len();
    if len < 2 {
        return;
    }
    // SAFETY: `lo` starts at index 0 and `hi` at index len-1, both within bounds.
    // We only dereference while lo < hi, so the two pointers never alias.
    unsafe {
        let base: *mut i32 = slice.as_mut_ptr();
        let mut lo = base;
        let mut hi = base.add(len - 1);
        while lo < hi {
            core::ptr::swap(lo, hi);
            lo = lo.add(1);
            hi = hi.sub(1);
        }
    }
}

/// Copy bytes from `src` to `dst` using `ptr.add()` inside a manual loop.
///
/// Demonstrates walking two raw pointers in lockstep.
pub fn manual_copy(src: &[u8], dst: &mut [u8]) {
    let count = src.len().min(dst.len());
    if count == 0 {
        return;
    }
    // SAFETY: `count` <= both slice lengths, so every offset in `0..count`
    // is valid for both `src_ptr` and `dst_ptr`.  The slices do not overlap
    // because one is shared and the other is exclusively borrowed.
    unsafe {
        let src_ptr: *const u8 = src.as_ptr();
        let dst_ptr: *mut u8 = dst.as_mut_ptr();
        for i in 0..count {
            *dst_ptr.add(i) = *src_ptr.add(i);
        }
    }
}

/// Read a value at a signed `offset` from the start of the slice using `ptr.offset()`.
///
/// Returns `None` when the computed index is out of bounds.
pub fn read_at_offset(slice: &[i32], offset: isize) -> Option<i32> {
    let len = slice.len() as isize;
    if offset < 0 || offset >= len {
        return None;
    }
    // SAFETY: we just checked 0 <= offset < len == slice.len(), so the pointer
    // stays within the allocation.
    Some(unsafe { *slice.as_ptr().offset(offset) })
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- strided_collect ---

    #[test]
    fn strided_collect_every_other() {
        let data: Vec<i32> = (0..10).collect();
        assert_eq!(strided_collect(&data, 2), vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn strided_collect_every_third() {
        let data: Vec<i32> = (0..10).collect();
        assert_eq!(strided_collect(&data, 3), vec![0, 3, 6, 9]);
    }

    #[test]
    fn strided_collect_stride_one_equals_slice() {
        let data = vec![7, 8, 9];
        assert_eq!(strided_collect(&data, 1), data);
    }

    #[test]
    fn strided_collect_empty_or_zero_stride() {
        assert_eq!(strided_collect(&[], 2), vec![]);
        assert_eq!(strided_collect(&[1, 2, 3], 0), vec![]);
    }

    #[test]
    fn strided_collect_stride_larger_than_len() {
        let data = vec![10, 20, 30];
        // stride = 5 > len = 3; only the first element is picked
        assert_eq!(strided_collect(&data, 5), vec![10]);
    }

    // --- reverse_in_place ---

    #[test]
    fn reverse_even_length() {
        let mut v = vec![1, 2, 3, 4];
        reverse_in_place(&mut v);
        assert_eq!(v, vec![4, 3, 2, 1]);
    }

    #[test]
    fn reverse_odd_length() {
        let mut v = vec![1, 2, 3, 4, 5];
        reverse_in_place(&mut v);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn reverse_single_and_empty() {
        let mut single = vec![42];
        reverse_in_place(&mut single);
        assert_eq!(single, vec![42]);

        let mut empty: Vec<i32> = vec![];
        reverse_in_place(&mut empty);
        assert_eq!(empty, vec![]);
    }

    // --- manual_copy ---

    #[test]
    fn manual_copy_basic() {
        let src = vec![1u8, 2, 3, 4, 5];
        let mut dst = vec![0u8; 5];
        manual_copy(&src, &mut dst);
        assert_eq!(dst, src);
    }

    #[test]
    fn manual_copy_dst_shorter() {
        let src = vec![10u8, 20, 30, 40];
        let mut dst = vec![0u8; 2];
        manual_copy(&src, &mut dst);
        assert_eq!(dst, vec![10, 20]);
    }

    // --- read_at_offset ---

    #[test]
    fn read_at_offset_valid() {
        let data = vec![100, 200, 300];
        assert_eq!(read_at_offset(&data, 0), Some(100));
        assert_eq!(read_at_offset(&data, 2), Some(300));
    }

    #[test]
    fn read_at_offset_out_of_bounds() {
        let data = vec![1, 2, 3];
        assert_eq!(read_at_offset(&data, 3), None);
        assert_eq!(read_at_offset(&data, -1), None);
    }

    #[test]
    fn read_at_offset_empty() {
        assert_eq!(read_at_offset(&[], 0), None);
    }
}
