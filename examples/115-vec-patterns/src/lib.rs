#![allow(clippy::all)]
/// Approach 1: map, filter, fold via iterator adapters (lazy, zero intermediate allocation)
/// OCaml: List.filter f |> List.map g |> List.fold_left (+) 0
pub fn sum_of_doubled_evens(data: &[i32]) -> i32 {
    data.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 2).sum()
}

/// Approach 2: zip pairs of slices into tuples
/// OCaml: List.map2 (fun x y -> (x, y)) a b
pub fn zip_names_ages<'a>(names: &[&'a str], ages: &[u32]) -> Vec<(&'a str, u32)> {
    names.iter().copied().zip(ages.iter().copied()).collect()
}

type PartitionedPairs<'a> = (Vec<(&'a str, u32)>, Vec<(&'a str, u32)>);

/// Partition pairs by age threshold — OCaml: List.partition (fun (_, age) -> age < threshold)
pub fn partition_by_age<'a>(pairs: &[(&'a str, u32)], threshold: u32) -> PartitionedPairs<'a> {
    pairs.iter().copied().partition(|&(_, age)| age < threshold)
}

/// Approach 3: flat_map (OCaml: List.concat_map / List.flatten ∘ List.map)
/// Expands each element into multiple elements and flattens the result.
pub fn expand_range(data: &[i32]) -> Vec<i32> {
    data.iter().flat_map(|&x| 0..x).collect()
}

/// Approach 4: fold to build a HashMap histogram
/// OCaml: List.fold_left (fun acc x -> ...) empty_map data
pub fn histogram(data: &[i32]) -> std::collections::HashMap<i32, usize> {
    data.iter()
        .fold(std::collections::HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
}

/// Approach 5: scan — running totals (prefix sums)
/// OCaml: no direct equivalent; closest is a custom fold that keeps intermediates
pub fn prefix_sums(data: &[i32]) -> Vec<i32> {
    data.iter()
        .scan(0_i32, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_doubled_evens_basic() {
        let data: Vec<i32> = (1..=10).collect();
        assert_eq!(sum_of_doubled_evens(&data), 60);
    }

    #[test]
    fn test_sum_of_doubled_evens_empty() {
        assert_eq!(sum_of_doubled_evens(&[]), 0);
    }

    #[test]
    fn test_sum_of_doubled_evens_all_odd() {
        assert_eq!(sum_of_doubled_evens(&[1, 3, 5]), 0);
    }

    #[test]
    fn test_zip_names_ages() {
        let names = ["Alice", "Bob", "Charlie"];
        let ages = [30_u32, 25, 35];
        let pairs = zip_names_ages(&names, &ages);
        assert_eq!(pairs, vec![("Alice", 30), ("Bob", 25), ("Charlie", 35)]);
    }

    #[test]
    fn test_partition_by_age() {
        let names = ["Alice", "Bob", "Charlie"];
        let ages = [30_u32, 25, 35];
        let pairs = zip_names_ages(&names, &ages);
        let (young, old) = partition_by_age(&pairs, 30);
        assert_eq!(young.len(), 1);
        assert_eq!(old.len(), 2);
        assert_eq!(young[0].0, "Bob");
    }

    #[test]
    fn test_expand_range() {
        // flat_map: [1,3] → [0] ++ [0,1,2] = [0,0,1,2]
        assert_eq!(expand_range(&[1, 3]), vec![0, 0, 1, 2]);
        assert_eq!(expand_range(&[]), vec![]);
    }

    #[test]
    fn test_histogram() {
        let hist = histogram(&[1, 2, 1, 3, 2, 1]);
        assert_eq!(hist[&1], 3);
        assert_eq!(hist[&2], 2);
        assert_eq!(hist[&3], 1);
    }

    #[test]
    fn test_prefix_sums() {
        assert_eq!(prefix_sums(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(prefix_sums(&[]), vec![]);
    }
}
