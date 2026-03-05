// 782. const eval: Limitations and Workarounds
// What works, what doesn't, and how to work around restrictions

// ── ALLOWED in const fn ────────────────────────────────────────────────────────

// ✓ Integer arithmetic
const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

// ✓ Loops and conditionals
const fn sum_range(lo: u64, hi: u64) -> u64 {
    let mut s = 0;
    let mut i = lo;
    while i <= hi { s += i; i += 1; }
    s
}

// ✓ Fixed-size arrays (stack-only, no Vec)
const fn reverse_array<const N: usize>(arr: [u32; N]) -> [u32; N] {
    let mut out = [0u32; N];
    let mut i = 0;
    while i < N { out[i] = arr[N - 1 - i]; i += 1; }
    out
}

// ✓ Bit manipulation
const fn popcount(mut n: u64) -> u32 {
    let mut count = 0;
    while n != 0 { count += (n & 1) as u32; n >>= 1; }
    count
}

// ✓ str operations (limited)
const fn starts_with_prefix(s: &[u8], prefix: &[u8]) -> bool {
    if s.len() < prefix.len() { return false; }
    let mut i = 0;
    while i < prefix.len() {
        if s[i] != prefix[i] { return false; }
        i += 1;
    }
    true
}

// ── WORKAROUNDS for limitations ────────────────────────────────────────────────

// ✗ Vec::new() NOT allowed in const → use fixed array
// const fn build_vec() -> Vec<i32> { ... }  // ERROR

// Workaround: return fixed array, caller converts if needed
const fn squares<const N: usize>() -> [u64; N] {
    let mut out = [0u64; N];
    let mut i = 0;
    while i < N { out[i] = (i * i) as u64; i += 1; }
    out
}

// ✗ String::new() NOT allowed → use &'static str or byte arrays
// const fn concat(a: &str, b: &str) -> String { ... }  // ERROR

// Workaround: precomputed string literals
const GREETING: &str = "hello";  // fine — &str is const
const VERSION_STR: &[u8] = b"v2.0.0";

// ✓ Since Rust 1.82: basic float arithmetic in const fn
// (previously needed workarounds)
#[allow(clippy::approx_constant)]
const fn approx_pi_digits() -> u64 {
    // Integer approximation: floor(π × 10^9)
    3_141_592_653
}

// ✓ const blocks (inline const evaluation, Rust 1.79+)
fn demonstrate_const_block() {
    // const block: evaluated at compile time, result inlined
    let n = const { gcd(48, 18) };  // n is a const 6
    println!("inline const gcd(48,18) = {n}");
}

// ── Constants ─────────────────────────────────────────────────────────────────

const GCD_48_18: u64 = gcd(48, 18);           // 6
const SUM_1_100: u64 = sum_range(1, 100);      // 5050
const SQUARES: [u64; 10] = squares::<10>();
const REVERSED: [u32; 5] = reverse_array([1, 2, 3, 4, 5]);
const POP_255: u32 = popcount(255);            // 8

fn main() {
    println!("gcd(48, 18)   = {GCD_48_18}");
    println!("sum(1..=100)  = {SUM_1_100}");
    println!("squares(0..9) = {SQUARES:?}");
    println!("reversed      = {REVERSED:?}");
    println!("popcount(255) = {POP_255}");

    demonstrate_const_block();

    // Using const fn at runtime (same function, runtime call)
    let runtime_gcd = gcd(1071, 462);
    println!("runtime gcd(1071, 462) = {runtime_gcd}"); // 21

    println!("Version: {}", std::str::from_utf8(VERSION_STR).unwrap());
    println!("PI ≈ {:.9}", approx_pi_digits() as f64 / 1e9);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_correct() {
        assert_eq!(GCD_48_18, 6);
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 13), 1);
    }

    #[test]
    fn sum_range_correct() {
        assert_eq!(SUM_1_100, 5050);
    }

    #[test]
    fn squares_correct() {
        assert_eq!(SQUARES[0], 0);
        assert_eq!(SQUARES[3], 9);
        assert_eq!(SQUARES[9], 81);
    }

    #[test]
    fn reverse_correct() {
        assert_eq!(REVERSED, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn starts_with() {
        assert!(starts_with_prefix(b"hello world", b"hello"));
        assert!(!starts_with_prefix(b"world", b"hello"));
    }
}
