//! # Const Lookup Table
//!
//! Pre-computed lookup tables at compile time.

/// Sine lookup table (256 entries, 0-2π)
pub const fn generate_sin_table() -> [i16; 256] {
    let mut table = [0i16; 256];
    let mut i = 0;
    while i < 256 {
        // Approximate sin using Taylor series
        let angle = (i as f64) * std::f64::consts::PI * 2.0 / 256.0;
        let sin_val = const_sin(angle);
        table[i] = (sin_val * 32767.0) as i16;
        i += 1;
    }
    table
}

/// Approximate sin at compile time using Taylor series
const fn const_sin(x: f64) -> f64 {
    // Normalize to -π to π
    let mut x = x;
    while x > std::f64::consts::PI {
        x -= 2.0 * std::f64::consts::PI;
    }
    while x < -std::f64::consts::PI {
        x += 2.0 * std::f64::consts::PI;
    }

    // Taylor: sin(x) ≈ x - x³/3! + x⁵/5! - x⁷/7! + ...
    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let x7 = x5 * x2;

    x - x3 / 6.0 + x5 / 120.0 - x7 / 5040.0
}

/// CRC32 lookup table
pub const fn generate_crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

/// Powers of 2 lookup
pub const fn generate_pow2_table() -> [u64; 64] {
    let mut table = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        table[i] = 1u64 << i;
        i += 1;
    }
    table
}

/// Factorial lookup (up to 20!)
pub const fn generate_factorial_table() -> [u64; 21] {
    let mut table = [1u64; 21];
    let mut i = 1;
    while i <= 20 {
        table[i] = table[i - 1] * (i as u64);
        i += 1;
    }
    table
}

// Pre-computed lookup tables
pub const CRC32_TABLE: [u32; 256] = generate_crc32_table();
pub const POW2_TABLE: [u64; 64] = generate_pow2_table();
pub const FACTORIAL_TABLE: [u64; 21] = generate_factorial_table();

/// Fast CRC32 using lookup table
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFF;
    for &byte in data {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[index];
    }
    !crc
}

/// Fast factorial using lookup
pub const fn factorial_lookup(n: usize) -> Option<u64> {
    if n <= 20 {
        Some(FACTORIAL_TABLE[n])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow2_table() {
        assert_eq!(POW2_TABLE[0], 1);
        assert_eq!(POW2_TABLE[10], 1024);
        assert_eq!(POW2_TABLE[20], 1048576);
    }

    #[test]
    fn test_factorial_table() {
        assert_eq!(FACTORIAL_TABLE[0], 1);
        assert_eq!(FACTORIAL_TABLE[5], 120);
        assert_eq!(FACTORIAL_TABLE[10], 3628800);
        assert_eq!(FACTORIAL_TABLE[20], 2432902008176640000);
    }

    #[test]
    fn test_crc32() {
        let data = b"hello";
        let crc = crc32(data);
        assert_eq!(crc, 0x3610A686);
    }

    #[test]
    fn test_factorial_lookup() {
        assert_eq!(factorial_lookup(5), Some(120));
        assert_eq!(factorial_lookup(21), None);
    }

    // Compile-time verification
    const _: () = assert!(POW2_TABLE[10] == 1024);
    const _: () = assert!(FACTORIAL_TABLE[5] == 120);
}
