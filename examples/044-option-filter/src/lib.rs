#![allow(clippy::all)]
// Option::filter: keep the value only if it satisfies a predicate, else None.
pub fn filter_positive_even(opt: Option<i32>) -> Option<i32> {
    opt.filter(|&x| x > 0).filter(|&x| x % 2 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_even_passes() {
        assert_eq!(filter_positive_even(Some(4)), Some(4));
    }

    #[test]
    fn test_negative_fails() {
        assert_eq!(filter_positive_even(Some(-4)), None);
    }

    #[test]
    fn test_odd_fails() {
        assert_eq!(filter_positive_even(Some(3)), None);
    }

    #[test]
    fn test_none_stays_none() {
        assert_eq!(filter_positive_even(None), None);
    }
}
