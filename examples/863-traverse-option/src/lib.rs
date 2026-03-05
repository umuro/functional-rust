// Example 064: Traverse with Option
// Apply a function returning Option to each element, collecting all or nothing

// Approach 1: Using Iterator::collect (Rust's built-in traverse!)
fn traverse_option<T, U, F: Fn(&T) -> Option<U>>(xs: &[T], f: F) -> Option<Vec<U>> {
    xs.iter().map(f).collect()
    // collect::<Option<Vec<U>>>() is Rust's traverse!
}

// Approach 2: Manual fold implementation
fn traverse_option_fold<T, U, F: Fn(&T) -> Option<U>>(xs: &[T], f: F) -> Option<Vec<U>> {
    xs.iter().try_fold(Vec::new(), |mut acc, x| {
        let y = f(x)?;
        acc.push(y);
        Some(acc)
    })
}

// Approach 3: Sequence — traverse with identity
fn sequence_option<T: Clone>(xs: &[Option<T>]) -> Option<Vec<T>> {
    xs.iter().cloned().collect()
}

fn safe_div10(x: &i32) -> Option<i32> {
    if *x == 0 { None } else { Some(10 / x) }
}

fn parse_int(s: &&str) -> Option<i32> {
    s.parse().ok()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse_all_succeed() {
        assert_eq!(traverse_option(&[2, 5, 1], safe_div10), Some(vec![5, 2, 10]));
    }

    #[test]
    fn test_traverse_one_fails() {
        assert_eq!(traverse_option(&[2, 0, 1], safe_div10), None);
    }

    #[test]
    fn test_traverse_empty() {
        assert_eq!(traverse_option(&[], safe_div10), Some(vec![]));
    }

    #[test]
    fn test_fold_version() {
        assert_eq!(traverse_option_fold(&[2, 5, 1], safe_div10), Some(vec![5, 2, 10]));
        assert_eq!(traverse_option_fold(&[2, 0, 1], safe_div10), None);
    }

    #[test]
    fn test_parse_strings() {
        let strs = ["1", "2", "3"];
        assert_eq!(traverse_option(&strs, parse_int), Some(vec![1, 2, 3]));
        let strs2 = ["1", "bad", "3"];
        assert_eq!(traverse_option(&strs2, parse_int), None);
    }

    #[test]
    fn test_sequence() {
        assert_eq!(sequence_option(&[Some(1), Some(2), Some(3)]), Some(vec![1, 2, 3]));
        assert_eq!(sequence_option(&[Some(1), None, Some(3)]), None);
    }

    #[test]
    fn test_collect_is_traverse() {
        // Rust's collect() on Iterator<Item=Option<T>> IS traverse!
        let result: Option<Vec<i32>> = vec![Some(1), Some(2), Some(3)]
            .into_iter()
            .collect();
        assert_eq!(result, Some(vec![1, 2, 3]));
    }
}
