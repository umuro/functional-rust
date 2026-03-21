// Solution 1: Idiomatic Rust — using .sum(), .product(), .max() from std
pub fn sum_idiomatic(numbers: &[i64]) -> i64 {
    numbers.iter().sum()
}

pub fn product_idiomatic(numbers: &[i64]) -> i64 {
    numbers.iter().product()
}

pub fn max_idiomatic(numbers: &[i64]) -> Option<i64> {
    numbers.iter().copied().max()
}

// Solution 2: Functional/recursive — mirrors OCaml's List.fold_left
// fold_left f acc [a; b; c] = f (f (f acc a) b) c
pub fn fold_left<T, Acc, F>(f: F, init: Acc, list: &[T]) -> Acc
where
    F: Fn(Acc, &T) -> Acc,
{
    list.iter().fold(init, f)
}

pub fn sum_fold(numbers: &[i64]) -> i64 {
    fold_left(|acc, &x| acc + x, 0, numbers)
}

pub fn product_fold(numbers: &[i64]) -> i64 {
    fold_left(|acc, &x| acc * x, 1, numbers)
}

pub fn max_fold(numbers: &[i64]) -> Option<i64> {
    match numbers {
        [] => None,
        [first, rest @ ..] => Some(fold_left(|acc, &x| acc.max(x), *first, rest)),
    }
}

// Solution 3: Recursive fold — explicit recursion as in OCaml
pub fn fold_left_rec<T, Acc, F>(f: &F, init: Acc, list: &[T]) -> Acc
where
    F: Fn(Acc, &T) -> Acc,
{
    match list {
        [] => init,
        [head, tail @ ..] => fold_left_rec(f, f(init, head), tail),
    }
}

pub fn sum_recursive(numbers: &[i64]) -> i64 {
    fold_left_rec(&|acc, &x| acc + x, 0, numbers)
}

pub fn product_recursive(numbers: &[i64]) -> i64 {
    fold_left_rec(&|acc, &x| acc * x, 1, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: &[i64] = &[1, 2, 3, 4, 5];

    // --- sum ---

    #[test]
    fn test_sum_empty() {
        assert_eq!(sum_idiomatic(&[]), 0);
        assert_eq!(sum_fold(&[]), 0);
        assert_eq!(sum_recursive(&[]), 0);
    }

    #[test]
    fn test_sum_single() {
        assert_eq!(sum_idiomatic(&[42]), 42);
        assert_eq!(sum_fold(&[42]), 42);
        assert_eq!(sum_recursive(&[42]), 42);
    }

    #[test]
    fn test_sum_multiple() {
        assert_eq!(sum_idiomatic(NUMBERS), 15);
        assert_eq!(sum_fold(NUMBERS), 15);
        assert_eq!(sum_recursive(NUMBERS), 15);
    }

    #[test]
    fn test_sum_negative() {
        assert_eq!(sum_idiomatic(&[-1, -2, 3]), 0);
        assert_eq!(sum_fold(&[-1, -2, 3]), 0);
        assert_eq!(sum_recursive(&[-1, -2, 3]), 0);
    }

    // --- product ---

    #[test]
    fn test_product_empty() {
        assert_eq!(product_idiomatic(&[]), 1);
        assert_eq!(product_fold(&[]), 1);
        assert_eq!(product_recursive(&[]), 1);
    }

    #[test]
    fn test_product_single() {
        assert_eq!(product_idiomatic(&[7]), 7);
        assert_eq!(product_fold(&[7]), 7);
        assert_eq!(product_recursive(&[7]), 7);
    }

    #[test]
    fn test_product_multiple() {
        assert_eq!(product_idiomatic(NUMBERS), 120);
        assert_eq!(product_fold(NUMBERS), 120);
        assert_eq!(product_recursive(NUMBERS), 120);
    }

    #[test]
    fn test_product_with_zero() {
        assert_eq!(product_idiomatic(&[1, 2, 0, 4]), 0);
        assert_eq!(product_fold(&[1, 2, 0, 4]), 0);
        assert_eq!(product_recursive(&[1, 2, 0, 4]), 0);
    }

    // --- max ---

    #[test]
    fn test_max_empty() {
        assert_eq!(max_idiomatic(&[]), None);
        assert_eq!(max_fold(&[]), None);
    }

    #[test]
    fn test_max_single() {
        assert_eq!(max_idiomatic(&[99]), Some(99));
        assert_eq!(max_fold(&[99]), Some(99));
    }

    #[test]
    fn test_max_multiple() {
        assert_eq!(max_idiomatic(NUMBERS), Some(5));
        assert_eq!(max_fold(NUMBERS), Some(5));
    }

    #[test]
    fn test_max_negative() {
        assert_eq!(max_idiomatic(&[-5, -1, -3]), Some(-1));
        assert_eq!(max_fold(&[-5, -1, -3]), Some(-1));
    }

    // --- generic fold_left ---

    #[test]
    fn test_fold_left_string_concat() {
        let words = ["hello", "world"];
        let result = fold_left(
            |acc, &w| format!("{acc} {w}"),
            String::from("start:"),
            &words,
        );
        assert_eq!(result, "start: hello world");
    }

    #[test]
    fn test_fold_left_count() {
        let result = fold_left(|acc, _| acc + 1usize, 0, NUMBERS);
        assert_eq!(result, 5);
    }
}
