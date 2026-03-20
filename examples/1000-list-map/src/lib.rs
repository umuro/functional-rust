#![allow(clippy::all)]
//! List mapping examples: idiomatic Rust iterators and functional recursion.
//!
//! This module demonstrates two approaches to applying a function to each element
//! of a list:
//! - **Idiomatic Rust**: Using iterators with `.iter().map().collect()`
//! - **Functional/Recursive**: Tail-recursive style, similar to OCaml's List.map

/// Apply a function to each element using iterators (idiomatic Rust).
///
/// This is the preferred approach in Rust. It uses iterator adapters which are
/// lazy evaluated, composable, and optimized by the compiler.
///
/// # Arguments
/// * `xs` - A slice of elements
/// * `f` - A function to apply to each element
///
/// # Example
/// ```
/// use example_1000_list_map::map_iter;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let doubled = map_iter(&numbers, |x| x * 2);
/// assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
/// ```
pub fn map_iter<T, U, F>(xs: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    xs.iter().map(f).collect()
}

/// Apply a function to each element using recursion (functional style).
///
/// This approach mirrors OCaml's List.map more closely, using tail recursion.
/// In Rust, this is less idiomatic but demonstrates functional programming patterns.
///
/// # Arguments
/// * `xs` - A vector of elements (owned, consumed)
/// * `f` - A function to apply to each element
///
/// # Example
/// ```
/// use example_1000_list_map::map_recursive;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let doubled = map_recursive(numbers.clone(), |x| x * 2);
/// assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
/// ```
pub fn map_recursive<T, U, F>(xs: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    fn go<T, U, F>(mut xs: Vec<T>, f: &F, mut acc: Vec<U>) -> Vec<U>
    where
        F: Fn(T) -> U,
    {
        if xs.is_empty() {
            acc
        } else {
            let head = xs.remove(0);
            acc.push(f(head));
            go(xs, f, acc)
        }
    }

    go(xs, &f, Vec::new())
}

/// Alternative recursive implementation using pattern matching on the vector directly.
/// This version is more elegant but requires consuming the vector via conversion.
pub fn map_recursive_match<T, U, F>(mut xs: Vec<T>, f: &F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    if xs.is_empty() {
        Vec::new()
    } else {
        let head = xs.remove(0);
        let mut tail_result = map_recursive_match(xs, f);
        let mut result = vec![f(head)];
        result.append(&mut tail_result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_iter_empty() {
        let empty: Vec<i32> = vec![];
        let result = map_iter(&empty, |x| x * 2);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_map_iter_single() {
        let single = vec![5];
        let result = map_iter(&single, |x| x * 2);
        assert_eq!(result, vec![10]);
    }

    #[test]
    fn test_map_iter_multiple() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = map_iter(&numbers, |x| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_map_iter_negative() {
        let numbers: Vec<i32> = vec![-1, -2, -3];
        let result = map_iter(&numbers, |&x| x.abs());
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_map_iter_type_conversion() {
        let numbers = vec![1, 2, 3];
        let result = map_iter(&numbers, |x| x.to_string());
        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_map_recursive_empty() {
        let empty: Vec<i32> = vec![];
        let result = map_recursive(empty, |x| x * 2);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_map_recursive_single() {
        let single = vec![5];
        let result = map_recursive(single, |x| x * 2);
        assert_eq!(result, vec![10]);
    }

    #[test]
    fn test_map_recursive_multiple() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = map_recursive(numbers, |x| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_map_recursive_negative() {
        let numbers = vec![-1 as i32, -2 as i32, -3 as i32];
        let result = map_recursive(numbers, |x: i32| x.abs());
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_map_recursive_match_empty() {
        let empty: Vec<i32> = vec![];
        let result = map_recursive_match(empty, &|x| x * 2);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_map_recursive_match_multiple() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = map_recursive_match(numbers, &|x| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_map_iter_squares() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = map_iter(&numbers, |x| x * x);
        assert_eq!(result, vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_map_recursive_squares() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = map_recursive(numbers, |x| x * x);
        assert_eq!(result, vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_map_iter_with_captures() {
        let numbers = vec![1, 2, 3];
        let multiplier = 3;
        let result = map_iter(&numbers, |x| x * multiplier);
        assert_eq!(result, vec![3, 6, 9]);
    }

    #[test]
    fn test_map_recursive_with_captures() {
        let numbers = vec![1, 2, 3];
        let multiplier = 3;
        let result = map_recursive(numbers, |x| x * multiplier);
        assert_eq!(result, vec![3, 6, 9]);
    }
}
