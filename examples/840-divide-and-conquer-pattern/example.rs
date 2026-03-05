/// Divide and Conquer: Generic Recursive Framework.
///
/// Demonstrated with: merge sort, binary search, max subarray.
/// The pattern: split → recurse → combine.

/// Merge sort: O(n log n) time, O(n) space.
fn merge_sort<T: Ord + Clone>(xs: &[T]) -> Vec<T> {
    if xs.len() <= 1 {
        return xs.to_vec();
    }
    let mid = xs.len() / 2;
    let left = merge_sort(&xs[..mid]);
    let right = merge_sort(&xs[mid..]);
    merge(left, right)
}

fn merge<T: Ord>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let (mut i, mut j) = (0, 0);
    let mut result = Vec::with_capacity(a.len() + b.len());
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            result.push(a[i].clone());
            i += 1;
        } else {
            result.push(b[j].clone());
            j += 1;
        }
    }
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}

/// Binary search: O(log n). Returns the index of target.
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let (mut lo, mut hi) = (0usize, arr.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}

/// Maximum crossing subarray sum (for divide step in max subarray).
fn max_crossing(arr: &[i64], lo: usize, mid: usize, hi: usize) -> i64 {
    let mut left_sum = i64::MIN;
    let mut s = 0i64;
    for i in (lo..=mid).rev() {
        s += arr[i];
        if s > left_sum { left_sum = s; }
    }
    let mut right_sum = i64::MIN;
    s = 0;
    for i in mid + 1..=hi {
        s += arr[i];
        if s > right_sum { right_sum = s; }
    }
    left_sum + right_sum
}

/// Maximum subarray sum via D&C: O(n log n).
fn max_subarray(arr: &[i64], lo: usize, hi: usize) -> i64 {
    if lo == hi { return arr[lo]; }
    let mid = lo + (hi - lo) / 2;
    let left_max = max_subarray(arr, lo, mid);
    let right_max = max_subarray(arr, mid + 1, hi);
    let cross_max = max_crossing(arr, lo, mid, hi);
    left_max.max(right_max).max(cross_max)
}

/// Generic D&C framework as a higher-order function.
fn divide_and_conquer<T, R, F, S, C>(
    problem: T,
    base_case: impl Fn(&T) -> Option<R>,
    split: impl Fn(T) -> Vec<T>,
    solve_sub: &F,
    combine: C,
) -> R
where
    F: Fn(T) -> R,
    C: Fn(Vec<R>) -> R,
    T: Clone,
{
    if let Some(result) = base_case(&problem) {
        return result;
    }
    let subproblems = split(problem);
    let sub_results: Vec<R> = subproblems.into_iter().map(|p| solve_sub(p)).collect();
    combine(sub_results)
}

fn main() {
    // Merge sort
    let xs = vec![5i32, 3, 8, 1, 9, 2, 7, 4, 6];
    let sorted = merge_sort(&xs);
    println!("merge_sort({xs:?}) = {sorted:?}");

    // Binary search
    let arr = vec![1i32, 3, 5, 7, 9, 11, 13];
    println!("binary_search(7) = {:?}", binary_search(&arr, &7));
    println!("binary_search(6) = {:?}", binary_search(&arr, &6));

    // Max subarray
    let nums = [-2i64, 1, -3, 4, -1, 2, 1, -5, 4];
    let n = nums.len();
    println!("max_subarray({nums:?}) = {} (expected 6)", max_subarray(&nums, 0, n - 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        assert_eq!(merge_sort(&[5, 3, 8, 1, 9, 2, 7, 4, 6]), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_merge_sort_empty() {
        assert_eq!(merge_sort::<i32>(&[]), vec![]);
    }

    #[test]
    fn test_merge_sort_single() {
        assert_eq!(merge_sort(&[42i32]), vec![42]);
    }

    #[test]
    fn test_merge_sort_strings() {
        assert_eq!(merge_sort(&["banana", "apple", "cherry"]), vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_binary_search_found() {
        let arr = vec![1, 3, 5, 7, 9, 11, 13];
        assert_eq!(binary_search(&arr, &7), Some(3));
        assert_eq!(binary_search(&arr, &1), Some(0));
        assert_eq!(binary_search(&arr, &13), Some(6));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = vec![1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &6), None);
        assert_eq!(binary_search(&arr, &0), None);
        assert_eq!(binary_search(&arr, &10), None);
    }

    #[test]
    fn test_max_subarray() {
        let nums = [-2i64, 1, -3, 4, -1, 2, 1, -5, 4];
        let n = nums.len();
        assert_eq!(max_subarray(&nums, 0, n - 1), 6); // [4,-1,2,1]
    }

    #[test]
    fn test_max_subarray_all_negative() {
        let nums = [-3i64, -1, -2];
        let n = nums.len();
        assert_eq!(max_subarray(&nums, 0, n - 1), -1);
    }
}
