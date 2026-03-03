// Solution 1: Idiomatic Rust — uses slice::sort (in-place, introsort-based)
pub fn quicksort_inplace<T: Ord>(data: &mut [T]) {
    data.sort();
}

// Solution 2: Functional/recursive — mirrors the OCaml structure exactly
pub fn quicksort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    match list {
        [] => vec![],
        [pivot, rest @ ..] => {
            let (left, right): (Vec<T>, Vec<T>) = rest.iter().cloned().partition(|x| x < pivot);
            let mut result = quicksort(&left);
            result.push(pivot.clone());
            result.extend(quicksort(&right));
            result
        }
    }
}

// Solution 3: In-place recursive quicksort — Lomuto partition scheme
pub fn quicksort_recursive<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }
    let pivot_idx = partition(data);
    quicksort_recursive(&mut data[..pivot_idx]);
    quicksort_recursive(&mut data[pivot_idx + 1..]);
}

fn partition<T: Ord>(data: &mut [T]) -> usize {
    let last = data.len() - 1;
    let mut store = 0;
    for i in 0..last {
        if data[i] <= data[last] {
            data.swap(i, store);
            store += 1;
        }
    }
    data.swap(store, last);
    store
}

fn main() {
    let input = [3, 6, 8, 10, 1, 2, 1];

    // Functional style (allocating, like OCaml)
    let sorted = quicksort(&input);
    println!("quicksort({input:?}) = {sorted:?}");

    // In-place recursive (Lomuto)
    let mut data = input.to_vec();
    quicksort_recursive(&mut data);
    println!("quicksort_recursive({input:?}) = {data:?}");

    // Stdlib sort
    let mut data2 = input.to_vec();
    quicksort_inplace(&mut data2);
    println!("quicksort_inplace({input:?}) = {data2:?}");
}

/* Output:
   quicksort([3, 6, 8, 10, 1, 2, 1]) = [1, 1, 2, 3, 6, 8, 10]
   quicksort_recursive([3, 6, 8, 10, 1, 2, 1]) = [1, 1, 2, 3, 6, 8, 10]
   quicksort_inplace([3, 6, 8, 10, 1, 2, 1]) = [1, 1, 2, 3, 6, 8, 10]
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_empty() {
        assert_eq!(quicksort::<i32>(&[]), vec![]);
    }

    #[test]
    fn test_functional_single() {
        assert_eq!(quicksort(&[42]), vec![42]);
    }

    #[test]
    fn test_functional_multiple() {
        assert_eq!(
            quicksort(&[3, 6, 8, 10, 1, 2, 1]),
            vec![1, 1, 2, 3, 6, 8, 10]
        );
    }

    #[test]
    fn test_functional_reversed() {
        assert_eq!(quicksort(&[5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_functional_already_sorted() {
        assert_eq!(quicksort(&[1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_functional_duplicates() {
        assert_eq!(quicksort(&[3, 3, 3]), vec![3, 3, 3]);
    }

    #[test]
    fn test_recursive_empty() {
        let mut data: Vec<i32> = vec![];
        quicksort_recursive(&mut data);
        assert_eq!(data, vec![]);
    }

    #[test]
    fn test_recursive_single() {
        let mut data = vec![7];
        quicksort_recursive(&mut data);
        assert_eq!(data, vec![7]);
    }

    #[test]
    fn test_recursive_multiple() {
        let mut data = vec![3, 6, 8, 10, 1, 2, 1];
        quicksort_recursive(&mut data);
        assert_eq!(data, vec![1, 1, 2, 3, 6, 8, 10]);
    }

    #[test]
    fn test_recursive_reversed() {
        let mut data = vec![5, 4, 3, 2, 1];
        quicksort_recursive(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_inplace_sorts_correctly() {
        let mut data = vec![3, 6, 8, 10, 1, 2, 1];
        quicksort_inplace(&mut data);
        assert_eq!(data, vec![1, 1, 2, 3, 6, 8, 10]);
    }

    #[test]
    fn test_inplace_strings() {
        let mut data = vec!["banana", "apple", "cherry"];
        quicksort_inplace(&mut data);
        assert_eq!(data, vec!["apple", "banana", "cherry"]);
    }
}
