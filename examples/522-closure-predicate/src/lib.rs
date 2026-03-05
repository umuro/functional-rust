//! Predicate Functions Pattern
//!
//! Composable predicates: and, or, not, all_of, any_of.

/// Combine two predicates with AND.
pub fn pred_and<T, P1, P2>(p1: P1, p2: P2) -> impl Fn(&T) -> bool
where
    P1: Fn(&T) -> bool,
    P2: Fn(&T) -> bool,
{
    move |x| p1(x) && p2(x)
}

/// Combine two predicates with OR.
pub fn pred_or<T, P1, P2>(p1: P1, p2: P2) -> impl Fn(&T) -> bool
where
    P1: Fn(&T) -> bool,
    P2: Fn(&T) -> bool,
{
    move |x| p1(x) || p2(x)
}

/// Negate a predicate.
pub fn pred_not<T, P>(p: P) -> impl Fn(&T) -> bool
where
    P: Fn(&T) -> bool,
{
    move |x| !p(x)
}

/// All predicates must be true.
pub fn all_of<T>(preds: Vec<Box<dyn Fn(&T) -> bool>>) -> impl Fn(&T) -> bool {
    move |x| preds.iter().all(|p| p(x))
}

/// Any predicate must be true.
pub fn any_of<T>(preds: Vec<Box<dyn Fn(&T) -> bool>>) -> impl Fn(&T) -> bool {
    move |x| preds.iter().any(|p| p(x))
}

/// Common numeric predicates.
pub fn is_positive() -> impl Fn(&i32) -> bool {
    |&x| x > 0
}

pub fn is_even() -> impl Fn(&i32) -> bool {
    |&x| x % 2 == 0
}

pub fn is_in_range(lo: i32, hi: i32) -> impl Fn(&i32) -> bool {
    move |&x| x >= lo && x <= hi
}

/// String predicates for &String references.
pub fn starts_with_str(prefix: String) -> impl Fn(&String) -> bool {
    move |s: &String| s.starts_with(&prefix)
}

pub fn has_length_str(len: usize) -> impl Fn(&String) -> bool {
    move |s: &String| s.len() == len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pred_and() {
        let is_positive_even = pred_and(is_positive(), is_even());
        assert!(is_positive_even(&4));
        assert!(!is_positive_even(&3));
        assert!(!is_positive_even(&-2));
    }

    #[test]
    fn test_pred_or() {
        let zero_or_positive = pred_or(|&x: &i32| x == 0, is_positive());
        assert!(zero_or_positive(&0));
        assert!(zero_or_positive(&5));
        assert!(!zero_or_positive(&-1));
    }

    #[test]
    fn test_pred_not() {
        let is_odd = pred_not(is_even());
        assert!(is_odd(&3));
        assert!(!is_odd(&4));
    }

    #[test]
    fn test_is_in_range() {
        let in_range = is_in_range(1, 10);
        assert!(in_range(&5));
        assert!(in_range(&1));
        assert!(in_range(&10));
        assert!(!in_range(&0));
        assert!(!in_range(&11));
    }

    #[test]
    fn test_all_of() {
        let preds: Vec<Box<dyn Fn(&i32) -> bool>> = vec![
            Box::new(is_positive()),
            Box::new(is_even()),
            Box::new(is_in_range(1, 100)),
        ];
        let check = all_of(preds);
        assert!(check(&4));
        assert!(!check(&101));
        assert!(!check(&-2));
    }

    #[test]
    fn test_any_of() {
        let preds: Vec<Box<dyn Fn(&i32) -> bool>> = vec![
            Box::new(|&x| x == 0),
            Box::new(|&x| x == 42),
            Box::new(|&x| x == 100),
        ];
        let check = any_of(preds);
        assert!(check(&42));
        assert!(!check(&50));
    }

    #[test]
    fn test_string_predicates() {
        let check = pred_and(starts_with_str("hello".into()), has_length_str(11));
        assert!(check(&"hello world".to_string()));
        assert!(!check(&"hello".to_string()));
        assert!(!check(&"hi there!!!".to_string()));
    }

    #[test]
    fn test_filter_with_predicate() {
        let is_valid = pred_and(is_positive(), is_even());
        let nums = vec![-2, -1, 0, 1, 2, 3, 4, 5, 6];
        let result: Vec<i32> = nums.into_iter().filter(|x| is_valid(x)).collect();
        assert_eq!(result, vec![2, 4, 6]);
    }
}
