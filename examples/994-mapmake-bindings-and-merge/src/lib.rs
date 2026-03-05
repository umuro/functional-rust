use std::collections::BTreeMap;

// Solution 1: Idiomatic Rust — use BTreeMap (sorted keys, mirrors OCaml Map.Make ordering)
// `bindings` in OCaml returns sorted key-value pairs; BTreeMap iterates in sorted order
pub fn map_bindings(map: &BTreeMap<String, i64>) -> Vec<(&str, i64)> {
    map.iter().map(|(k, v)| (k.as_str(), *v)).collect()
}

// Merge two maps: when a key exists in both, sum the values (mirrors OCaml union callback)
pub fn map_union_sum(
    m1: &BTreeMap<String, i64>,
    m2: &BTreeMap<String, i64>,
) -> BTreeMap<String, i64> {
    let mut result = m1.clone(); // Clone m1; justified: we need an owned map to mutate
    for (k, v) in m2 {
        result
            .entry(k.clone()) // Clone key; justified: entry API requires owned key on insert
            .and_modify(|existing| *existing += v)
            .or_insert(*v);
    }
    result
}

// Solution 2: Functional style — build merged map via iterator fold (no explicit mutation)
pub fn map_union_sum_fold(
    m1: &BTreeMap<String, i64>,
    m2: &BTreeMap<String, i64>,
) -> BTreeMap<String, i64> {
    m2.iter().fold(m1.clone(), |mut acc, (k, v)| {
        acc.entry(k.clone())
            .and_modify(|existing| *existing += v)
            .or_insert(*v);
        acc
    })
}

// Solution 3: Generic merge — accepts any conflict-resolution function
// Mirrors OCaml's `Map.union : (key -> 'a -> 'a -> 'a option) -> 'a t -> 'a t -> 'a t`
pub fn map_union_with<F>(
    m1: &BTreeMap<String, i64>,
    m2: &BTreeMap<String, i64>,
    resolve: F,
) -> BTreeMap<String, i64>
where
    F: Fn(&str, i64, i64) -> Option<i64>,
{
    let mut result: BTreeMap<String, i64> = m1.clone();
    for (k, v2) in m2 {
        match result.get(k) {
            Some(&v1) => match resolve(k, v1, *v2) {
                Some(merged) => {
                    result.insert(k.clone(), merged);
                }
                None => {
                    result.remove(k);
                }
            },
            None => {
                result.insert(k.clone(), *v2);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_m1() -> BTreeMap<String, i64> {
        [("a", 1), ("b", 2), ("c", 3)]
            .iter()
            .map(|&(k, v)| (k.to_string(), v))
            .collect()
    }

    fn make_m2() -> BTreeMap<String, i64> {
        [("b", 20), ("c", 30), ("d", 40)]
            .iter()
            .map(|&(k, v)| (k.to_string(), v))
            .collect()
    }

    #[test]
    fn test_bindings_sorted_order() {
        let map = make_m1();
        let pairs = map_bindings(&map);
        assert_eq!(pairs, vec![("a", 1), ("b", 2), ("c", 3)]);
    }

    #[test]
    fn test_bindings_empty() {
        let map: BTreeMap<String, i64> = BTreeMap::new();
        assert_eq!(map_bindings(&map), vec![]);
    }

    #[test]
    fn test_union_sum_overlap() {
        let merged = map_union_sum(&make_m1(), &make_m2());
        // "a" only in m1 → 1; "b" conflict → 2+20=22; "c" conflict → 3+30=33; "d" only in m2 → 40
        assert_eq!(merged["a"], 1);
        assert_eq!(merged["b"], 22);
        assert_eq!(merged["c"], 33);
        assert_eq!(merged["d"], 40);
    }

    #[test]
    fn test_union_sum_disjoint() {
        let m1: BTreeMap<String, i64> = [("x", 10)]
            .iter()
            .map(|&(k, v)| (k.to_string(), v))
            .collect();
        let m2: BTreeMap<String, i64> = [("y", 20)]
            .iter()
            .map(|&(k, v)| (k.to_string(), v))
            .collect();
        let merged = map_union_sum(&m1, &m2);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged["x"], 10);
        assert_eq!(merged["y"], 20);
    }

    #[test]
    fn test_union_sum_empty_left() {
        let merged = map_union_sum(&BTreeMap::new(), &make_m2());
        assert_eq!(merged, make_m2());
    }

    #[test]
    fn test_union_sum_empty_right() {
        let merged = map_union_sum(&make_m1(), &BTreeMap::new());
        assert_eq!(merged, make_m1());
    }

    #[test]
    fn test_union_fold_matches_imperative() {
        let m1 = make_m1();
        let m2 = make_m2();
        assert_eq!(map_union_sum(&m1, &m2), map_union_sum_fold(&m1, &m2));
    }

    #[test]
    fn test_union_with_sum_resolution() {
        let merged = map_union_with(&make_m1(), &make_m2(), |_k, v1, v2| Some(v1 + v2));
        assert_eq!(merged["b"], 22);
        assert_eq!(merged["c"], 33);
        assert_eq!(merged["d"], 40);
    }

    #[test]
    fn test_union_with_keep_left() {
        // Resolution: always keep left value (prefer m1)
        let merged = map_union_with(&make_m1(), &make_m2(), |_k, v1, _v2| Some(v1));
        assert_eq!(merged["b"], 2); // m1's value kept
        assert_eq!(merged["c"], 3);
        assert_eq!(merged["d"], 40); // only in m2
    }

    #[test]
    fn test_union_with_remove_conflicts() {
        // Resolution: remove keys that appear in both maps
        let merged = map_union_with(&make_m1(), &make_m2(), |_k, _v1, _v2| None);
        // "b" and "c" conflict → removed; "a" only in m1; "d" only in m2
        assert!(!merged.contains_key("b"));
        assert!(!merged.contains_key("c"));
        assert_eq!(merged["a"], 1);
        assert_eq!(merged["d"], 40);
    }

    #[test]
    fn test_bindings_after_merge_sorted() {
        let merged = map_union_sum(&make_m1(), &make_m2());
        let pairs = map_bindings(&merged);
        // Keys must be in lexicographic order
        let keys: Vec<&str> = pairs.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, vec!["a", "b", "c", "d"]);
    }
}
