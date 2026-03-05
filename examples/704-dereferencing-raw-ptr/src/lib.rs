//! # Dereferencing Raw Pointers

pub fn raw_ptr_example() -> i32 {
    let x = 42;
    let ptr = &x as *const i32;
    unsafe { *ptr }
}

pub fn mutable_raw_ptr() -> i32 {
    let mut x = 10;
    let ptr = &mut x as *mut i32;
    unsafe { *ptr += 5; *ptr }
}

pub fn ptr_from_address(addr: usize) -> *const i32 {
    addr as *const i32
}

pub fn null_check(ptr: *const i32) -> Option<i32> {
    if ptr.is_null() { None } else { unsafe { Some(*ptr) } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_raw() { assert_eq!(raw_ptr_example(), 42); }
    #[test]
    fn test_mut() { assert_eq!(mutable_raw_ptr(), 15); }
    #[test]
    fn test_null() {
        let x = 5;
        assert_eq!(null_check(&x), Some(5));
        assert_eq!(null_check(std::ptr::null()), None);
    }
}
