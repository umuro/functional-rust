//! # Kadane's Algorithm

pub fn max_subarray(arr: &[i32]) -> i32 {
    if arr.is_empty() { return 0; }
    let (mut max_so_far, mut max_ending) = (arr[0], arr[0]);
    for &x in &arr[1..] {
        max_ending = x.max(max_ending + x);
        max_so_far = max_so_far.max(max_ending);
    }
    max_so_far
}

pub fn max_subarray_indices(arr: &[i32]) -> (i32, usize, usize) {
    if arr.is_empty() { return (0, 0, 0); }
    let (mut max_so_far, mut max_ending) = (arr[0], arr[0]);
    let (mut start, mut end, mut s) = (0, 0, 0);
    for i in 1..arr.len() {
        if arr[i] > max_ending + arr[i] { max_ending = arr[i]; s = i; }
        else { max_ending += arr[i]; }
        if max_ending > max_so_far { max_so_far = max_ending; start = s; end = i; }
    }
    (max_so_far, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_kadane() { assert_eq!(max_subarray(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6); }
}
