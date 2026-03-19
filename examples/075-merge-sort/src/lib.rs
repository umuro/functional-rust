#![allow(clippy::all)]
// 075: Merge Sort

// Approach 1: Classic recursive merge sort
fn merge(l1: &[i32], l2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(l1.len() + l2.len());
    let (mut i, mut j) = (0, 0);
    while i < l1.len() && j < l2.len() {
        if l1[i] <= l2[j] {
            result.push(l1[i]);
            i += 1;
        } else {
            result.push(l2[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&l1[i..]);
    result.extend_from_slice(&l2[j..]);
    result
}

fn merge_sort(v: &[i32]) -> Vec<i32> {
    if v.len() <= 1 {
        return v.to_vec();
    }
    let mid = v.len() / 2;
    let left = merge_sort(&v[..mid]);
    let right = merge_sort(&v[mid..]);
    merge(&left, &right)
}

// Approach 2: Generic with comparator
fn merge_sort_by<T: Clone, F>(v: &[T], cmp: &F) -> Vec<T>
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    if v.len() <= 1 {
        return v.to_vec();
    }
    let mid = v.len() / 2;
    let left = merge_sort_by(&v[..mid], cmp);
    let right = merge_sort_by(&v[mid..], cmp);
    let mut result = Vec::with_capacity(v.len());
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

// Approach 3: Functional style with iterators
fn merge_sort_fn(v: &[i32]) -> Vec<i32> {
    if v.len() <= 1 {
        return v.to_vec();
    }
    let mid = v.len() / 2;
    merge(&merge_sort_fn(&v[..mid]), &merge_sort_fn(&v[mid..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        assert_eq!(
            merge_sort(&[5, 3, 8, 1, 9, 2, 7]),
            vec![1, 2, 3, 5, 7, 8, 9]
        );
        assert_eq!(merge_sort(&[]), Vec::<i32>::new());
        assert_eq!(merge_sort(&[1]), vec![1]);
        assert_eq!(merge_sort(&[2, 1]), vec![1, 2]);
    }

    #[test]
    fn test_merge_sort_by() {
        let v = vec![5, 3, 8, 1];
        assert_eq!(
            merge_sort_by(&v, &|a: &i32, b: &i32| a.cmp(b)),
            vec![1, 3, 5, 8]
        );
        assert_eq!(
            merge_sort_by(&v, &|a: &i32, b: &i32| b.cmp(a)),
            vec![8, 5, 3, 1]
        );
    }

    #[test]
    fn test_merge_sort_fn() {
        assert_eq!(
            merge_sort_fn(&[5, 3, 8, 1, 9, 2, 7]),
            vec![1, 2, 3, 5, 7, 8, 9]
        );
    }
}
