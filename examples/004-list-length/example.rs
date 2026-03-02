// Find the length of a list

// Idiomatic Rust (O(1) for slices)
fn length<T>(list: &[T]) -> usize {
    list.len()
}

// Functional style with iterator
fn length_iter<T>(list: &[T]) -> usize {
    list.iter().count()
}

// Recursive (educational, not idiomatic)
fn length_recursive<T>(list: &[T]) -> usize {
    match list {
        [] => 0,
        [_, rest @ ..] => 1 + length_recursive(rest),
    }
}

// Tail-recursive (like OCaml)
fn length_tail<T>(list: &[T]) -> usize {
    fn aux<T>(n: usize, list: &[T]) -> usize {
        match list {
            [] => n,
            [_, rest @ ..] => aux(n + 1, rest),
        }
    }
    aux(0, list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let empty: Vec<i32> = vec![];
        assert_eq!(length(&empty), 0);
        assert_eq!(length(&[1, 2, 3, 4]), 4);
        
        // Large list (would stack overflow with naive recursion)
        let large: Vec<_> = (0..10000).collect();
        assert_eq!(length(&large), 10000);
    }
}

fn main() {
    println!("length([1,2,3,4]) = {}", length(&[1, 2, 3, 4]));
    println!("✓ Rust tests passed");
}
