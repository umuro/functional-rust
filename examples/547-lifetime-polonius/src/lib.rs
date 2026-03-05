//! Polonius Borrow Checker Concepts
//!
//! Patterns where current NLL is conservative; Polonius accepts.

use std::collections::HashMap;

/// Classic Polonius example: get-or-insert.
/// NLL-friendly workaround using contains_key.
pub fn get_or_insert<'a>(
    map: &'a mut HashMap<String, String>,
    key: String,
) -> &'a str {
    if !map.contains_key(&key) {
        map.insert(key.clone(), format!("default_{}", key));
    }
    map.get(&key).unwrap()
}

/// Another workaround: use entry API.
pub fn get_or_insert_entry<'a>(
    map: &'a mut HashMap<String, String>,
    key: String,
) -> &'a str {
    map.entry(key.clone())
        .or_insert_with(|| format!("default_{}", key))
}

/// Pattern that Polonius would accept directly.
/// We work around by returning Option differently.
pub fn find_or_create(items: &mut Vec<String>, target: &str) -> usize {
    for (i, item) in items.iter().enumerate() {
        if item == target {
            return i;
        }
    }
    items.push(target.to_string());
    items.len() - 1
}

/// Conditional return pattern.
pub fn get_cached<'a>(cache: &'a mut HashMap<i32, String>, key: i32) -> &'a str {
    if !cache.contains_key(&key) {
        cache.insert(key, format!("computed_{}", key));
    }
    cache.get(&key).unwrap()
}

/// Split the logic to help the borrow checker.
fn compute_if_missing(cache: &mut HashMap<i32, String>, key: i32) {
    if !cache.contains_key(&key) {
        cache.insert(key, format!("value_{}", key));
    }
}

pub fn get_with_helper<'a>(cache: &'a mut HashMap<i32, String>, key: i32) -> &'a str {
    compute_if_missing(cache, key);
    cache.get(&key).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_or_insert() {
        let mut map = HashMap::new();
        let v1 = get_or_insert(&mut map, "key1".into());
        assert!(v1.contains("default_key1"));
    }

    #[test]
    fn test_get_or_insert_entry() {
        let mut map = HashMap::new();
        let v1 = get_or_insert_entry(&mut map, "key2".into());
        assert!(v1.contains("default_key2"));
    }

    #[test]
    fn test_find_or_create_existing() {
        let mut items = vec!["a".into(), "b".into(), "c".into()];
        let idx = find_or_create(&mut items, "b");
        assert_eq!(idx, 1);
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_find_or_create_new() {
        let mut items = vec!["a".into(), "b".into()];
        let idx = find_or_create(&mut items, "c");
        assert_eq!(idx, 2);
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_get_cached() {
        let mut cache = HashMap::new();
        let v = get_cached(&mut cache, 42);
        assert!(v.contains("42"));
    }

    #[test]
    fn test_get_with_helper() {
        let mut cache = HashMap::new();
        let v = get_with_helper(&mut cache, 7);
        assert!(v.contains("7"));
    }
}
