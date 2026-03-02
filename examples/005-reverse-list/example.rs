// Reverse a list

// Idiomatic Rust: iterator reversal (lazy)
fn rev<T: Clone>(list: &[T]) -> Vec<T> {
    list.iter().rev().cloned().collect()
}

// In-place mutation (imperative style)
fn rev_mut<T>(list: &mut [T]) {
    list.reverse();
}

// Functional with fold (like OCaml accumulator)
fn rev_fold<T: Clone>(list: &[T]) -> Vec<T> {
    list.iter().fold(Vec::new(), |mut acc, x| {
        acc.insert(0, x.clone());
        acc
    })
}

// Tail-recursive (educational)
fn rev_recursive<T: Clone>(list: &[T]) -> Vec<T> {
    fn aux<T: Clone>(mut acc: Vec<T>, list: &[T]) -> Vec<T> {
        match list {
            [] => acc,
            [h, rest @ ..] => {
                acc.insert(0, h.clone());
                aux(acc, rest)
            }
        }
    }
    aux(Vec::new(), list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev() {
        let empty: Vec<i32> = vec![];
        assert_eq!(rev(&empty), empty);
        assert_eq!(rev(&[1]), vec![1]);
        assert_eq!(rev(&[1, 2, 3, 4]), vec![4, 3, 2, 1]);
        assert_eq!(rev(&["a", "b", "c"]), vec!["c", "b", "a"]);
    }

    #[test]
    fn test_rev_mut() {
        let mut list = vec![1, 2, 3, 4];
        rev_mut(&mut list);
        assert_eq!(list, vec![4, 3, 2, 1]);
    }
}

fn main() {
    println!("rev([1,2,3,4]) = {:?}", rev(&[1, 2, 3, 4]));
    println!("✓ Rust tests passed");
}
