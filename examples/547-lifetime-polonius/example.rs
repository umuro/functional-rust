//! # 547. Polonius Borrow Checker Concepts
//! Patterns where current NLL is too conservative vs Polonius.

use std::collections::HashMap;

/// Classic Polonius example: get-or-insert
/// NLL rejects this; Polonius accepts it
/// We show the workaround for stable Rust
fn get_or_insert_nll<'a>(
    map: &'a mut HashMap<String, String>,
    key: String,
) -> &'a str {
    // NLL-friendly version — check then insert
    if !map.contains_key(&key) {
        map.insert(key.clone(), format!("default_{}", key));
    }
    map.get(&key).unwrap()
}

/// The version NLL rejects (but Polonius accepts):
/// fn get_or_insert_polonius<'a>(map: &'a mut HashMap<String, String>, key: String) -> &'a str {
///     match map.get(&key) {  // borrows map as &'a
///         Some(v) => v,      // returns &'a str
///         None => {
///             map.insert(key.clone(), "default".to_string()); // ERROR: map still borrowed!
///             map.get(&key).unwrap()
///         }
///     }
/// }
/// 
/// With Polonius: the borrow from `map.get()` doesn't reach the `None` arm,
/// so inserting there is fine. NLL is too conservative here.

/// Entry API — the idiomatic workaround
fn get_or_insert_entry<'a>(
    map: &'a mut HashMap<String, String>,
    key: String,
) -> &'a str {
    map.entry(key).or_insert_with(|| "default_value".to_string())
}

/// Another Polonius pattern: conditional return with otherwise-mut
fn find_or_last<'a>(v: &'a mut Vec<i32>, target: i32) -> &'a mut i32 {
    // NLL struggles here — workaround: use index
    let pos = v.iter().position(|&x| x == target);
    match pos {
        Some(i) => &mut v[i],
        None    => v.last_mut().unwrap(),
    }
}

/// Two-phase borrows (partial Polonius feature, enabled in NLL)
fn two_phase_example() {
    let mut v = vec![1, 2, 3];
    // Two-phase: &mut v borrow starts "reserved" for the outer call,
    // inner v.len() can still use v as &v
    v.push(v.len() as i32); // works in modern Rust via two-phase borrows!
    println!("two-phase: {:?}", v);
}

/// Flow-sensitive borrow — Polonius understands the branch ends the borrow
fn flow_sensitive(v: &mut Vec<i32>, condition: bool) -> i32 {
    if condition {
        let r = v[0]; // borrow v as &v, read first element
        r             // borrow ends here
    } else {
        v.push(99);   // mutable borrow — valid since condition branch ended
        *v.last().unwrap()
    }
}

fn main() {
    let mut map = HashMap::new();
    let v1 = get_or_insert_nll(&mut map, "hello".to_string());
    println!("v1: {}", v1);
    let v2 = get_or_insert_entry(&mut map, "world".to_string());
    println!("v2: {}", v2);
    println!("map: {:?}", map);

    let mut nums = vec![10, 20, 30, 40, 50];
    let r = find_or_last(&mut nums, 30);
    *r *= 2;
    println!("after find_or_last(30)*2: {:?}", nums);

    let r2 = find_or_last(&mut nums, 99); // not found — use last
    *r2 = 999;
    println!("after find_or_last(99)=999: {:?}", nums);

    two_phase_example();

    println!("flow_sensitive(true): {}", flow_sensitive(&mut vec![1,2,3], true));
    println!("flow_sensitive(false): {}", flow_sensitive(&mut vec![1,2,3], false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_or_insert() {
        let mut m = HashMap::new();
        m.insert("existing".to_string(), "value".to_string());
        let r = get_or_insert_nll(&mut m, "existing".to_string());
        assert_eq!(r, "value");
        let r2 = get_or_insert_nll(&mut m, "new".to_string());
        assert!(r2.starts_with("default_"));
    }

    #[test]
    fn test_find_or_last() {
        let mut v = vec![1, 2, 3, 4, 5];
        let r = find_or_last(&mut v, 3);
        assert_eq!(*r, 3);
    }

    #[test]
    fn test_flow_sensitive() {
        assert_eq!(flow_sensitive(&mut vec![42, 1, 2], true), 42);
    }
}
