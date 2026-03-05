/// Randomised Quickselect — O(n) average time.
///
/// Find the k-th smallest element without fully sorting.
/// Random pivot avoids worst-case O(n²) on sorted input.

/// Simple XorShift PRNG (no external crates needed).
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self { Rng(seed) }
    fn next_usize(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 as usize) % n
    }
}

/// Lomuto partition: rearrange arr[lo..=hi] around a random pivot.
/// Returns the final pivot index.
fn partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize, rng: &mut Rng) -> usize {
    let pivot_idx = lo + rng.next_usize(hi - lo + 1);
    arr.swap(pivot_idx, hi);

    let mut store = lo;
    for i in lo..hi {
        if arr[i] < arr[hi] {
            arr.swap(store, i);
            store += 1;
        }
    }
    arr.swap(store, hi);
    store
}

/// Quickselect: return the k-th smallest element (0-indexed k).
/// Mutates the slice but does NOT fully sort it.
pub fn quickselect<T: Ord>(arr: &mut [T], mut lo: usize, mut hi: usize, k: usize) -> &T {
    let mut rng = Rng::new(0xdeadbeef);
    loop {
        if lo == hi { return &arr[lo]; }
        let p = partition(arr, lo, hi, &mut rng);
        match p.cmp(&k) {
            std::cmp::Ordering::Equal => return &arr[p],
            std::cmp::Ordering::Greater => hi = p - 1,
            std::cmp::Ordering::Less => lo = p + 1,
        }
    }
}

/// k-th smallest (1-indexed). Does not modify original array.
pub fn kth_smallest<T: Ord + Clone>(arr: &[T], k: usize) -> T {
    assert!(k >= 1 && k <= arr.len(), "k out of range");
    let mut copy = arr.to_vec();
    let n = copy.len();
    quickselect(&mut copy, 0, n - 1, k - 1).clone()
}

/// Median (1-indexed midpoint for odd, average for even).
fn median_f64(arr: &[f64]) -> f64 {
    let n = arr.len();
    if n % 2 == 1 {
        kth_smallest(arr, (n + 1) / 2)
    } else {
        (kth_smallest(arr, n / 2) + kth_smallest(arr, n / 2 + 1)) / 2.0
    }
}

fn main() {
    let arr = [7i32, 10, 4, 3, 20, 15];
    println!("Array: {arr:?}");
    for k in 1..=arr.len() {
        println!("  {k}-th smallest: {}", kth_smallest(&arr, k));
    }

    let flarr: Vec<f64> = arr.iter().map(|&x| x as f64).collect();
    println!("Median: {}", median_f64(&flarr));

    // Large array: verify k-th smallest matches sorted
    let big: Vec<i32> = (0..100).rev().collect(); // [99, 98, ..., 0]
    println!("\n50th smallest of 0..99: {}", kth_smallest(&big, 50)); // 49
    println!("1st smallest of 0..99:  {}", kth_smallest(&big, 1));   // 0
    println!("100th smallest of 0..99: {}", kth_smallest(&big, 100)); // 99
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_sorted_input() {
        let arr: Vec<i32> = (1..=10).collect();
        for k in 1..=10 {
            assert_eq!(kth_smallest(&arr, k), k as i32);
        }
    }

    #[test]
    fn test_kth_reverse_input() {
        let arr: Vec<i32> = (1..=10).rev().collect();
        for k in 1..=10 {
            assert_eq!(kth_smallest(&arr, k), k as i32);
        }
    }

    #[test]
    fn test_kth_random_input() {
        let arr = vec![7i32, 10, 4, 3, 20, 15];
        let mut sorted = arr.clone();
        sorted.sort();
        for k in 1..=arr.len() {
            assert_eq!(kth_smallest(&arr, k), sorted[k - 1]);
        }
    }

    #[test]
    fn test_kth_duplicates() {
        let arr = vec![3i32, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let mut sorted = arr.clone();
        sorted.sort();
        for k in 1..=arr.len() {
            assert_eq!(kth_smallest(&arr, k), sorted[k - 1]);
        }
    }

    #[test]
    fn test_single_element() {
        assert_eq!(kth_smallest(&[42i32], 1), 42);
    }

    #[test]
    fn test_median() {
        // Odd length: exact median
        assert_eq!(median_f64(&[1.0, 5.0, 3.0, 2.0, 4.0]), 3.0);
        // Even length: average of two middle
        assert_eq!(median_f64(&[1.0, 2.0, 3.0, 4.0]), 2.5);
    }
}
