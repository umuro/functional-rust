// 001: Higher-Order Functions
// map, filter, fold — the three pillars of functional programming

// Approach 1: Using iterator methods
fn double_all(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|&x| x * 2).collect()
}

fn evens(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|&&x| x % 2 == 0).copied().collect()
}

fn sum(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &x| acc + x)
}

// Approach 2: Manual recursive implementations
fn my_map(f: fn(i32) -> i32, slice: &[i32]) -> Vec<i32> {
    if slice.is_empty() {
        vec![]
    } else {
        let mut result = vec![f(slice[0])];
        result.extend(my_map(f, &slice[1..]));
        result
    }
}

fn my_filter(pred: fn(i32) -> bool, slice: &[i32]) -> Vec<i32> {
    if slice.is_empty() {
        vec![]
    } else {
        let mut result = if pred(slice[0]) { vec![slice[0]] } else { vec![] };
        result.extend(my_filter(pred, &slice[1..]));
        result
    }
}

fn my_fold(f: fn(i32, i32) -> i32, acc: i32, slice: &[i32]) -> i32 {
    if slice.is_empty() {
        acc
    } else {
        my_fold(f, f(acc, slice[0]), &slice[1..])
    }
}

// Approach 3: Chained iterators
fn sum_of_doubled_evens(nums: &[i32]) -> i32 {
    nums.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_all() {
        assert_eq!(double_all(&[1, 2, 3]), vec![2, 4, 6]);
    }

    #[test]
    fn test_evens() {
        let nums: Vec<i32> = (1..=10).collect();
        assert_eq!(evens(&nums), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_sum() {
        let nums: Vec<i32> = (1..=10).collect();
        assert_eq!(sum(&nums), 55);
    }

    #[test]
    fn test_my_map() {
        assert_eq!(my_map(|x| x + 1, &[1, 2, 3]), vec![2, 3, 4]);
    }

    #[test]
    fn test_my_filter() {
        assert_eq!(my_filter(|x| x > 3, &[1, 2, 3, 4, 5]), vec![4, 5]);
    }

    #[test]
    fn test_my_fold() {
        assert_eq!(my_fold(|a, b| a + b, 0, &[1, 2, 3]), 6);
    }

    #[test]
    fn test_sum_of_doubled_evens() {
        let nums: Vec<i32> = (1..=10).collect();
        assert_eq!(sum_of_doubled_evens(&nums), 60);
    }
}
