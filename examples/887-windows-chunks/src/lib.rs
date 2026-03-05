// Example 093: Windows and Chunks
// Sliding window algorithms in Rust

// === Approach 1: Windows (overlapping) ===

// Idiomatic Rust: uses built-in .windows(n) for zero-copy overlapping slices
pub fn moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    if window_size == 0 {
        return vec![];
    }
    data.windows(window_size)
        .map(|w| w.iter().sum::<f64>() / window_size as f64)
        .collect()
}

pub fn pairwise_diff(data: &[i32]) -> Vec<i32> {
    data.windows(2).map(|w| w[1] - w[0]).collect()
}

// Returns the center value of every window where it is strictly greater than both neighbors
pub fn local_maxima(data: &[i32]) -> Vec<i32> {
    data.windows(3)
        .filter(|w| w[1] > w[0] && w[1] > w[2])
        .map(|w| w[1])
        .collect()
}

// === Approach 2: Chunks (non-overlapping) ===

// Idiomatic Rust: .chunks(n) slices into non-overlapping blocks; last may be shorter
pub fn chunk_sums(data: &[i32], size: usize) -> Vec<i32> {
    data.chunks(size).map(|c| c.iter().sum()).collect()
}

pub fn chunk_maxes(data: &[i32], size: usize) -> Vec<i32> {
    data.chunks(size)
        .map(|c| *c.iter().max().unwrap())
        .collect()
}

// chunks_exact skips the remainder; access leftover via .remainder()
pub fn chunk_exact_sums(data: &[i32], size: usize) -> (Vec<i32>, &[i32]) {
    let iter = data.chunks_exact(size);
    let remainder = iter.remainder();
    let sums = iter.map(|c| c.iter().sum()).collect();
    (sums, remainder)
}

// === Approach 3: Manual recursive (OCaml-style) ===

// Recursive windows — mirrors the OCaml implementation
pub fn windows_recursive<T: Clone>(slice: &[T], n: usize) -> Vec<Vec<T>> {
    if n == 0 || slice.len() < n {
        return vec![];
    }
    let mut result = vec![slice[..n].to_vec()];
    result.extend(windows_recursive(&slice[1..], n));
    result
}

// Recursive chunks — mirrors OCaml List-based chunking
pub fn chunks_recursive<T: Clone>(slice: &[T], n: usize) -> Vec<Vec<T>> {
    if n == 0 || slice.is_empty() {
        return vec![];
    }
    let end = n.min(slice.len());
    let mut result = vec![slice[..end].to_vec()];
    result.extend(chunks_recursive(&slice[end..], n));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- moving_average ---

    #[test]
    fn test_moving_average_basic() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let result = moving_average(&data, 3);
        assert_eq!(result, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_moving_average_window_equals_len() {
        let data = [1.0, 2.0, 3.0];
        let result = moving_average(&data, 3);
        assert_eq!(result, vec![2.0]);
    }

    #[test]
    fn test_moving_average_window_larger_than_data() {
        let data = [1.0, 2.0];
        let result = moving_average(&data, 5);
        assert!(result.is_empty());
    }

    #[test]
    fn test_moving_average_zero_window() {
        let data = [1.0, 2.0, 3.0];
        let result = moving_average(&data, 0);
        assert!(result.is_empty());
    }

    // --- pairwise_diff ---

    #[test]
    fn test_pairwise_diff_basic() {
        assert_eq!(pairwise_diff(&[1, 3, 6, 10]), vec![2, 3, 4]);
    }

    #[test]
    fn test_pairwise_diff_single() {
        assert!(pairwise_diff(&[42]).is_empty());
    }

    #[test]
    fn test_pairwise_diff_empty() {
        assert!(pairwise_diff(&[]).is_empty());
    }

    // --- local_maxima ---

    #[test]
    fn test_local_maxima_basic() {
        assert_eq!(local_maxima(&[1, 3, 2, 5, 4]), vec![3, 5]);
    }

    #[test]
    fn test_local_maxima_none() {
        assert!(local_maxima(&[1, 2, 3, 4]).is_empty());
    }

    // --- chunk_sums ---

    #[test]
    fn test_chunk_sums_even_split() {
        assert_eq!(chunk_sums(&[1, 2, 3, 4, 5, 6], 2), vec![3, 7, 11]);
    }

    #[test]
    fn test_chunk_sums_with_remainder() {
        assert_eq!(chunk_sums(&[1, 2, 3, 4, 5], 3), vec![6, 9]);
    }

    #[test]
    fn test_chunk_sums_empty() {
        assert!(chunk_sums(&[], 3).is_empty());
    }

    // --- chunk_exact_sums ---

    #[test]
    fn test_chunk_exact_sums_remainder() {
        let data = [1, 2, 3, 4, 5];
        let (sums, remainder) = chunk_exact_sums(&data, 2);
        assert_eq!(sums, vec![3, 7]);
        assert_eq!(remainder, &[5]);
    }

    #[test]
    fn test_chunk_exact_sums_no_remainder() {
        let data = [1, 2, 3, 4];
        let (sums, remainder) = chunk_exact_sums(&data, 2);
        assert_eq!(sums, vec![3, 7]);
        assert!(remainder.is_empty());
    }

    // --- recursive variants ---

    #[test]
    fn test_windows_recursive() {
        let result = windows_recursive(&[1, 2, 3, 4, 5], 3);
        assert_eq!(result, vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
    }

    #[test]
    fn test_windows_recursive_too_large() {
        let result = windows_recursive(&[1, 2], 5);
        assert!(result.is_empty());
    }

    #[test]
    fn test_chunks_recursive() {
        let result = chunks_recursive(&[1, 2, 3, 4, 5], 3);
        assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5]]);
    }

    #[test]
    fn test_chunks_recursive_even() {
        let result = chunks_recursive(&[1, 2, 3, 4], 2);
        assert_eq!(result, vec![vec![1, 2], vec![3, 4]]);
    }
}
