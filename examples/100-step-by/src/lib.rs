#![allow(clippy::all)]
// 100: Step By

#[cfg(test)]
mod tests {
    #[test]
    fn test_step_by() {
        let v: Vec<i32> = (0..10).step_by(2).collect();
        assert_eq!(v, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_step_by_25() {
        let v: Vec<i32> = (0..100).step_by(25).collect();
        assert_eq!(v, vec![0, 25, 50, 75]);
    }

    #[test]
    fn test_step_by_5() {
        let v: Vec<i32> = (0..20).step_by(5).collect();
        assert_eq!(v, vec![0, 5, 10, 15]);
    }

    #[test]
    fn test_step_by_1() {
        let v: Vec<i32> = (0..3).step_by(1).collect();
        assert_eq!(v, vec![0, 1, 2]);
    }
}
