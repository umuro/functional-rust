/// Idiomatic Rust: flatten a slice of Vecs using Iterator::flatten
pub fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flatten().cloned().collect()
}

/// Functional/recursive: build the flat list by processing head and tail
pub fn flatten_recursive<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    match nested {
        [] => vec![],
        [head, rest @ ..] => {
            let mut result = head.clone();
            result.extend(flatten_recursive(rest));
            result
        }
    }
}

/// concat_map equivalent: flat_map each element to a list of derived values
pub fn concat_map<T, U, F>(list: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> Vec<U>,
{
    list.iter().flat_map(f).collect()
}

fn main() {
    let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6], vec![7, 8, 9, 10]];
    let flat = flatten(&nested);
    println!("Flat: {}", flat.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));

    let flat_rec = flatten_recursive(&nested);
    println!("Flat (recursive): {:?}", flat_rec);

    let pairs = concat_map(&[1, 2, 3], |x| vec![*x, x * 10]);
    println!("Pairs (concat_map): {:?}", pairs);
}

/* Output:
   Flat: 1 2 3 4 5 6 7 8 9 10
   Flat (recursive): [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
   Pairs (concat_map): [1, 10, 2, 20, 3, 30]
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_empty() {
        let nested: Vec<Vec<i32>> = vec![];
        assert_eq!(flatten(&nested), vec![]);
    }

    #[test]
    fn test_flatten_single_inner() {
        let nested = vec![vec![42]];
        assert_eq!(flatten(&nested), vec![42]);
    }

    #[test]
    fn test_flatten_multiple() {
        let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6], vec![7, 8, 9, 10]];
        assert_eq!(flatten(&nested), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_flatten_with_empty_sublists() {
        let nested = vec![vec![], vec![1, 2], vec![], vec![3]];
        assert_eq!(flatten(&nested), vec![1, 2, 3]);
    }

    #[test]
    fn test_flatten_recursive_empty() {
        let nested: Vec<Vec<i32>> = vec![];
        assert_eq!(flatten_recursive(&nested), vec![]);
    }

    #[test]
    fn test_flatten_recursive_matches_idiomatic() {
        let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6], vec![7, 8, 9, 10]];
        assert_eq!(flatten_recursive(&nested), flatten(&nested));
    }

    #[test]
    fn test_concat_map_empty() {
        let list: Vec<i32> = vec![];
        assert_eq!(concat_map(&list, |x| vec![*x, x * 10]), vec![]);
    }

    #[test]
    fn test_concat_map_pairs() {
        let list = vec![1, 2, 3];
        assert_eq!(
            concat_map(&list, |x| vec![*x, x * 10]),
            vec![1, 10, 2, 20, 3, 30]
        );
    }
}
