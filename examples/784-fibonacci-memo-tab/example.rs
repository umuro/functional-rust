// 784. Fibonacci: Memoisation vs Tabulation DP
// Four approaches: naive, top-down memo, bottom-up tabulation, space-optimised

use std::collections::HashMap;

// ── 1. Naive recursion — O(2^n), for comparison only ─────────────────────────

pub fn fib_naive(n: u32) -> u64 {
    match n { 0 => 0, 1 => 1, n => fib_naive(n-1) + fib_naive(n-2) }
}

// ── 2. Top-down memoisation — O(n) time, O(n) space ──────────────────────────

pub fn fib_memo_inner(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    if let Some(&v) = memo.get(&n) { return v; }
    let v = fib_memo_inner(n-1, memo) + fib_memo_inner(n-2, memo);
    memo.insert(n, v);
    v
}

pub fn fib_memo(n: u64) -> u64 {
    fib_memo_inner(n, &mut HashMap::new())
}

// ── 3. Bottom-up tabulation — O(n) time, O(n) space ──────────────────────────

pub fn fib_tab(n: usize) -> u64 {
    if n == 0 { return 0; }
    let mut t = vec![0u64; n + 1];
    t[1] = 1;
    for i in 2..=n {
        t[i] = t[i-1] + t[i-2];
    }
    t[n]
}

// ── 4. Space-optimised — O(1) space (two variables) ──────────────────────────

pub fn fib_opt(n: usize) -> u64 {
    match n { 0 => return 0, 1 => return 1, _ => {} }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

// ── 5. Iterator-based (infinite stream, Rust idiom) ───────────────────────────

pub struct FibIter { a: u64, b: u64 }
impl FibIter {
    pub fn new() -> Self { Self { a: 0, b: 1 } }
}
impl Iterator for FibIter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let val = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(val)
    }
}

// ── Benchmark comparison ───────────────────────────────────────────────────────

use std::time::Instant;

fn time_it<F: Fn() -> u64>(name: &str, f: F) {
    let t = Instant::now();
    let result = f();
    let elapsed = t.elapsed();
    println!("{name:25} = {result:20}  [{elapsed:?}]");
}

fn main() {
    println!("=== Correctness check (all should agree) ===");
    for n in [0, 1, 2, 10, 30, 50] {
        let a = fib_memo(n as u64);
        let b = fib_tab(n);
        let c = fib_opt(n);
        assert_eq!(a, b as u64);
        assert_eq!(a, c as u64);
        println!("  fib({n:2}) = {a}");
    }

    println!("\n=== Performance ===");
    time_it("fib_naive(35)",  || fib_naive(35));
    time_it("fib_memo(1000)", || fib_memo(1000) % 1_000_000_007);
    time_it("fib_tab(1000)",  || fib_tab(1000)  % 1_000_000_007);
    time_it("fib_opt(1000)",  || fib_opt(1000)  % 1_000_000_007);

    println!("\n=== Iterator (first 15 Fibonacci numbers) ===");
    let fibs: Vec<u64> = FibIter::new().take(15).collect();
    println!("{fibs:?}");

    println!("\n=== First Fibonacci > 1,000,000 ===");
    let big = FibIter::new().find(|&n| n > 1_000_000).unwrap();
    println!("{big}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_agree(n: usize) {
        let a = fib_memo(n as u64);
        let b = fib_tab(n) as u64;
        let c = fib_opt(n) as u64;
        assert_eq!(a, b, "memo vs tab at n={n}");
        assert_eq!(b, c, "tab vs opt at n={n}");
    }

    #[test]
    fn base_cases() { all_agree(0); all_agree(1); }
    #[test]
    fn small_values() { for i in 2..=20 { all_agree(i); } }
    #[test]
    fn larger_values() { all_agree(50); all_agree(80); }

    #[test]
    fn naive_matches_opt() {
        for i in 0..15 { assert_eq!(fib_naive(i as u32), fib_opt(i)); }
    }

    #[test]
    fn iter_correct() {
        let v: Vec<u64> = FibIter::new().take(8).collect();
        assert_eq!(v, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }
}
