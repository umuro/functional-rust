/// Recursive quicksort matching the OCaml functional style.
///
/// This implementation closely mirrors the OCaml version:
/// - Takes a comparator function `gt` (for "greater than")
/// - Recursively partitions the list into smaller and larger elements
/// - Concatenates results using allocation (not in-place)
///
/// **Note:** This is functionally equivalent to OCaml but NOT idiomatic Rust.
/// See `quicksort_idiomatic` for the idiomatic approach.
///
/// The comparator must implement `Copy` to allow recursive calls.
pub fn quicksort<T: Clone + Ord, F: Fn(&T, &T) -> bool + Copy>(gt: F, mut xs: Vec<T>) -> Vec<T> {
    if xs.is_empty() {
        return xs;
    }

    let pivot = xs.remove(0);
    let (ys, zs): (Vec<T>, Vec<T>) = xs.into_iter().partition(|x| gt(&pivot, x));

    let mut left = quicksort(gt, ys);
    let mut result = quicksort(gt, zs);
    left.push(pivot);
    left.append(&mut result);
    left
}

/// Idiomatic Rust quicksort using the standard library's `sort_by`.
///
/// This is the **preferred approach** in production Rust:
/// - Uses proven, optimized std::sort (introsort: quicksort → heapsort)
/// - In-place, single-pass, O(n) space complexity advantage
/// - Leverages Rust's stability and compiler optimizations
///
/// **Trade-off:** Less educational about the algorithm itself, but vastly superior
/// performance and correctness in real-world code.
pub fn quicksort_idiomatic<T: Ord>(xs: Vec<T>) -> Vec<T> {
    let mut result = xs;
    result.sort();
    result
}

/// Idiomatic Rust quicksort with custom comparator.
///
/// Like `quicksort_idiomatic`, but accepts a custom comparison function.
/// Still uses the optimized stdlib implementation under the hood.
pub fn quicksort_idiomatic_by<T, F: Fn(&T, &T) -> std::cmp::Ordering>(
    mut xs: Vec<T>,
    f: F,
) -> Vec<T> {
    xs.sort_by(f);
    xs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let result: Vec<i32> = quicksort(|a, b| a > b, vec![]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_single_element() {
        let result = quicksort(|a, b| a > b, vec![42]);
        assert_eq!(result, vec![42]);
    }

    #[test]
    fn test_multiple_elements() {
        let result = quicksort(|a, b| a > b, vec![4, 65, 2, -31, 0, 99, 83, 782, 1]);
        assert_eq!(result, vec![-31, 0, 1, 2, 4, 65, 83, 99, 782]);
    }

    #[test]
    fn test_already_sorted() {
        let result = quicksort(|a, b| a > b, vec![1, 2, 3, 4, 5]);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let result = quicksort(|a, b| a > b, vec![5, 4, 3, 2, 1]);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        let result = quicksort(|a, b| a > b, vec![3, 1, 3, 2, 1, 3]);
        assert_eq!(result, vec![1, 1, 2, 3, 3, 3]);
    }

    #[test]
    fn test_with_custom_comparator_descending() {
        let result = quicksort(|a, b| a < b, vec![4, 65, 2, -31, 0, 99, 83, 782, 1]);
        assert_eq!(result, vec![782, 99, 83, 65, 4, 2, 1, 0, -31]);
    }

    #[test]
    fn test_idiomatic_multiple_elements() {
        let result = quicksort_idiomatic(vec![4, 65, 2, -31, 0, 99, 83, 782, 1]);
        assert_eq!(result, vec![-31, 0, 1, 2, 4, 65, 83, 99, 782]);
    }

    #[test]
    fn test_idiomatic_by_descending() {
        let result =
            quicksort_idiomatic_by(vec![4, 65, 2, -31, 0, 99, 83, 782, 1], |a, b| b.cmp(a));
        assert_eq!(result, vec![782, 99, 83, 65, 4, 2, 1, 0, -31]);
    }

    #[test]
    fn test_strings_ascending() {
        let result = quicksort_idiomatic(vec!["zebra", "apple", "banana", "cherry"]);
        assert_eq!(result, vec!["apple", "banana", "cherry", "zebra"]);
    }

    #[test]
    fn test_negative_numbers() {
        let result = quicksort(|a, b| a > b, vec![-5, -1, -10, 0, 5, 1]);
        assert_eq!(result, vec![-10, -5, -1, 0, 1, 5]);
    }
}
