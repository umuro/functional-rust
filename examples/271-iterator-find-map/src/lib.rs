//! 271. Transform-and-find with find_map()
//!
//! `find_map(f)` finds the first `Some(...)` result — single pass, lazy.

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_map_parse() {
        let strings = ["foo", "bar", "42", "baz"];
        let result = strings.iter().find_map(|s| s.parse::<i32>().ok());
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_find_map_none() {
        let strings = ["foo", "bar"];
        let result = strings.iter().find_map(|s| s.parse::<i32>().ok());
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_map_first_match() {
        let nums = [1i32, 2, 3, 4, 5];
        let result = nums.iter().find_map(|&x| if x > 3 { Some(x * 10) } else { None });
        assert_eq!(result, Some(40));
    }
}
