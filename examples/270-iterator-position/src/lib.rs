//! 270. Finding index with position()
//!
//! `position(pred)` returns `Option<usize>` — index of first element where predicate holds.

#[cfg(test)]
mod tests {
    #[test]
    fn test_position_found() {
        let v = [10i32, 20, 30, 40];
        assert_eq!(v.iter().position(|&x| x == 30), Some(2));
    }

    #[test]
    fn test_position_not_found() {
        let v = [1i32, 2, 3];
        assert_eq!(v.iter().position(|&x| x == 99), None);
    }

    #[test]
    fn test_rposition() {
        let v = [1i32, 2, 3, 2, 1];
        assert_eq!(v.iter().rposition(|&x| x == 2), Some(3));
    }

    #[test]
    fn test_position_first_occurrence() {
        let v = [5i32, 5, 5];
        assert_eq!(v.iter().position(|&x| x == 5), Some(0));
    }
}
