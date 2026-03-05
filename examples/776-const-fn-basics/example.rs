// 776. const fn: Compile-Time Computation
// Functions evaluated at compile time — zero runtime cost

// ── const fn definitions ───────────────────────────────────────────────────────

const fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

const fn pow2(n: u32) -> u64 {
    if n == 0 { 1 } else { 2 * pow2(n - 1) }
}

const fn min_u32(a: u32, b: u32) -> u32 {
    if a < b { a } else { b }
}

const fn max_u32(a: u32, b: u32) -> u32 {
    if a > b { a } else { b }
}

/// Integer square root (floor) — pure const
const fn isqrt(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x { x = y; y = (x + n / x) / 2; }
    x
}

/// FNV-1a 32-bit hash of a byte slice — usable in const context
const fn fnv1a_32(bytes: &[u8]) -> u32 {
    let mut hash: u32 = 0x811c9dc5;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u32;
        hash = hash.wrapping_mul(0x01000193);
        i += 1;
    }
    hash
}

// ── Constants computed at compile time ────────────────────────────────────────

const FAC10: u64 = factorial(10);   // 3628800
const FAC15: u64 = factorial(15);   // 1307674368000
const POW2_16: u64 = pow2(16);       // 65536
const SQRT_1000: u64 = isqrt(1000); // 31
const HELLO_HASH: u32 = fnv1a_32(b"hello"); // compile-time FNV hash

// ── Const generic with const fn ───────────────────────────────────────────────

const fn clamp<const LO: i32, const HI: i32>(v: i32) -> i32 {
    if v < LO { LO } else if v > HI { HI } else { v }
}

// ── Compile-time lookup table ─────────────────────────────────────────────────

const POW2_TABLE: [u64; 16] = {
    let mut t = [0u64; 16];
    let mut i = 0usize;
    while i < 16 {
        t[i] = pow2(i as u32);
        i += 1;
    }
    t
};

fn main() {
    println!("10!         = {FAC10}");
    println!("15!         = {FAC15}");
    println!("2^16        = {POW2_16}");
    println!("sqrt(1000)  = {SQRT_1000}");
    println!("FNV(\"hello\") = {HELLO_HASH:#010x}");

    println!("\nPowers of 2:");
    for (i, &v) in POW2_TABLE.iter().enumerate() {
        println!("  2^{i:2} = {v}");
    }

    // const generic clamp — checked at compile time
    println!("\nclamp<0, 100>(150) = {}", clamp::<0, 100>(150));
    println!("clamp<0, 100>(-10) = {}", clamp::<0, 100>(-10));
    println!("clamp<0, 100>(50)  = {}", clamp::<0, 100>(50));

    // Runtime use of const fn
    let n = 7u64;
    println!("\nRuntime factorial(7) = {}", factorial(n)); // same fn, runtime call
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_values() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(FAC10, 3_628_800);
    }

    #[test]
    fn isqrt_correct() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(1), 1);
        assert_eq!(isqrt(4), 2);
        assert_eq!(isqrt(9), 3);
        assert_eq!(isqrt(10), 3); // floor
        assert_eq!(SQRT_1000, 31);
    }

    #[test]
    fn pow2_table_correct() {
        assert_eq!(POW2_TABLE[0], 1);
        assert_eq!(POW2_TABLE[8], 256);
        assert_eq!(POW2_TABLE[15], 32768);
    }

    #[test]
    fn fnv_deterministic() {
        assert_eq!(HELLO_HASH, fnv1a_32(b"hello"));
    }
}
