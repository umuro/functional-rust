// Example 103: Shared References (&T)
//
// Shared borrows let multiple readers access data without transferring ownership.
// Rule: unlimited &T borrows allowed simultaneously, but no &mut T while &T exists.
// This enforces "multiple readers, zero writers" at compile time with zero runtime cost.

// Approach 1: Borrowing instead of moving — &str borrows the String in place
pub fn string_info(s: &str) -> usize {
    let len = s.len();
    let upper = s.to_uppercase();
    // Returns len so caller can use both the original string and the result
    let _ = upper; // used for demonstration; in real code you'd return or log it
    len
}

// Approach 2: Multiple shared readers of a slice — each function borrows independently
pub fn sum_slice(data: &[i32]) -> i32 {
    data.iter().sum()
}

pub fn max_slice(data: &[i32]) -> Option<i32> {
    data.iter().copied().reduce(i32::max)
}

pub fn min_slice(data: &[i32]) -> Option<i32> {
    data.iter().copied().reduce(i32::min)
}

/// Compute sum, max, and min from the same slice using three simultaneous borrows.
/// All three references coexist because none of them mutate `data`.
pub fn stats(data: &[i32]) -> (i32, Option<i32>, Option<i32>) {
    let s = sum_slice(data); // borrow 1
    let mx = max_slice(data); // borrow 2
    let mn = min_slice(data); // borrow 3
    (s, mx, mn)
}

// Approach 3: Shared reference prevents accidental mutation
// Accepting &[T] instead of Vec<T> signals "read only, no ownership transfer"
pub fn contains_duplicate(data: &[i32]) -> bool {
    (1..data.len()).any(|i| data[..i].contains(&data[i]))
}

// Approach 4: Nested shared references — borrowing a reference to a reference
pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_info_borrows_without_moving() {
        let msg = String::from("hello world");
        let len1 = string_info(&msg); // borrow, not move
        let len2 = string_info(&msg); // can borrow again — msg still alive
        assert_eq!(len1, len2);
        assert_eq!(len1, 11);
        // msg is still accessible here — the borrows ended
        assert_eq!(msg.len(), 11);
    }

    #[test]
    fn test_multiple_simultaneous_borrows() {
        let data = [3, 1, 4, 1, 5, 9, 2, 6];
        let (s, mx, mn) = stats(&data);
        assert_eq!(s, 31);
        assert_eq!(mx, Some(9));
        assert_eq!(mn, Some(1));
    }

    #[test]
    fn test_empty_slice() {
        let data: &[i32] = &[];
        assert_eq!(sum_slice(data), 0);
        assert_eq!(max_slice(data), None);
        assert_eq!(min_slice(data), None);
    }

    #[test]
    fn test_contains_duplicate() {
        assert!(contains_duplicate(&[1, 2, 3, 2]));
        assert!(!contains_duplicate(&[1, 2, 3, 4]));
        assert!(!contains_duplicate(&[]));
        assert!(!contains_duplicate(&[42]));
    }

    #[test]
    fn test_longest_shared_lifetime() {
        let s1 = String::from("longer string");
        let result;
        {
            let s2 = String::from("short");
            result = longest(s1.as_str(), s2.as_str());
            // Both borrows valid here — result lifetime tied to the shorter scope
            assert_eq!(result, "longer string");
        }
        // s2 dropped, but result was only used inside the block
        assert_eq!(s1, "longer string");
    }

    #[test]
    fn test_first_char() {
        assert_eq!(first_char("hello"), Some('h'));
        assert_eq!(first_char(""), None);
        assert_eq!(first_char("αβγ"), Some('α'));
    }
}
