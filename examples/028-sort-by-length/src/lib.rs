#![allow(clippy::all)]
// Sort a list of lists by length (OCaml 99 Problems #28) — direct and frequency variants.
use std::collections::HashMap;

pub fn sort_by_length(lists: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut v = lists.to_vec();
    v.sort_by_key(|l| l.len());
    v
}

pub fn sort_by_length_freq(lists: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut freq: HashMap<usize, usize> = HashMap::new();
    for l in lists {
        *freq.entry(l.len()).or_insert(0) += 1;
    }
    let mut v = lists.to_vec();
    v.sort_by_key(|l| freq[&l.len()]);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_length() {
        let lists = vec![vec![1, 2, 3], vec![1], vec![1, 2]];
        assert_eq!(sort_by_length(&lists), vec![vec![1], vec![1, 2], vec![1, 2, 3]]);
    }

    #[test]
    fn test_sort_by_length_is_stable() {
        let lists = vec![vec![1, 1], vec![2, 2], vec![3]];
        // both length-2 lists keep their relative order
        assert_eq!(sort_by_length(&lists), vec![vec![3], vec![1, 1], vec![2, 2]]);
    }

    #[test]
    fn test_sort_by_length_freq() {
        let lists = vec![
            vec![1, 2, 3],
            vec![1, 2],
            vec![1, 2, 3, 4],
            vec![1, 2],
            vec![1, 2, 3, 4, 5],
            vec![1, 2],
            vec![1],
        ];
        // lengths 3,2,4,2,5,2,1 -> freq(3)=1, freq(2)=3, freq(4)=1, freq(5)=1, freq(1)=1
        assert_eq!(
            sort_by_length_freq(&lists),
            vec![
                vec![1, 2, 3],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4, 5],
                vec![1],
                vec![1, 2],
                vec![1, 2],
                vec![1, 2],
            ]
        );
    }
}
