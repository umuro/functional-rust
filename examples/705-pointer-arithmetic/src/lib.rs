//! # Pointer Arithmetic

pub fn ptr_offset_example() {
    let arr = [10, 20, 30, 40, 50];
    let ptr = arr.as_ptr();
    
    unsafe {
        assert_eq!(*ptr, 10);
        assert_eq!(*ptr.add(1), 20);
        assert_eq!(*ptr.add(4), 50);
    }
}

pub fn slice_from_raw_parts() -> Vec<i32> {
    let data = vec![1, 2, 3, 4, 5];
    let ptr = data.as_ptr();
    let len = data.len();
    
    unsafe { std::slice::from_raw_parts(ptr, len).to_vec() }
}

pub fn ptr_diff(a: *const i32, b: *const i32) -> isize {
    unsafe { b.offset_from(a) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_offset() { ptr_offset_example(); }
    #[test]
    fn test_slice() { assert_eq!(slice_from_raw_parts(), vec![1, 2, 3, 4, 5]); }
}
