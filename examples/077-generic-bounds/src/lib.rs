#![allow(clippy::all)]
// 077: Generic Bounds
// Constraining generic types with trait bounds

use std::fmt::Display;

// Approach 1: Single bound
fn print_item<T: Display>(item: &T) -> String {
    format!("{}", item)
}

fn print_list<T: Display>(items: &[T]) -> String {
    let strs: Vec<String> = items.iter().map(|x| format!("{}", x)).collect();
    format!("[{}]", strs.join(", "))
}

// Approach 2: Multiple bounds
fn print_and_clone<T: Display + Clone>(item: &T) -> (String, T) {
    (format!("{}", item), item.clone())
}

fn find_max<T: PartialOrd + Clone>(items: &[T]) -> Option<T> {
    items
        .iter()
        .cloned()
        .reduce(|a, b| if a >= b { a } else { b })
}

// Approach 3: Bounds for arithmetic
fn sum_items<T: std::iter::Sum + Copy>(items: &[T]) -> T {
    items.iter().copied().sum()
}

fn contains<T: PartialEq>(items: &[T], target: &T) -> bool {
    items.iter().any(|x| x == target)
}

// Generic pair with bounds
fn larger<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_item() {
        assert_eq!(print_item(&42), "42");
        assert_eq!(print_item(&"hello"), "hello");
    }

    #[test]
    fn test_print_list() {
        assert_eq!(print_list(&[1, 2, 3]), "[1, 2, 3]");
    }

    #[test]
    fn test_print_and_clone() {
        let (s, v) = print_and_clone(&42);
        assert_eq!(s, "42");
        assert_eq!(v, 42);
    }

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[3, 1, 4, 1, 5]), Some(5));
        assert_eq!(find_max::<i32>(&[]), None);
    }

    #[test]
    fn test_contains() {
        assert!(contains(&[1, 2, 3], &2));
        assert!(!contains(&[1, 2, 3], &4));
    }

    #[test]
    fn test_larger() {
        assert_eq!(larger(10, 20), 20);
        assert_eq!(larger("z", "a"), "z");
    }
}
