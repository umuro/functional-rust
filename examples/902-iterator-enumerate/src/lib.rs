//! 258. Index-value pairs with enumerate()
//!
//! `enumerate()` adds a zero-based index to every iterator element,
//! turning `Iterator<Item=T>` into `Iterator<Item=(usize, T)>`.
//! This integrates cleanly with filter, map, find, and other adapters
//! without needing a mutable counter variable.

/// Return only elements at even indices (0, 2, 4, ...).
///
/// Takes &[T] — borrows the slice, no allocation needed for the input.
pub fn even_indexed<T>(items: &[T]) -> Vec<&T> {
    items
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v)
        .collect()
}

/// Number each element as "1. item", "2. item", ... (1-based).
pub fn number_items(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(i, s)| format!("{}. {}", i + 1, s))
        .collect()
}

/// Find the index of the first element satisfying the predicate.
pub fn find_index<T, F>(items: &[T], pred: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    items
        .iter()
        .enumerate()
        .find(|(_, v)| pred(v))
        .map(|(i, _)| i)
}

/// Return (index, value) pairs where the value satisfies the predicate.
pub fn indexed_filter<T, F>(items: &[T], pred: F) -> Vec<(usize, &T)>
where
    F: Fn(&T) -> bool,
{
    items.iter().enumerate().filter(|(_, v)| pred(v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_indexed_empty() {
        let empty: &[i32] = &[];
        assert_eq!(even_indexed(empty), Vec::<&i32>::new());
    }

    #[test]
    fn test_even_indexed_single() {
        assert_eq!(even_indexed(&[42]), vec![&42]);
    }

    #[test]
    fn test_even_indexed_multiple() {
        let fruits = ["apple", "banana", "cherry", "date"];
        assert_eq!(even_indexed(&fruits), vec![&"apple", &"cherry"]);
    }

    #[test]
    fn test_even_indexed_two_elements() {
        let items = [10, 20];
        assert_eq!(even_indexed(&items), vec![&10]);
    }

    #[test]
    fn test_number_items_empty() {
        assert_eq!(number_items(&[]), Vec::<String>::new());
    }

    #[test]
    fn test_number_items_single() {
        assert_eq!(number_items(&["only"]), vec!["1. only"]);
    }

    #[test]
    fn test_number_items_multiple() {
        let items = ["apple", "banana", "cherry"];
        assert_eq!(
            number_items(&items),
            vec!["1. apple", "2. banana", "3. cherry"]
        );
    }

    #[test]
    fn test_find_index_present() {
        let fruits = ["apple", "banana", "cherry"];
        assert_eq!(find_index(&fruits, |f| f.starts_with('c')), Some(2));
    }

    #[test]
    fn test_find_index_absent() {
        let fruits = ["apple", "banana", "cherry"];
        assert_eq!(find_index(&fruits, |f| f.starts_with('z')), None);
    }

    #[test]
    fn test_find_index_first_match() {
        let nums = [3, 7, 2, 8, 4];
        // First even number is at index 2
        assert_eq!(find_index(&nums, |n| n % 2 == 0), Some(2));
    }

    #[test]
    fn test_indexed_filter_multiple_matches() {
        let nums = [1, 2, 3, 4, 5, 6];
        let evens = indexed_filter(&nums, |n| n % 2 == 0);
        assert_eq!(evens, vec![(1, &2), (3, &4), (5, &6)]);
    }

    #[test]
    fn test_indexed_filter_no_matches() {
        let nums = [1, 3, 5];
        assert_eq!(indexed_filter(&nums, |n| n % 2 == 0), vec![]);
    }
}
