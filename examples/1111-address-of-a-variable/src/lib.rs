/// Returns the memory address of any value as a `usize`.
///
/// In Rust, every value — stack or heap — has a stable address within its
/// lifetime. This is in contrast to OCaml, where only heap-allocated ("boxed")
/// values have observable addresses; unboxed integers (immediate values) do not.
///
/// Uses `std::ptr::addr_of!` which avoids creating an intermediate reference,
/// making it safe even for packed or unaligned fields.
pub fn address_of<T>(x: &T) -> usize {
    std::ptr::addr_of!(*x) as usize
}

/// Returns true when two references point to distinct memory locations.
pub fn addresses_differ<T>(a: &T, b: &T) -> bool {
    address_of(a) != address_of(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_variable_has_nonzero_address() {
        let x: i32 = 42;
        assert_ne!(address_of(&x), 0);
    }

    #[test]
    fn test_float_has_nonzero_address() {
        let pi: f64 = 3.14;
        assert_ne!(address_of(&pi), 0);
    }

    #[test]
    fn test_two_distinct_stack_vars_have_different_addresses() {
        let a: i32 = 1;
        let b: i32 = 2;
        assert!(addresses_differ(&a, &b));
    }

    #[test]
    fn test_reference_address_matches_original() {
        let x: i32 = 99;
        let r = &x;
        // The address seen through r is identical to x's own address.
        assert_eq!(address_of(&x), address_of(r));
    }

    #[test]
    fn test_boxed_value_has_nonzero_address() {
        // Deref through Box to reach the heap-allocated data.
        let b: Box<i32> = Box::new(42);
        assert_ne!(address_of(&*b), 0);
    }

    #[test]
    fn test_boxed_value_differs_from_stack_var() {
        let stack: i32 = 7;
        let heap: Box<i32> = Box::new(7);
        // Same value, different storage locations.
        assert_ne!(address_of(&stack), address_of(&*heap));
    }

    #[test]
    fn test_different_boxes_have_different_addresses() {
        let a: Box<i32> = Box::new(1);
        let b: Box<i32> = Box::new(1);
        // Independent heap allocations live at distinct addresses.
        assert_ne!(address_of(&*a), address_of(&*b));
    }

    #[test]
    fn test_address_is_stable_across_reborrow() {
        let x: u64 = 0xDEAD_BEEF;
        let addr1 = address_of(&x);
        let addr2 = address_of(&x);
        assert_eq!(addr1, addr2);
    }
}
