// Idiomatic Rust: use the iterator-based map directly from std
// This is how Rust developers write it — leveraging the standard library
pub fn map_idiomatic<T, U, F>(f: F, items: &[T]) -> Vec<U>
where
    F: Fn(T) -> U,
    T: Copy,
{
    items.iter().map(|&x| f(x)).collect()
}

// Functional/recursive: explicit recursion similar to OCaml
// Shows the abstraction principle: we extract the common pattern (apply f to each element)
pub fn map_recursive<T, U, F>(f: F, items: &[T]) -> Vec<U>
where
    F: Fn(T) -> U,
    T: Copy,
{
    match items {
        [] => Vec::new(),
        [head, rest @ ..] => {
            let mut result = vec![f(*head)];
            result.extend(map_recursive(f, rest));
            result
        }
    }
}

// Generic map over slices — the fundamental abstraction
// This demonstrates partial application: we can bind this to create specialized functions
pub fn map<T, U, F>(f: F, items: &[T]) -> Vec<U>
where
    F: Fn(T) -> U,
    T: Copy,
{
    map_idiomatic(f, items)
}

// Partial application examples — creating specialized transformers by binding map with specific functions
pub fn add_one(items: &[i32]) -> Vec<i32> {
    map(|x| x + 1, items)
}

pub fn to_string_int(items: &[i32]) -> Vec<String> {
    map(|x| x.to_string(), items)
}

pub fn double(items: &[i32]) -> Vec<i32> {
    map(|x| x * 2, items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_empty() {
        let empty: &[i32] = &[];
        let result = map(|x| x + 1, empty);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_map_single() {
        let result = map(|x| x + 1, &[5]);
        assert_eq!(result, vec![6]);
    }

    #[test]
    fn test_map_multiple() {
        let result = map(|x| x + 1, &[1, 2, 3, 4, 5]);
        assert_eq!(result, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_add_one() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = add_one(&nums);
        assert_eq!(result, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_to_string_int() {
        let nums = vec![1, 2, 3];
        let result = to_string_int(&nums);
        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_double() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = double(&nums);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_map_recursive() {
        let result = map_recursive(|x| x * 2, &[1, 2, 3]);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_recursive_vs_idiomatic() {
        let nums = &[1, 2, 3, 4, 5];
        let idiomatic = map_idiomatic(|x| x + 1, nums);
        let recursive = map_recursive(|x| x + 1, nums);
        assert_eq!(idiomatic, recursive);
    }
}
