use std::collections::BTreeMap;

// Solution 1: Idiomatic Rust — use BTreeMap (sorted keys, mirrors OCaml Map.Make ordering)
pub fn map_bindings(map: &BTreeMap<String, i64>) -> Vec<(&str, i64)> {
    map.iter().map(|(k, v)| (k.as_str(), *v)).collect()
}

// Merge two maps: when a key exists in both, sum the values
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

// Solution 2: Functional style — build merged map via iterator fold
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

// Solution 3: Generic merge with custom conflict resolution
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

fn main() {
    let m1: BTreeMap<String, i64> = [("a", 1), ("b", 2), ("c", 3)]
        .iter()
        .map(|&(k, v)| (k.to_string(), v))
        .collect();

    let m2: BTreeMap<String, i64> = [("b", 20), ("c", 30), ("d", 40)]
        .iter()
        .map(|&(k, v)| (k.to_string(), v))
        .collect();

    println!("m1 bindings:");
    for (k, v) in map_bindings(&m1) {
        println!("  {k}: {v}");
    }

    let merged = map_union_sum(&m1, &m2);
    println!("\nmerged (sum on conflict) bindings:");
    for (k, v) in map_bindings(&merged) {
        println!("  {k}: {v}");
    }

    let merged_fold = map_union_sum_fold(&m1, &m2);
    println!(
        "\nmerged via fold (same result): {:?}",
        merged_fold == merged
    );

    let merged_keep_left = map_union_with(&m1, &m2, |_k, v1, _v2| Some(v1));
    println!("\nmerged (keep-left on conflict) bindings:");
    for (k, v) in map_bindings(&merged_keep_left) {
        println!("  {k}: {v}");
    }
}

/* Output:
   m1 bindings:
     a: 1
     b: 2
     c: 3

   merged (sum on conflict) bindings:
     a: 1
     b: 22
     c: 33
     d: 40

   merged via fold (same result): true

   merged (keep-left on conflict) bindings:
     a: 1
     b: 2
     c: 3
     d: 40
*/
