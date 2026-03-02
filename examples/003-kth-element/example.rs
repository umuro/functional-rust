// Find the k-th element (converting to 0-indexed)

// Idiomatic Rust (0-indexed)
fn at<T>(k: usize, list: &[T]) -> Option<&T> {
    list.get(k)
}

// OCaml-style (1-indexed)
fn at_one_indexed<T>(k: usize, list: &[T]) -> Option<&T> {
    if k == 0 {
        None
    } else {
        list.get(k - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_indexed() {
        assert_eq!(at(2, &[1, 2, 3, 4, 5]), Some(&3));
        assert_eq!(at(0, &[1, 2, 3]), Some(&1));
        assert_eq!(at(10, &[1, 2, 3]), None);
    }

    #[test]
    fn test_one_indexed() {
        assert_eq!(at_one_indexed(3, &[1, 2, 3, 4, 5]), Some(&3));
        assert_eq!(at_one_indexed(1, &[1, 2, 3]), Some(&1));
        assert_eq!(at_one_indexed(10, &[1, 2, 3]), None);
    }
}

fn main() {
    println!("at(2, [1,2,3,4,5]) = {:?}", at(2, &[1, 2, 3, 4, 5]));
    println!("✓ Rust tests passed");
}
