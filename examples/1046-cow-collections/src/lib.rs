// 1046: Clone-on-Write: Cow<'_, [T]> for Read-Mostly Data
// Avoid cloning until mutation is actually needed

use std::borrow::Cow;

/// Process data without cloning if no modification needed
fn process_data(data: &[i32], threshold: i32) -> Cow<'_, [i32]> {
    if data.iter().all(|&x| x <= threshold) {
        // No change needed — borrow original data
        Cow::Borrowed(data)
    } else {
        // Need to modify — clone and filter
        Cow::Owned(data.iter().map(|&x| x.min(threshold)).collect())
    }
}

fn cow_borrow_vs_owned() {
    let data = vec![1, 2, 3, 4, 5];

    // Case 1: All values within threshold — no clone
    let result = process_data(&data, 10);
    assert!(matches!(result, Cow::Borrowed(_)));
    assert_eq!(&*result, &[1, 2, 3, 4, 5]);

    // Case 2: Some values exceed threshold — clones and caps
    let result = process_data(&data, 3);
    assert!(matches!(result, Cow::Owned(_)));
    assert_eq!(&*result, &[1, 2, 3, 3, 3]);
}

/// Cow<str> for string processing
fn normalize_name(name: &str) -> Cow<'_, str> {
    if name.contains(char::is_uppercase) {
        // Need to modify — allocate new string
        Cow::Owned(name.to_lowercase())
    } else {
        // Already lowercase — just borrow
        Cow::Borrowed(name)
    }
}

fn cow_str_demo() {
    let name1 = "alice";
    let result1 = normalize_name(name1);
    assert!(matches!(result1, Cow::Borrowed(_)));
    assert_eq!(&*result1, "alice");

    let name2 = "Alice";
    let result2 = normalize_name(name2);
    assert!(matches!(result2, Cow::Owned(_)));
    assert_eq!(&*result2, "alice");
}

/// to_mut() triggers clone only on first mutation
fn to_mut_demo() {
    let data = vec![1, 2, 3, 4, 5];
    let mut cow: Cow<'_, [i32]> = Cow::Borrowed(&data);

    // Reading doesn't clone
    assert_eq!(cow[0], 1);
    assert!(matches!(cow, Cow::Borrowed(_)));

    // Mutation triggers clone
    cow.to_mut()[0] = 99;
    assert!(matches!(cow, Cow::Owned(_)));
    assert_eq!(cow[0], 99);

    // Second mutation doesn't clone again
    cow.to_mut()[1] = 88;
    assert_eq!(&*cow, &[99, 88, 3, 4, 5]);

    // Original unchanged
    assert_eq!(data, vec![1, 2, 3, 4, 5]);
}

/// Cow in function signatures — accept both owned and borrowed
fn print_items(items: Cow<'_, [i32]>) -> usize {
    items.len()
}

fn flexible_api() {
    let owned = vec![1, 2, 3];
    let borrowed = &[4, 5, 6][..];

    assert_eq!(print_items(Cow::Owned(owned)), 3);
    assert_eq!(print_items(Cow::Borrowed(borrowed)), 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cow_borrow() {
        cow_borrow_vs_owned();
    }

    #[test]
    fn test_cow_str() {
        cow_str_demo();
    }

    #[test]
    fn test_to_mut() {
        to_mut_demo();
    }

    #[test]
    fn test_flexible() {
        flexible_api();
    }

    #[test]
    fn test_into_owned() {
        let data = vec![1, 2, 3];
        let cow: Cow<'_, [i32]> = Cow::Borrowed(&data);
        let owned: Vec<i32> = cow.into_owned();
        assert_eq!(owned, vec![1, 2, 3]);
    }
}
