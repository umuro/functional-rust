#![allow(clippy::all)]
// Generate an inclusive range [a, b] (OCaml 99 Problems #22); empty when a > b
pub fn range(a: i32, b: i32) -> Vec<i32> {
    if a > b {
        Vec::new()
    } else {
        (a..=b).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_ascending() {
        assert_eq!(range(4, 9), vec![4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_range_single_element() {
        assert_eq!(range(5, 5), vec![5]);
    }

    #[test]
    fn test_range_empty_when_a_greater_than_b() {
        let empty: Vec<i32> = vec![];
        assert_eq!(range(5, 3), empty);
    }

    #[test]
    fn test_range_negative_to_positive() {
        assert_eq!(range(-2, 2), vec![-2, -1, 0, 1, 2]);
    }
}
