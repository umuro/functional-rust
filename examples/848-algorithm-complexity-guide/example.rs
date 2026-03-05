/// Algorithm Complexity Guide — Rust Reference.
///
/// Demonstrates each complexity class with idiomatic Rust code.
/// Includes empirical timing and Rust-specific complexity notes.

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

// ─── O(1) — Constant time ────────────────────────────────────────────────────

/// O(1): Direct array access, HashMap lookup.
fn constant_access(v: &[i32], i: usize) -> i32 { v[i] }

fn hashmap_lookup(map: &HashMap<&str, i32>, key: &str) -> Option<i32> {
    map.get(key).copied()
}

// ─── O(log n) — Logarithmic ──────────────────────────────────────────────────

/// O(log n): Binary search. Each step halves the search space.
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let (mut lo, mut hi) = (0usize, arr.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}

// ─── O(n) — Linear ──────────────────────────────────────────────────────────

/// O(n): Single pass through data.
fn linear_max(v: &[i32]) -> Option<i32> { v.iter().copied().max() }

fn linear_contains(v: &[i32], target: i32) -> bool { v.contains(&target) }

// ─── O(n log n) — Linearithmic ──────────────────────────────────────────────

/// O(n log n): Standard sort. Rust's sort is TimSort (stable) or pdqsort (unstable).
fn sort_demo(mut v: Vec<i32>) -> Vec<i32> {
    v.sort_unstable(); // slightly faster when stability not needed
    v
}

/// O(n log n): Count distinct elements via BTreeMap (sorted order).
fn count_distinct_sorted(v: &[i32]) -> BTreeMap<i32, usize> {
    let mut map = BTreeMap::new();
    for &x in v { *map.entry(x).or_insert(0) += 1; }
    map
}

// ─── O(n²) — Quadratic ──────────────────────────────────────────────────────

/// O(n²): Insertion sort — optimal for nearly-sorted or tiny arrays (n < ~32).
fn insertion_sort(v: &mut [i32]) {
    for i in 1..v.len() {
        let key = v[i];
        let mut j = i;
        while j > 0 && v[j - 1] > key {
            v[j] = v[j - 1];
            j -= 1;
        }
        v[j] = key;
    }
}

/// O(n²): All pairs — useful when n is small or pairs are sparse.
fn all_pairs_sum(v: &[i32]) -> Vec<i32> {
    v.iter().flat_map(|&a| v.iter().map(move |&b| a + b)).collect()
}

// ─── O(2ⁿ) — Exponential ────────────────────────────────────────────────────

/// O(2ⁿ): Generate all subsets.
fn all_subsets(v: &[i32]) -> Vec<Vec<i32>> {
    let n = v.len();
    (0u64..1 << n)
        .map(|mask| (0..n).filter(|&i| mask >> i & 1 == 1).map(|i| v[i]).collect())
        .collect()
}

// ─── Complexity class summary ─────────────────────────────────────────────────

/// Demonstrate master theorem recurrences:
/// T(n) = a·T(n/b) + f(n)
///
/// Case 1: f(n) = O(n^(log_b a - ε)) → T(n) = Θ(n^(log_b a))
/// Case 2: f(n) = Θ(n^(log_b a))      → T(n) = Θ(n^(log_b a) · log n)
/// Case 3: f(n) = Ω(n^(log_b a + ε)) → T(n) = Θ(f(n))
///
/// Examples:
///   Merge sort:     a=2, b=2, f=O(n)    → Case 2 → O(n log n)
///   Binary search:  a=1, b=2, f=O(1)    → Case 2 → O(log n)
///   Strassen:       a=7, b=2, f=O(n²)   → Case 1 → O(n^2.807)
///   Linear search:  a=1, b=2, f=O(n)    → Case 3 → O(n)

fn print_complexity_table() {
    println!("\n╔══════════════════════╦═══════════╦═══════════════════════════╗");
    println!("║ Class                ║ n=1000    ║ Rust context              ║");
    println!("╠══════════════════════╬═══════════╬═══════════════════════════╣");
    println!("║ O(1)                 ║ 1         ║ arr[i], HashMap::get      ║");
    println!("║ O(log n)             ║ 10        ║ binary_search, BTreeMap   ║");
    println!("║ O(n)                 ║ 1,000     ║ iter, contains, max       ║");
    println!("║ O(n log n)           ║ 10,000    ║ sort, sort_unstable        ║");
    println!("║ O(n²)               ║ 10⁶      ║ nested loops (small n ok) ║");
    println!("║ O(n³)               ║ 10⁹      ║ Floyd-Warshall (n<500)    ║");
    println!("║ O(2ⁿ)               ║ 10³⁰⁰    ║ subsets (only n<25)       ║");
    println!("╚══════════════════════╩═══════════╩═══════════════════════════╝");
}

/// Empirical timing comparison: O(n log n) sort vs O(n²) insertion sort.
fn timing_demo() {
    let sizes = [100usize, 1000, 5000];
    println!("\n── Empirical timing ──");
    for &n in &sizes {
        let data: Vec<i32> = (0..n as i32).rev().collect(); // worst case: reverse sorted

        let mut v1 = data.clone();
        let t1 = Instant::now();
        insertion_sort(&mut v1);
        let d1 = t1.elapsed();

        let t2 = Instant::now();
        let _v2 = sort_demo(data);
        let d2 = t2.elapsed();

        println!("  n={n:>5}: insertion_sort={d1:>10?}  std::sort={d2:>10?}  ratio={:.1}x",
            d1.as_nanos() as f64 / d2.as_nanos().max(1) as f64);
    }
}

fn main() {
    // O(1)
    let arr = vec![10, 20, 30, 40, 50];
    println!("O(1) arr[2] = {}", constant_access(&arr, 2));

    // O(log n)
    let sorted = vec![1, 3, 5, 7, 9, 11, 13];
    println!("O(log n) binary_search(7) = {:?}", binary_search(&sorted, 7));

    // O(n)
    println!("O(n) max = {:?}", linear_max(&arr));

    // O(n log n)
    let unsorted = vec![5, 3, 8, 1, 9, 2];
    println!("O(n log n) sort = {:?}", sort_demo(unsorted));

    // O(n²)
    let mut v = vec![5, 3, 8, 1, 9, 2];
    insertion_sort(&mut v);
    println!("O(n²) insertion_sort = {v:?}");

    // O(2ⁿ) — only for tiny n!
    let small = vec![1, 2, 3];
    println!("O(2ⁿ) all_subsets([1,2,3]) = {} subsets", all_subsets(&small).len());

    print_complexity_table();
    timing_demo();

    println!("\n── Rust complexity notes ──");
    println!("  Vec::push:      amortised O(1) — doubles capacity when full");
    println!("  HashMap::get:   O(1) average (SipHash), O(n) worst case");
    println!("  BTreeMap::get:  O(log n) always — ordered, cache-friendlier for iteration");
    println!("  Iterator chain: lazy O(n), no intermediate allocations");
    println!("  sort_unstable:  O(n log n) worst case, faster than sort in practice");
    println!("  Advice: profile first, then optimise — LLVM often surprises you");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant() {
        assert_eq!(constant_access(&[10, 20, 30], 1), 20);
    }

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, 5), Some(2));
        assert_eq!(binary_search(&arr, 6), None);
        assert_eq!(binary_search(&arr, 1), Some(0));
        assert_eq!(binary_search(&arr, 9), Some(4));
    }

    #[test]
    fn test_linear_max() {
        assert_eq!(linear_max(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(9));
        assert_eq!(linear_max(&[]), None);
    }

    #[test]
    fn test_sort() {
        assert_eq!(sort_demo(vec![5, 3, 8, 1, 9]), vec![1, 3, 5, 8, 9]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![5, 3, 8, 1, 9, 2];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_all_subsets_count() {
        assert_eq!(all_subsets(&[1, 2, 3]).len(), 8); // 2³
        assert_eq!(all_subsets(&[1, 2, 3, 4]).len(), 16); // 2⁴
    }

    #[test]
    fn test_insertion_sort_matches_std() {
        let data = vec![7i32, 3, 9, 1, 5, 8, 2, 6, 4];
        let mut v1 = data.clone();
        let v2 = sort_demo(data);
        insertion_sort(&mut v1);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_complexity_empirical() {
        // Verify that sort is actually O(n log n) in practice: measure ratio
        let n = 1000;
        let data: Vec<i32> = (0..n as i32).rev().collect();

        let mut v_ins = data.clone();
        let t1 = Instant::now();
        insertion_sort(&mut v_ins);
        let ins_ns = t1.elapsed().as_nanos();

        let t2 = Instant::now();
        let _v_std = sort_demo(data);
        let std_ns = t2.elapsed().as_nanos();

        // std sort should be faster for n=1000
        // (Not guaranteed in tests, but overwhelmingly true)
        println!("insertion_sort: {ins_ns}ns, std::sort: {std_ns}ns");
        // Don't assert timing — flaky in CI; just verify correctness
        assert!(v_ins.windows(2).all(|w| w[0] <= w[1]));
    }
}
