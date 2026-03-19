#![allow(clippy::all)]
// Example 127: Const Functions — Compile-Time Computation
// Rust's `const fn` enables true compile-time evaluation, unlike OCaml which
// evaluates module-level expressions at program start (runtime initialization).

// Approach 1: Iterative const fn — const contexts cannot use recursion with stack growth,
// so we use loops (allowed in const fn since Rust 1.46).
pub const fn fibonacci(n: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    let mut i = 0u64;
    while i < n {
        let temp = b;
        b = a + b;
        a = temp;
        i += 1;
    }
    a
}

// These are evaluated at compile time — values baked directly into the binary.
pub const FIB_10: u64 = fibonacci(10);
pub const FIB_20: u64 = fibonacci(20);
pub const FIB_30: u64 = fibonacci(30);

// Approach 2: const fn for integer exponentiation using binary exponentiation.
pub const fn pow_int(mut base: u64, mut exp: u32) -> u64 {
    let mut acc = 1u64;
    while exp > 0 {
        if exp % 2 == 1 {
            acc *= base;
        }
        base *= base;
        exp /= 2;
    }
    acc
}

pub const TWO_TO_16: u64 = pow_int(2, 16);
pub const TEN_TO_6: u64 = pow_int(10, 6);

// Approach 3: Build a lookup table at compile time.
// const fn can populate fixed-size arrays — the result lives in read-only memory.
pub const fn build_square_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        table[i] = (i * i) as u32;
        i += 1;
    }
    table
}

pub const SQUARE_TABLE: [u32; 256] = build_square_table();

// Approach 4: const fn for buffer-size computations — common in embedded/protocol code.
pub const fn align_up(size: usize, align: usize) -> usize {
    (size + align - 1) & !(align - 1)
}

pub const PACKET_SIZE: usize = 1024;
pub const ALIGNED_PACKET: usize = align_up(PACKET_SIZE, 64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_known_values() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn test_fibonacci_constants_match_runtime() {
        // The constants were computed at compile time; verify they match runtime calls.
        assert_eq!(FIB_10, fibonacci(10));
        assert_eq!(FIB_20, fibonacci(20));
        assert_eq!(FIB_30, fibonacci(30));
    }

    #[test]
    fn test_pow_int() {
        assert_eq!(pow_int(2, 0), 1);
        assert_eq!(pow_int(2, 10), 1024);
        assert_eq!(pow_int(10, 3), 1000);
        assert_eq!(TWO_TO_16, 65536);
        assert_eq!(TEN_TO_6, 1_000_000);
    }

    #[test]
    fn test_square_table() {
        assert_eq!(SQUARE_TABLE[0], 0);
        assert_eq!(SQUARE_TABLE[1], 1);
        assert_eq!(SQUARE_TABLE[15], 225);
        assert_eq!(SQUARE_TABLE[255], 65025);
        // All entries must satisfy the square property.
        for (i, &val) in SQUARE_TABLE.iter().enumerate() {
            assert_eq!(val, (i * i) as u32, "mismatch at index {i}");
        }
    }

    #[test]
    fn test_align_up() {
        assert_eq!(align_up(0, 64), 0);
        assert_eq!(align_up(1, 64), 64);
        assert_eq!(align_up(64, 64), 64);
        assert_eq!(align_up(65, 64), 128);
        assert_eq!(ALIGNED_PACKET, 1024); // 1024 is already 64-byte aligned
    }
}
