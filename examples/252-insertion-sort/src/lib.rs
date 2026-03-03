// Solution 1: Idiomatic Rust — in-place insertion sort using slice swaps
// How a Rust developer writes it: mutate in place, no allocation per step
pub fn insertion_sort_inplace<T: Ord>(data: &mut [T]) {
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j - 1, j);
            j -= 1;
        }
    }
}

// Solution 2: Functional — mirrors the OCaml fold structure exactly
// `List.fold_left (fun acc x -> insert x acc) [] lst`
// Uses partition_point for O(log n) search within the O(n) insert
pub fn insertion_sort_functional<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    list.iter().cloned().fold(Vec::new(), |mut acc, x| {
        // Insert x before the first element strictly greater than x.
        // OCaml: `if x <= h then x :: l` — x goes before h when x <= h,
        // meaning x goes after all elements < x (stable, left-biased).
        let pos = acc.partition_point(|h| h < &x);
        acc.insert(pos, x);
        acc
    })
}

// Solution 3: Recursive — explicit recursion mirroring OCaml's `insert` function
// `let rec insert x = function | [] -> [x] | h :: t as l -> if x <= h then x :: l else h :: insert x t`
pub fn insert_rec<T: Ord + Clone>(x: T, list: &[T]) -> Vec<T> {
    match list {
        [] => vec![x],
        [h, rest @ ..] => {
            if x <= *h {
                let mut result = vec![x];
                result.extend_from_slice(list);
                result
            } else {
                let mut result = vec![h.clone()];
                result.extend(insert_rec(x, rest));
                result
            }
        }
    }
}

pub fn insertion_sort_recursive<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    list.iter()
        .cloned()
        .fold(Vec::new(), |acc, x| insert_rec(x, &acc))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- insertion_sort_inplace ---

    #[test]
    fn test_inplace_empty() {
        let mut data: Vec<i32> = vec![];
        insertion_sort_inplace(&mut data);
        assert_eq!(data, vec![]);
    }

    #[test]
    fn test_inplace_single() {
        let mut data = vec![42];
        insertion_sort_inplace(&mut data);
        assert_eq!(data, vec![42]);
    }

    #[test]
    fn test_inplace_multiple() {
        let mut data = vec![5, 3, 1, 4, 2];
        insertion_sort_inplace(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_inplace_already_sorted() {
        let mut data = vec![1, 2, 3, 4, 5];
        insertion_sort_inplace(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_inplace_reversed() {
        let mut data = vec![5, 4, 3, 2, 1];
        insertion_sort_inplace(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_inplace_duplicates() {
        let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
        insertion_sort_inplace(&mut data);
        assert_eq!(data, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    // --- insertion_sort_functional ---

    #[test]
    fn test_functional_empty() {
        assert_eq!(insertion_sort_functional::<i32>(&[]), vec![]);
    }

    #[test]
    fn test_functional_single() {
        assert_eq!(insertion_sort_functional(&[7]), vec![7]);
    }

    #[test]
    fn test_functional_multiple() {
        assert_eq!(
            insertion_sort_functional(&[5, 3, 1, 4, 2]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_functional_duplicates() {
        assert_eq!(
            insertion_sort_functional(&[3, 1, 4, 1, 5]),
            vec![1, 1, 3, 4, 5]
        );
    }

    // --- insertion_sort_recursive ---

    #[test]
    fn test_recursive_empty() {
        assert_eq!(insertion_sort_recursive::<i32>(&[]), vec![]);
    }

    #[test]
    fn test_recursive_single() {
        assert_eq!(insertion_sort_recursive(&[99]), vec![99]);
    }

    #[test]
    fn test_recursive_multiple() {
        assert_eq!(
            insertion_sort_recursive(&[5, 3, 1, 4, 2]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_recursive_strings() {
        assert_eq!(
            insertion_sort_recursive(&["banana", "apple", "cherry"]),
            vec!["apple", "banana", "cherry"]
        );
    }
}
