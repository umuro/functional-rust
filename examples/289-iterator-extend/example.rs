//! 289. Extending collections with extend()
//!
//! `extend()` appends elements from an iterator to an existing collection in place.

use std::collections::{HashMap, HashSet};

fn main() {
    // Extend a Vec
    let mut base = vec![1i32, 2, 3];
    base.extend([4, 5, 6]);
    println!("Extended Vec: {:?}", base);

    // Extend with multiple iterators
    let mut nums = vec![1i32, 2];
    nums.extend(3..=5);
    nums.extend([6, 7, 8]);
    println!("Multi-extend: {:?}", nums);

    // Extend String with chars
    let mut s = String::from("Hello");
    s.extend(", world!".chars());
    println!("Extended String: {}", s);

    // Extend HashMap
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    map.extend([("b", 2), ("c", 3)]);
    println!("Extended HashMap: {:?}", map);

    // Extend HashSet
    let mut set: HashSet<i32> = [1, 2, 3].iter().copied().collect();
    set.extend([3, 4, 5]); // 3 is duplicate, ignored
    let mut sorted: Vec<i32> = set.into_iter().collect();
    sorted.sort();
    println!("Extended HashSet: {:?}", sorted);

    // Extend with filter
    let mut evens = vec![2i32, 4];
    evens.extend((1..20).filter(|x| x % 2 == 0).take(3));
    println!("Evens extended: {:?}", evens);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_extend_vec_basic() {
        let mut v = vec![1i32, 2, 3];
        v.extend([4, 5, 6]);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_extend_string() {
        let mut s = String::from("hello");
        s.extend(" world".chars());
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_extend_with_range() {
        let mut v: Vec<i32> = vec![];
        v.extend(1..=5);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_extend_hashset_dedup() {
        use std::collections::HashSet;
        let mut s: HashSet<i32> = [1, 2].iter().copied().collect();
        s.extend([2, 3]);
        assert_eq!(s.len(), 3);
    }
}
