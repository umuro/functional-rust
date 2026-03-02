// Find the last element of a list

// Solution 1: Idiomatic Rust (O(1) slice access)
fn last<T>(list: &[T]) -> Option<&T> {
    list.last()
}

// Solution 2: Pattern matching (functional style)
fn last_pattern<T>(list: &[T]) -> Option<&T> {
    match list {
        [] => None,
        [.., last] => Some(last),
    }
}

// Solution 3: Recursive (like OCaml, but not idiomatic)
fn last_recursive<T>(list: &[T]) -> Option<&T> {
    match list {
        [] => None,
        [x] => Some(x),
        [_, rest @ ..] => last_recursive(rest),
    }
}

// Solution 4: Iterator-based (also idiomatic)
fn last_iter<T>(list: &[T]) -> Option<&T> {
    list.iter().last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(last(&empty), None);
    }

    #[test]
    fn test_single() {
        assert_eq!(last(&[1]), Some(&1));
    }

    #[test]
    fn test_multiple() {
        assert_eq!(last(&[1, 2, 3, 4]), Some(&4));
        assert_eq!(last(&["a", "b", "c", "d"]), Some(&"d"));
    }

    #[test]
    fn test_all_implementations() {
        let list = vec![1, 2, 3, 4];
        assert_eq!(last(&list), last_pattern(&list));
        assert_eq!(last(&list), last_recursive(&list));
        assert_eq!(last(&list), last_iter(&list));
    }
}

fn main() {
    println!("last([1,2,3,4]) = {:?}", last(&[1, 2, 3, 4]));
    println!("last([]) = {:?}", last::<i32>(&[]));
    println!("✓ All tests passed");
}

/* Output:
   last([1,2,3,4]) = Some(4)
   last([]) = None
   ✓ All tests passed
*/
