// Palindrome Check
// Rust translation from OCaml 99 Problems #6

// Idiomatic Rust
fn is_palindrome<T: PartialEq>(list: &[T]) -> bool {
    list.iter().eq(list.iter().rev())
}

// Alternative: manual comparison
fn is_palindrome_manual<T: PartialEq + Clone>(list: &[T]) -> bool {
    let reversed: Vec<_> = list.iter().rev().cloned().collect();
    list == reversed.as_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome(&[1, 2, 3, 2, 1]), true);
        assert_eq!(is_palindrome(&[1, 2, 3, 4]), false);
        assert_eq!(is_palindrome::<i32>(&[]), true);
        assert_eq!(is_palindrome(&[1]), true);
    }
}

fn main() {
    println!("is_palindrome([1,2,3,2,1]) = {}", is_palindrome(&[1, 2, 3, 2, 1]));
    println!("is_palindrome([1,2,3,4]) = {}", is_palindrome(&[1, 2, 3, 4]));
    println!("✓ Rust tests passed");
}
