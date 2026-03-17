#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let result: Vec<i32> = map_transform(&[], |x| x * 2);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_double_elements() {
        assert_eq!(map_transform(&[1, 2, 3, 4, 5], |x| x * 2), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(map_transform(&[7], |x| x + 1), vec![8]);
    }

    #[test]
    fn test_string_transform() {
        let words = vec!["hello", "world"];
        assert_eq!(map_transform(&words, |s| s.len()), vec![5, 5]);
    }

    #[test]
    fn test_recursive_empty() {
        let result: Vec<i32> = map_recursive(&[], &|x| x * 2);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_recursive_double() {
        assert_eq!(map_recursive(&[1, 2, 3, 4, 5], &|x| x * 2), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_recursive_single() {
        assert_eq!(map_recursive(&[42], &|x| x * 3), vec![126]);
    }

    #[test]
    fn test_recursive_square() {
        assert_eq!(map_recursive(&[1, 2, 3, 4], &|x| x * x), vec![1, 4, 9, 16]);
    }
}
