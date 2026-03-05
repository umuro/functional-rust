// List.flatten — Flatten Nested Lists
// Concatenate a list of lists into a single list

// Solution 1: Idiomatic Rust — flatten() iterator adapter
pub fn flatten_idiomatic<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flatten().cloned().collect()
}

// Solution 2: Functional/recursive — mirrors OCaml List.flatten structure
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

// Solution 3: concat_map — mirrors OCaml List.concat_map
// Applies a function to each element and flattens the results
pub fn concat_map<T, U, F>(items: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> Vec<U>,
{
    items.iter().flat_map(f).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_outer() {
        let empty: &[Vec<i32>] = &[];
        assert_eq!(flatten_idiomatic(empty), Vec::<i32>::new());
        assert_eq!(flatten_recursive(empty), Vec::<i32>::new());
    }

    #[test]
    fn test_single_inner_list() {
        let nested = vec![vec![1, 2, 3]];
        assert_eq!(flatten_idiomatic(&nested), vec![1, 2, 3]);
        assert_eq!(flatten_recursive(&nested), vec![1, 2, 3]);
    }

    #[test]
    fn test_multiple_inner_lists() {
        let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6], vec![7, 8, 9, 10]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(flatten_idiomatic(&nested), expected);
        assert_eq!(flatten_recursive(&nested), expected);
    }

    #[test]
    fn test_inner_empty_lists() {
        let nested = vec![vec![1], vec![], vec![2, 3], vec![]];
        let expected = vec![1, 2, 3];
        assert_eq!(flatten_idiomatic(&nested), expected);
        assert_eq!(flatten_recursive(&nested), expected);
    }

    #[test]
    fn test_all_empty_inner_lists() {
        let nested: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
        assert_eq!(flatten_idiomatic(&nested), Vec::<i32>::new());
        assert_eq!(flatten_recursive(&nested), Vec::<i32>::new());
    }

    #[test]
    fn test_concat_map_duplicate() {
        // Mirrors OCaml: List.concat_map (fun x -> [x; x * 10]) [1; 2; 3]
        let result = concat_map(&[1, 2, 3], |x| vec![*x, x * 10]);
        assert_eq!(result, vec![1, 10, 2, 20, 3, 30]);
    }

    #[test]
    fn test_concat_map_filter_expand() {
        // Use concat_map to both filter and expand
        let result = concat_map(&[1, 2, 3, 4], |x| {
            if x % 2 == 0 {
                vec![*x, *x]
            } else {
                vec![]
            }
        });
        assert_eq!(result, vec![2, 2, 4, 4]);
    }

    #[test]
    fn test_strings() {
        let nested = vec![vec!["hello", "world"], vec!["foo", "bar"]];
        assert_eq!(
            flatten_idiomatic(&nested),
            vec!["hello", "world", "foo", "bar"]
        );
    }
}
