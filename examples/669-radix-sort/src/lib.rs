//! # Radix Sort
//!
//! Sorts by processing digits from least to most significant.
//! Time: O(d * (n + k)) where d = digits, k = radix

/// LSD radix sort for non-negative integers
pub fn radix_sort(arr: &mut [u32]) {
    if arr.is_empty() { return; }
    
    let max = *arr.iter().max().unwrap();
    let mut exp = 1u32;
    
    while max / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_by_digit(arr: &mut [u32], exp: u32) {
    let n = arr.len();
    let mut output = vec![0u32; n];
    let mut count = [0usize; 10];
    
    for &x in arr.iter() { count[((x / exp) % 10) as usize] += 1; }
    for i in 1..10 { count[i] += count[i - 1]; }
    
    for &x in arr.iter().rev() {
        let digit = ((x / exp) % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = x;
    }
    
    arr.copy_from_slice(&output);
}

/// Radix sort for strings (MSD)
pub fn radix_sort_strings(arr: &mut [String]) {
    if arr.len() <= 1 { return; }
    msd_sort(arr, 0);
}

fn msd_sort(arr: &mut [String], d: usize) {
    if arr.len() <= 1 { return; }
    
    let mut buckets: Vec<Vec<String>> = vec![Vec::new(); 257]; // 256 chars + empty
    
    for s in arr.iter() {
        let idx = s.as_bytes().get(d).map(|&b| b as usize + 1).unwrap_or(0);
        buckets[idx].push(s.clone());
    }
    
    let mut i = 0;
    for (bucket_idx, bucket) in buckets.iter_mut().enumerate() {
        if bucket.len() > 1 && bucket_idx > 0 {
            msd_sort(bucket, d + 1);
        }
        for s in bucket.drain(..) {
            arr[i] = s;
            i += 1;
        }
    }
}

/// Binary radix sort
pub fn binary_radix_sort(arr: &mut [u32]) {
    if arr.is_empty() { return; }
    
    for bit in (0..32).rev() {
        let mask = 1u32 << bit;
        let (zeros, ones): (Vec<_>, Vec<_>) = arr.iter().partition(|&&x| x & mask == 0);
        let merged: Vec<_> = zeros.into_iter().chain(ones).cloned().collect();
        arr.copy_from_slice(&merged);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort() {
        let mut arr = vec![170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }

    #[test]
    fn test_radix_sort_strings() {
        let mut arr: Vec<String> = vec!["she", "sells", "seashells", "by", "the", "sea"]
            .into_iter().map(String::from).collect();
        radix_sort_strings(&mut arr);
        assert!(arr.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_binary_radix() {
        let mut arr = vec![170, 45, 75, 90, 802, 24, 2, 66];
        binary_radix_sort(&mut arr);
        assert_eq!(arr, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<u32> = vec![];
        radix_sort(&mut arr);
        assert!(arr.is_empty());
    }
}
