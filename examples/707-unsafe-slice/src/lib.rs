//! # Unsafe Slice Operations

pub unsafe fn get_unchecked(arr: &[i32], idx: usize) -> i32 { *arr.get_unchecked(idx) }

pub fn split_at_mut_manual(arr: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = arr.len();
    let ptr = arr.as_mut_ptr();
    assert!(mid <= len);
    unsafe { (std::slice::from_raw_parts_mut(ptr, mid), std::slice::from_raw_parts_mut(ptr.add(mid), len - mid)) }
}

pub fn swap_elements(arr: &mut [i32], i: usize, j: usize) {
    let ptr = arr.as_mut_ptr();
    unsafe { std::ptr::swap(ptr.add(i), ptr.add(j)); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split() {
        let mut arr = [1, 2, 3, 4, 5];
        let (a, b) = split_at_mut_manual(&mut arr, 2);
        assert_eq!(a, &[1, 2]);
        assert_eq!(b, &[3, 4, 5]);
    }
}
