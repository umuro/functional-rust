/// Partition a slice into two Vecs based on a predicate.
/// Elements for which `pred` returns true go into the first Vec (yes),
/// the rest into the second Vec (no).
///
/// Solution 1: Idiomatic Rust — uses `partition` from Iterator
pub fn partition_idiomatic<T, F>(items: &[T], pred: F) -> (Vec<&T>, Vec<&T>)
where
    F: Fn(&T) -> bool,
{
    // items.iter() yields &T; Iterator::partition's closure receives &&T, so deref once
    items.iter().partition(|x| pred(x))
}

/// Solution 2: Functional fold — mirrors the OCaml accumulator style
pub fn partition_fold<T, F>(items: &[T], pred: F) -> (Vec<&T>, Vec<&T>)
where
    F: Fn(&T) -> bool,
{
    items.iter().fold((vec![], vec![]), |(mut yes, mut no), x| {
        if pred(x) {
            yes.push(x);
        } else {
            no.push(x);
        }
        (yes, no)
    })
}

/// Solution 3: Recursive — explicit recursion matching OCaml style
pub fn partition_recursive<'a, T, F>(items: &'a [T], pred: &F) -> (Vec<&'a T>, Vec<&'a T>)
where
    F: Fn(&T) -> bool,
{
    match items {
        [] => (vec![], vec![]),
        [head, rest @ ..] => {
            let (mut yes, mut no) = partition_recursive(rest, pred);
            if pred(head) {
                yes.insert(0, head);
            } else {
                no.insert(0, head);
            }
            (yes, no)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let empty: &[i32] = &[];
        let (yes, no) = partition_idiomatic(empty, |_| true);
        assert!(yes.is_empty());
        assert!(no.is_empty());
    }

    #[test]
    fn test_single_matches() {
        let (yes, no) = partition_idiomatic(&[3_i32], |x| *x <= 5);
        assert_eq!(yes, vec![&3]);
        assert!(no.is_empty());
    }

    #[test]
    fn test_single_no_match() {
        let (yes, no) = partition_idiomatic(&[7_i32], |x| *x <= 5);
        assert!(yes.is_empty());
        assert_eq!(no, vec![&7]);
    }

    #[test]
    fn test_numbers_split_at_5() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10_i32];
        let (small, big) = partition_idiomatic(&numbers, |x| *x <= 5);
        assert_eq!(small, vec![&1, &2, &3, &4, &5]);
        assert_eq!(big, vec![&6, &7, &8, &9, &10]);
    }

    #[test]
    fn test_all_match() {
        let data = [1, 2, 3_i32];
        let (yes, no) = partition_idiomatic(&data, |x| *x < 10);
        assert_eq!(yes, vec![&1, &2, &3]);
        assert!(no.is_empty());
    }

    #[test]
    fn test_none_match() {
        let data = [10, 20, 30_i32];
        let (yes, no) = partition_idiomatic(&data, |x| *x < 5);
        assert!(yes.is_empty());
        assert_eq!(no, vec![&10, &20, &30]);
    }

    #[test]
    fn test_fold_matches_idiomatic() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10_i32];
        let (s1, b1) = partition_idiomatic(&numbers, |x| *x <= 5);
        let (s2, b2) = partition_fold(&numbers, |x| *x <= 5);
        assert_eq!(s1, s2);
        assert_eq!(b1, b2);
    }

    #[test]
    fn test_recursive_matches_idiomatic() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10_i32];
        let pred = |x: &i32| *x <= 5;
        let (s1, b1) = partition_idiomatic(&numbers, pred);
        let (s2, b2) = partition_recursive(&numbers, &pred);
        assert_eq!(s1, s2);
        assert_eq!(b1, b2);
    }

    #[test]
    fn test_even_odd_partition() {
        let data = [1, 2, 3, 4, 5, 6_i32];
        let (evens, odds) = partition_idiomatic(&data, |x| *x % 2 == 0);
        assert_eq!(evens, vec![&2, &4, &6]);
        assert_eq!(odds, vec![&1, &3, &5]);
    }
}
