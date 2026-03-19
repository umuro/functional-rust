#![allow(clippy::all)]
/// Merge Sort — Functional Divide and Conquer
///
/// Pure functional merge sort: split the list, sort each half, merge.
/// OCaml uses pattern matching on lists; Rust uses slices and Vec.

/// Merge two sorted slices into a new sorted Vec.
pub fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

/// Functional merge sort — recursive, creates new Vecs at each level.
pub fn merge_sort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    if list.len() <= 1 {
        return list.to_vec();
    }
    let mid = list.len() / 2;
    let left = merge_sort(&list[..mid]);
    let right = merge_sort(&list[mid..]);
    merge(&left, &right)
}

/// Merge using iterators — more functional style.
pub fn merge_iter<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut li = left.iter().peekable();
    let mut ri = right.iter().peekable();
    loop {
        match (li.peek(), ri.peek()) {
            (Some(l), Some(r)) => {
                if l <= r {
                    result.push((*li.next().unwrap()).clone());
                } else {
                    result.push((*ri.next().unwrap()).clone());
                }
            }
            (Some(_), None) => {
                result.extend(li.cloned());
                break;
            }
            (None, Some(_)) => {
                result.extend(ri.cloned());
                break;
            }
            (None, None) => break,
        }
    }
    result
}

/// Custom comparator version (like OCaml's `cmp` parameter).
pub fn merge_sort_by<T: Clone>(list: &[T], cmp: &impl Fn(&T, &T) -> std::cmp::Ordering) -> Vec<T> {
    if list.len() <= 1 {
        return list.to_vec();
    }
    let mid = list.len() / 2;
    let left = merge_sort_by(&list[..mid], cmp);
    let right = merge_sort_by(&list[mid..], cmp);
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if cmp(&left[i], &right[j]) != std::cmp::Ordering::Greater {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(merge_sort(&[5, 2, 8, 1, 9, 3]), vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(merge_sort::<i32>(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_single() {
        assert_eq!(merge_sort(&[42]), vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        assert_eq!(merge_sort(&[1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        assert_eq!(merge_sort(&[5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(merge_sort(&[3, 1, 2, 1, 3]), vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_strings() {
        assert_eq!(
            merge_sort(&["banana", "apple", "cherry"]),
            vec!["apple", "banana", "cherry"]
        );
    }

    #[test]
    fn test_custom_cmp() {
        // Sort descending
        let result = merge_sort_by(&[1, 5, 3, 2, 4], &|a: &i32, b: &i32| b.cmp(a));
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }
}
