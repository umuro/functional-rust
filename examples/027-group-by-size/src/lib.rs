#![allow(clippy::all)]
// Partition a list into disjoint groups of the given sizes (OCaml 99 Problems #27),
// composing combinations() to select each group in turn.
fn combinations<T: Clone + PartialEq>(k: usize, list: &[T]) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    match list.split_first() {
        None => vec![],
        Some((head, tail)) => {
            let mut with_head: Vec<Vec<T>> = combinations(k - 1, tail)
                .into_iter()
                .map(|mut c| {
                    c.insert(0, head.clone());
                    c
                })
                .collect();
            with_head.extend(combinations(k, tail));
            with_head
        }
    }
}

pub fn group<T: Clone + PartialEq>(list: &[T], sizes: &[usize]) -> Vec<Vec<Vec<T>>> {
    match sizes.split_first() {
        None => vec![vec![]],
        Some((&k, rest)) => {
            let mut result = Vec::new();
            for combo in combinations(k, list) {
                let remaining: Vec<T> = list.iter().filter(|x| !combo.contains(x)).cloned().collect();
                for sub in group(&remaining, rest) {
                    let mut g = vec![combo.clone()];
                    g.extend(sub);
                    result.push(g);
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_group_counts_match_multinomial_coefficient() {
        // C(3,1) * C(2,2) = 3 * 1 = 3
        assert_eq!(group(&[1, 2, 3], &[1, 2]).len(), 3);
    }

    #[test]
    fn test_each_grouping_partitions_the_original_set() {
        let groupings = group(&[1, 2, 3, 4], &[2, 2]);
        for grouping in &groupings {
            let flattened: HashSet<_> = grouping.iter().flatten().collect();
            assert_eq!(flattened, [1, 2, 3, 4].iter().collect());
            assert_eq!(grouping.len(), 2);
            for g in grouping {
                assert_eq!(g.len(), 2);
            }
        }
    }

    #[test]
    fn test_larger_grouping_count() {
        // C(9,2) * C(7,3) * C(4,4) = 36 * 35 * 1 = 1260
        let list: Vec<i32> = (1..=9).collect();
        assert_eq!(group(&list, &[2, 3, 4]).len(), 1260);
    }

    #[test]
    fn test_empty_sizes_yields_one_empty_grouping() {
        assert_eq!(group(&[1, 2, 3], &[]), vec![Vec::<Vec<i32>>::new()]);
    }
}
