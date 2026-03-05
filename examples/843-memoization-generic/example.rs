/// Generic Memoisation with HashMap cache.
///
/// Transforms overlapping-subproblem recursion from exponential to polynomial.
/// Fibonacci, coin change, edit distance.

use std::collections::HashMap;

/// Fibonacci with manual HashMap memoisation.
fn fib(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    if let Some(&v) = cache.get(&n) { return v; }
    let v = fib(n - 1, cache) + fib(n - 2, cache);
    cache.insert(n, v);
    v
}

/// Coin change: minimum number of coins to make `amount`.
fn coin_change(coins: &[u64], amount: u64) -> Option<u64> {
    let mut cache: HashMap<u64, Option<u64>> = HashMap::new();
    coin_change_rec(coins, amount, &mut cache)
}

fn coin_change_rec(coins: &[u64], amount: u64, cache: &mut HashMap<u64, Option<u64>>) -> Option<u64> {
    if amount == 0 { return Some(0); }
    if let Some(&v) = cache.get(&amount) { return v; }

    let result = coins.iter()
        .filter(|&&c| c <= amount)
        .filter_map(|&c| coin_change_rec(coins, amount - c, cache).map(|n| n + 1))
        .min();

    cache.insert(amount, result);
    result
}

/// Edit distance (Levenshtein) with memoisation.
fn edit_distance(s: &[u8], t: &[u8]) -> usize {
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    edit_rec(s, t, 0, 0, &mut cache)
}

fn edit_rec(
    s: &[u8], t: &[u8],
    i: usize, j: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if i == s.len() { return t.len() - j; }
    if j == t.len() { return s.len() - i; }
    if let Some(&v) = cache.get(&(i, j)) { return v; }

    let v = if s[i] == t[j] {
        edit_rec(s, t, i + 1, j + 1, cache)
    } else {
        1 + edit_rec(s, t, i + 1, j, cache)     // delete
            .min(edit_rec(s, t, i, j + 1, cache)) // insert
            .min(edit_rec(s, t, i + 1, j + 1, cache)) // replace
    };
    cache.insert((i, j), v);
    v
}

/// Longest Common Subsequence with memoisation.
fn lcs(s: &[u8], t: &[u8]) -> usize {
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    lcs_rec(s, t, 0, 0, &mut cache)
}

fn lcs_rec(s: &[u8], t: &[u8], i: usize, j: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if i == s.len() || j == t.len() { return 0; }
    if let Some(&v) = cache.get(&(i, j)) { return v; }
    let v = if s[i] == t[j] {
        1 + lcs_rec(s, t, i + 1, j + 1, cache)
    } else {
        lcs_rec(s, t, i + 1, j, cache).max(lcs_rec(s, t, i, j + 1, cache))
    };
    cache.insert((i, j), v);
    v
}

fn main() {
    let mut cache = HashMap::new();
    println!("Fibonacci (memoised):");
    for n in 0..=15u64 {
        print!("fib({n})={} ", fib(n, &mut cache));
    }
    println!();

    println!("\nCoin change:");
    println!("  coins=[1,5,10,25], amount=41: {:?}", coin_change(&[1, 5, 10, 25], 41));
    println!("  coins=[2], amount=3: {:?}", coin_change(&[2], 3));
    println!("  coins=[1,3,4], amount=6: {:?}", coin_change(&[1, 3, 4], 6));

    println!("\nEdit distance:");
    println!("  'kitten' vs 'sitting': {}", edit_distance(b"kitten", b"sitting")); // 3
    println!("  'abc' vs 'abc': {}", edit_distance(b"abc", b"abc"));               // 0
    println!("  '' vs 'abc': {}", edit_distance(b"", b"abc"));                     // 3

    println!("\nLCS:");
    println!("  'ABCBDAB' vs 'BDCAB': {}", lcs(b"ABCBDAB", b"BDCAB")); // 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let mut cache = HashMap::new();
        assert_eq!(fib(0, &mut cache), 0);
        assert_eq!(fib(1, &mut cache), 1);
        assert_eq!(fib(10, &mut cache), 55);
        assert_eq!(fib(20, &mut cache), 6765);
    }

    #[test]
    fn test_coin_change_basic() {
        assert_eq!(coin_change(&[1, 5, 10, 25], 41), Some(4)); // 25+10+5+1
        assert_eq!(coin_change(&[1, 3, 4], 6), Some(2));       // 3+3
        assert_eq!(coin_change(&[2], 3), None);                 // impossible
    }

    #[test]
    fn test_coin_change_zero() {
        assert_eq!(coin_change(&[1, 5], 0), Some(0));
    }

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance(b"kitten", b"sitting"), 3);
        assert_eq!(edit_distance(b"", b""), 0);
        assert_eq!(edit_distance(b"abc", b"abc"), 0);
        assert_eq!(edit_distance(b"", b"abc"), 3);
        assert_eq!(edit_distance(b"abc", b""), 3);
        assert_eq!(edit_distance(b"a", b"b"), 1);
    }

    #[test]
    fn test_lcs() {
        assert_eq!(lcs(b"ABCBDAB", b"BDCAB"), 4);
        assert_eq!(lcs(b"abc", b"abc"), 3);
        assert_eq!(lcs(b"abc", b"def"), 0);
    }
}
