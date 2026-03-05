//! # Bucket Sort
//!
//! Distributes elements into buckets, sorts each, then concatenates.
//! Time: O(n + k) average, Space: O(n + k)

/// Bucket sort for floats in [0, 1)
pub fn bucket_sort_unit(arr: &mut [f64]) {
    if arr.len() <= 1 { return; }
    
    let n = arr.len();
    let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n];
    
    for &x in arr.iter() {
        let idx = (x * n as f64) as usize;
        let idx = idx.min(n - 1);
        buckets[idx].push(x);
    }
    
    for bucket in &mut buckets {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    
    let mut i = 0;
    for bucket in buckets {
        for x in bucket {
            arr[i] = x;
            i += 1;
        }
    }
}

/// Generic bucket sort with custom range
pub fn bucket_sort(arr: &mut [f64], min: f64, max: f64) {
    if arr.len() <= 1 || min >= max { return; }
    
    let n = arr.len();
    let range = max - min;
    let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n];
    
    for &x in arr.iter() {
        let normalized = (x - min) / range;
        let idx = ((normalized * n as f64) as usize).min(n - 1);
        buckets[idx].push(x);
    }
    
    for bucket in &mut buckets {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    
    let mut i = 0;
    for bucket in buckets {
        for x in bucket { arr[i] = x; i += 1; }
    }
}

/// Bucket sort for integers
pub fn bucket_sort_int(arr: &mut [i32], num_buckets: usize) {
    if arr.len() <= 1 { return; }
    
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    
    if min == max { return; }
    
    let range = (max - min + 1) as f64;
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); num_buckets];
    
    for &x in arr.iter() {
        let idx = (((x - min) as f64 / range) * num_buckets as f64) as usize;
        let idx = idx.min(num_buckets - 1);
        buckets[idx].push(x);
    }
    
    for bucket in &mut buckets { bucket.sort(); }
    
    let mut i = 0;
    for bucket in buckets {
        for x in bucket { arr[i] = x; i += 1; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_sort_unit() {
        let mut arr = vec![0.42, 0.32, 0.23, 0.52, 0.25, 0.47, 0.51];
        bucket_sort_unit(&mut arr);
        assert!(arr.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_bucket_sort_range() {
        let mut arr = vec![4.2, 3.2, 2.3, 5.2, 2.5, 4.7, 5.1];
        bucket_sort(&mut arr, 2.0, 6.0);
        assert!(arr.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_bucket_sort_int() {
        let mut arr = vec![42, 32, 23, 52, 25, 47, 51];
        bucket_sort_int(&mut arr, 5);
        assert_eq!(arr, vec![23, 25, 32, 42, 47, 51, 52]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<f64> = vec![];
        bucket_sort_unit(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_single() {
        let mut arr = vec![0.5];
        bucket_sort_unit(&mut arr);
        assert_eq!(arr, vec![0.5]);
    }
}
