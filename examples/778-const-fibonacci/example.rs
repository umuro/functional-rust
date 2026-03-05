// 778. Fibonacci Computed at Compile Time
// const fn table: embedded in binary, O(1) lookup

// ── const fn: iterative (required in const context) ───────────────────────────

const fn fib_iter(n: u32) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut i = 2;
    while i <= n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    b
}

// ── Compile-time table of first 93 Fibonacci numbers ─────────────────────────

// fib(93) is the last one that fits in u64
const FIB_TABLE: [u64; 93] = {
    let mut t = [0u64; 93];
    t[0] = 0;
    t[1] = 1;
    let mut i = 2usize;
    while i < 93 {
        t[i] = t[i-1] + t[i-2];
        i += 1;
    }
    t
};

/// O(1) Fibonacci lookup — table is in the binary's read-only segment
pub fn fib(n: usize) -> Option<u64> {
    FIB_TABLE.get(n).copied()
}

// ── Runtime recursive (for comparison) ────────────────────────────────────────

pub fn fib_recursive(n: u32) -> u64 {
    match n { 0 => 0, 1 => 1, n => fib_recursive(n-1) + fib_recursive(n-2) }
}

// ── Memoized runtime (for comparison) ─────────────────────────────────────────

pub fn fib_memoized(n: usize) -> u64 {
    let mut memo = vec![0u64; n + 1];
    if n >= 1 { memo[1] = 1; }
    for i in 2..=n {
        memo[i] = memo[i-1] + memo[i-2];
    }
    memo[n]
}

// ── Matrix exponentiation (for very large n) ──────────────────────────────────

/// F(n) via matrix exponentiation — O(log n), works for any n
pub fn fib_matrix(n: u64) -> u128 {
    if n == 0 { return 0; }
    fn mat_mul(a: [[u128; 2]; 2], b: [[u128; 2]; 2]) -> [[u128; 2]; 2] {
        [[a[0][0]*b[0][0] + a[0][1]*b[1][0], a[0][0]*b[0][1] + a[0][1]*b[1][1]],
         [a[1][0]*b[0][0] + a[1][1]*b[1][0], a[1][0]*b[0][1] + a[1][1]*b[1][1]]]
    }
    fn mat_pow(mut m: [[u128; 2]; 2], mut p: u64) -> [[u128; 2]; 2] {
        let mut result = [[1, 0], [0, 1]]; // identity
        while p > 0 {
            if p & 1 == 1 { result = mat_mul(result, m); }
            m = mat_mul(m, m);
            p >>= 1;
        }
        result
    }
    let m = mat_pow([[1, 1], [1, 0]], n);
    m[0][1]
}

fn main() {
    println!("Compile-time Fibonacci table (first 20):");
    for i in 0..20 {
        println!("  fib({i:2}) = {}", FIB_TABLE[i]);
    }

    println!("\nLookup via fib():");
    println!("  fib(50) = {:?}", fib(50));
    println!("  fib(92) = {:?}", fib(92)); // largest u64 fib
    println!("  fib(93) = {:?}", fib(93)); // None — out of range

    println!("\nMatrix exponentiation (arbitrary size, u128):");
    println!("  fib(100) = {}", fib_matrix(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_starts_correct() {
        assert_eq!(FIB_TABLE[0], 0);
        assert_eq!(FIB_TABLE[1], 1);
        assert_eq!(FIB_TABLE[2], 1);
        assert_eq!(FIB_TABLE[3], 2);
        assert_eq!(FIB_TABLE[10], 55);
    }

    #[test]
    fn fib_iter_matches_table() {
        for i in 0..20u32 {
            assert_eq!(fib_iter(i), FIB_TABLE[i as usize], "fib({i})");
        }
    }

    #[test]
    fn recursive_matches_table() {
        for i in 0..15u32 {
            assert_eq!(fib_recursive(i), FIB_TABLE[i as usize]);
        }
    }

    #[test]
    fn out_of_range_returns_none() {
        assert_eq!(fib(93), None);
    }

    #[test]
    fn matrix_fib_correct() {
        for i in 0u64..20 {
            assert_eq!(fib_matrix(i), FIB_TABLE[i as usize] as u128, "matrix fib({i})");
        }
    }
}
