//! impl Trait in Argument Position

pub fn print_all(items: impl Iterator<Item = impl std::fmt::Display>) {
    for item in items {
        println!("{}", item);
    }
}

pub fn sum_all(nums: impl Iterator<Item = i32>) -> i32 {
    nums.sum()
}

pub fn process<F: Fn(i32) -> i32>(items: impl Iterator<Item = i32>, f: F) -> Vec<i32> {
    items.map(f).collect()
}

pub fn debug_any(val: impl std::fmt::Debug) -> String {
    format!("{:?}", val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum_all(vec![1, 2, 3, 4].into_iter()), 10);
    }
    #[test]
    fn test_process() {
        assert_eq!(process(vec![1, 2, 3].into_iter(), |x| x * 2), vec![2, 4, 6]);
    }
    #[test]
    fn test_debug() {
        assert!(debug_any(vec![1, 2]).contains("1"));
    }
    #[test]
    fn test_empty() {
        assert_eq!(sum_all(std::iter::empty()), 0);
    }
}
