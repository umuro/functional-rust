/// Association List — Functional Key-Value Store
///
/// The simplest possible map: a list of (key, value) pairs.
/// Insert prepends (O(1)), lookup scans (O(n)).
/// New entries shadow old ones, just like OCaml's association lists.

/// Insert a key-value pair at the front (shadows any existing key).
pub fn insert<K, V>(k: K, v: V, list: Vec<(K, V)>) -> Vec<(K, V)> {
    let mut new_list = vec![(k, v)];
    new_list.extend(list);
    new_list
}

/// Lookup the first matching key. Returns a reference to the value.
pub fn lookup<'a, K: PartialEq, V>(k: &K, list: &'a [(K, V)]) -> Option<&'a V> {
    list.iter().find(|(key, _)| key == k).map(|(_, v)| v)
}

/// Remove the first occurrence of a key.
pub fn remove<K: PartialEq, V>(k: &K, list: Vec<(K, V)>) -> Vec<(K, V)> {
    let mut found = false;
    list.into_iter()
        .filter(|(key, _)| {
            if !found && key == k {
                found = true;
                false
            } else {
                true
            }
        })
        .collect()
}

/// Get all keys (may contain duplicates due to shadowing).
pub fn keys<K: Clone, V>(list: &[(K, V)]) -> Vec<K> {
    list.iter().map(|(k, _)| k.clone()).collect()
}

/// Iterator-based lookup using `find` — idiomatic Rust.
pub fn lookup_iter<K: PartialEq, V>(k: &K, list: &[(K, V)]) -> Option<&V> {
    list.iter()
        .find_map(|(key, val)| if key == k { Some(val) } else { None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_lookup() {
        let d = vec![];
        let d = insert("a", 1, d);
        let d = insert("b", 2, d);
        assert_eq!(lookup(&"a", &d), Some(&1));
        assert_eq!(lookup(&"b", &d), Some(&2));
        assert_eq!(lookup(&"c", &d), None);
    }

    #[test]
    fn test_shadowing() {
        let d = vec![];
        let d = insert("a", 1, d);
        let d = insert("a", 99, d);
        // Latest value shadows
        assert_eq!(lookup(&"a", &d), Some(&99));
    }

    #[test]
    fn test_remove() {
        let d = vec![];
        let d = insert("a", 1, d);
        let d = insert("b", 2, d);
        let d = insert("a", 99, d);
        let d = remove(&"a", d);
        // After removing first "a" (99), the shadowed "a" (1) is visible
        assert_eq!(lookup(&"a", &d), Some(&1));
    }

    #[test]
    fn test_keys() {
        let d = insert("a", 1, insert("b", 2, vec![]));
        assert_eq!(keys(&d), vec!["a", "b"]);
    }

    #[test]
    fn test_empty() {
        let d: Vec<(&str, i32)> = vec![];
        assert_eq!(lookup(&"x", &d), None);
        assert!(keys(&d).is_empty());
    }

    #[test]
    fn test_lookup_iter() {
        let d = insert("x", 42, vec![]);
        assert_eq!(lookup_iter(&"x", &d), Some(&42));
        assert_eq!(lookup_iter(&"y", &d), None);
    }
}

fn main() {
    println!("{:?}", lookup(&"a", &d), Some(&1));
    println!("{:?}", lookup(&"b", &d), Some(&2));
    println!("{:?}", lookup(&"c", &d), None);
}
