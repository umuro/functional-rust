//! # Counting Sort
//!
//! Non-comparison sort for integers in a known range.
//! Time: O(n + k), Space: O(k) where k is range size.

/// Counting sort for non-negative integers
pub fn counting_sort(arr: &mut [usize]) {
    if arr.is_empty() { return; }
    
    let max = *arr.iter().max().unwrap();
    let mut count = vec![0; max + 1];
    
    for &x in arr.iter() { count[x] += 1; }
    
    let mut i = 0;
    for (val, &cnt) in count.iter().enumerate() {
        for _ in 0..cnt {
            arr[i] = val;
            i += 1;
        }
    }
}

/// Stable counting sort (preserves order of equal elements)
pub fn counting_sort_stable<T: Clone>(arr: &[T], key: impl Fn(&T) -> usize, max_key: usize) -> Vec<T> {
    let mut count = vec![0; max_key + 1];
    
    for item in arr { count[key(item)] += 1; }
    
    // Cumulative count
    for i in 1..count.len() { count[i] += count[i - 1]; }
    
    let mut output = vec![arr[0].clone(); arr.len()];
    for item in arr.iter().rev() {
        let k = key(item);
        count[k] -= 1;
        output[count[k]] = item.clone();
    }
    output
}

/// Sort with negative numbers
pub fn counting_sort_signed(arr: &mut [i32]) {
    if arr.is_empty() { return; }
    
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    let range = (max - min + 1) as usize;
    
    let mut count = vec![0; range];
    for &x in arr.iter() { count[(x - min) as usize] += 1; }
    
    let mut i = 0;
    for (offset, &cnt) in count.iter().enumerate() {
        let val = offset as i32 + min;
        for _ in 0..cnt {
            arr[i] = val;
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut arr = vec![4, 2, 2, 8, 3, 3, 1];
        counting_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 2, 3, 3, 4, 8]);
    }

    #[test]
    fn test_counting_sort_stable() {
        let arr = vec![(3, 'a'), (1, 'b'), (3, 'c'), (1, 'd')];
        let sorted = counting_sort_stable(&arr, |x| x.0, 3);
        assert_eq!(sorted[0].1, 'b');
        assert_eq!(sorted[1].1, 'd');
    }

    #[test]
    fn test_counting_sort_signed() {
        let mut arr = vec![4, -2, 2, -8, 3, -3, 1];
        counting_sort_signed(&mut arr);
        assert_eq!(arr, vec![-8, -3, -2, 1, 2, 3, 4]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<usize> = vec![];
        counting_sort(&mut arr);
        assert!(arr.is_empty());
    }
}
