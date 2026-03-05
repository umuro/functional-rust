/// Look up the value associated with `key` in an association list.
/// Returns `None` if the key is not found (mirrors OCaml's `List.assoc_opt`).
///
/// Association lists are slices of `(K, V)` pairs searched linearly.
/// The first matching key wins — identical to OCaml semantics.
pub fn assoc<'a, K, V>(key: &K, pairs: &'a [(K, V)]) -> Option<&'a V>
where
    K: PartialEq,
{
    pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
}

/// Return `true` if `key` appears in the association list.
/// Mirrors OCaml's `List.mem_assoc`.
pub fn mem_assoc<K, V>(key: &K, pairs: &[(K, V)]) -> bool
where
    K: PartialEq,
{
    pairs.iter().any(|(k, _)| k == key)
}

/// Return a new `Vec` with the **first** occurrence of `key` removed.
/// Mirrors OCaml's `List.remove_assoc`.
pub fn remove_assoc<'a, K, V>(key: &K, pairs: &'a [(K, V)]) -> Vec<&'a (K, V)>
where
    K: PartialEq,
{
    let mut removed = false;
    pairs
        .iter()
        .filter(|(k, _)| {
            if !removed && k == key {
                removed = true;
                false
            } else {
                true
            }
        })
        .collect()
}

// ── Recursive variants (closer to OCaml pattern-matching style) ──────────────

/// Recursive `assoc`: mirrors OCaml's structural recursion over a list.
pub fn assoc_recursive<'a, K, V>(key: &K, pairs: &'a [(K, V)]) -> Option<&'a V>
where
    K: PartialEq,
{
    match pairs {
        [] => None,
        [(k, v), ..] if k == key => Some(v),
        [_, rest @ ..] => assoc_recursive(key, rest),
    }
}

/// Recursive `mem_assoc`: mirrors OCaml's structural recursion.
pub fn mem_assoc_recursive<K, V>(key: &K, pairs: &[(K, V)]) -> bool
where
    K: PartialEq,
{
    match pairs {
        [] => false,
        [(k, _), ..] if k == key => true,
        [_, rest @ ..] => mem_assoc_recursive(key, rest),
    }
}

/// Recursive `remove_assoc`: removes the first match, rebuilds from there.
pub fn remove_assoc_recursive<'a, K, V>(key: &K, pairs: &'a [(K, V)]) -> Vec<&'a (K, V)>
where
    K: PartialEq,
{
    match pairs {
        [] => vec![],
        [(k, _), rest @ ..] if k == key => rest.iter().collect(),
        [head, rest @ ..] => {
            let mut result = vec![head];
            result.extend(remove_assoc_recursive(key, rest));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PHONEBOOK: &[(&str, &str)] = &[
        ("Alice", "555-1234"),
        ("Bob", "555-5678"),
        ("Carol", "555-9012"),
    ];

    // ── assoc ────────────────────────────────────────────────────────────────

    #[test]
    fn test_assoc_found() {
        assert_eq!(assoc(&"Bob", PHONEBOOK), Some(&"555-5678"));
    }

    #[test]
    fn test_assoc_not_found() {
        assert_eq!(assoc(&"Dave", PHONEBOOK), None);
    }

    #[test]
    fn test_assoc_empty() {
        assert_eq!(assoc(&"Alice", &[] as &[(&str, &str)]), None);
    }

    #[test]
    fn test_assoc_first_key_wins() {
        let pairs = &[("a", 1), ("a", 2)];
        assert_eq!(assoc(&"a", pairs), Some(&1));
    }

    // ── mem_assoc ────────────────────────────────────────────────────────────

    #[test]
    fn test_mem_assoc_present() {
        assert!(mem_assoc(&"Alice", PHONEBOOK));
    }

    #[test]
    fn test_mem_assoc_absent() {
        assert!(!mem_assoc(&"Dave", PHONEBOOK));
    }

    #[test]
    fn test_mem_assoc_empty() {
        assert!(!mem_assoc(&"Alice", &[] as &[(&str, &str)]));
    }

    #[test]
    fn test_mem_assoc_all_keys() {
        for (k, _) in PHONEBOOK {
            assert!(mem_assoc(k, PHONEBOOK));
        }
    }

    // ── remove_assoc ─────────────────────────────────────────────────────────

    #[test]
    fn test_remove_assoc_removes_first_match() {
        let result = remove_assoc(&"Bob", PHONEBOOK);
        let keys: Vec<&&str> = result.iter().map(|(k, _)| k).collect();
        assert_eq!(keys, [&"Alice", &"Carol"]);
    }

    #[test]
    fn test_remove_assoc_missing_key_unchanged() {
        let result = remove_assoc(&"Dave", PHONEBOOK);
        assert_eq!(result.len(), PHONEBOOK.len());
    }

    #[test]
    fn test_remove_assoc_empty() {
        let result = remove_assoc(&"Alice", &[] as &[(&str, &str)]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_remove_assoc_only_first_duplicate() {
        let pairs = &[("x", 1), ("x", 2), ("y", 3)];
        let result = remove_assoc(&"x", pairs);
        let keys: Vec<&&str> = result.iter().map(|(k, _)| k).collect();
        assert_eq!(keys, [&"x", &"y"]);
    }

    // ── recursive variants ───────────────────────────────────────────────────

    #[test]
    fn test_assoc_recursive_found() {
        assert_eq!(assoc_recursive(&"Carol", PHONEBOOK), Some(&"555-9012"));
    }

    #[test]
    fn test_assoc_recursive_not_found() {
        assert_eq!(assoc_recursive(&"Dave", PHONEBOOK), None);
    }

    #[test]
    fn test_mem_assoc_recursive_present() {
        assert!(mem_assoc_recursive(&"Bob", PHONEBOOK));
    }

    #[test]
    fn test_mem_assoc_recursive_absent() {
        assert!(!mem_assoc_recursive(&"Zara", PHONEBOOK));
    }

    #[test]
    fn test_remove_assoc_recursive_removes_first() {
        let result = remove_assoc_recursive(&"Alice", PHONEBOOK);
        let keys: Vec<&&str> = result.iter().map(|(k, _)| k).collect();
        assert_eq!(keys, [&"Bob", &"Carol"]);
    }

    #[test]
    fn test_remove_assoc_recursive_missing() {
        let result = remove_assoc_recursive(&"Dave", PHONEBOOK);
        assert_eq!(result.len(), PHONEBOOK.len());
    }
}
