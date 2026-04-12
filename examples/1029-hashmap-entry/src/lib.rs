#![allow(dead_code)]
#![allow(clippy::all)]
// 1029: HashMap Entry API
// Rust's Entry API avoids double lookups for insert-or-update patterns

use std::collections::HashMap;

/// or_insert: insert a default value if key is absent
fn or_insert_demo() {
    let mut m = HashMap::new();
    m.insert("a", 1);

    // Insert "b" with default 42 if not present
    m.entry("b").or_insert(42);
    // "a" already exists — or_insert does nothing
    m.entry("a").or_insert(99);

    assert_eq!(m["a"], 1);
    assert_eq!(m["b"], 42);
}

/// or_insert_with: compute default lazily
fn or_insert_with_demo() {
    let mut m = HashMap::new();

    let keys = vec!["x", "y"];
    for key in keys {
        m.entry(key).or_insert_with(|| {
            // Expensive computation only runs if key absent
            match key {
                "x" => 100,
                "y" => 200,
                _ => 0,
            }
        });
    }

    assert_eq!(m["x"], 100);
    assert_eq!(m["y"], 200);
}

/// and_modify + or_insert: modify existing OR insert default
fn and_modify_demo() {
    let mut m: HashMap<&str, i32> = HashMap::new();

    // First insert: key absent → or_insert(1)
    m.entry("count").and_modify(|c| *c += 1).or_insert(1);
    assert_eq!(m["count"], 1);

    // Second: key exists → and_modify runs
    m.entry("count").and_modify(|c| *c += 1).or_insert(1);
    assert_eq!(m["count"], 2);

    // Third: still modifying
    m.entry("count").and_modify(|c| *c += 1).or_insert(1);
    assert_eq!(m["count"], 3);
}

/// or_insert returns a mutable reference for in-place mutation
fn entry_ref_demo() {
    let mut m: HashMap<char, Vec<usize>> = HashMap::new();
    let word = "hello";

    for (i, ch) in word.chars().enumerate() {
        m.entry(ch).or_default().push(i);
    }

    assert_eq!(m[&'l'], vec![2, 3]);
    assert_eq!(m[&'h'], vec![0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or_insert() {
        or_insert_demo();
    }

    #[test]
    fn test_or_insert_with() {
        or_insert_with_demo();
    }

    #[test]
    fn test_and_modify() {
        and_modify_demo();
    }

    #[test]
    fn test_entry_ref() {
        entry_ref_demo();
    }

    #[test]
    fn test_or_default() {
        let mut m: HashMap<&str, Vec<i32>> = HashMap::new();
        m.entry("nums").or_default().push(42);
        assert_eq!(m["nums"], vec![42]);
    }
}
