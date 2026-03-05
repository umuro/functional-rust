//! 288. Collecting into various collections
//!
//! `collect()` materializes a lazy iterator into any `FromIterator<T>` type.

use std::collections::{HashMap, HashSet, BTreeMap, LinkedList};

fn main() {
    // Collect into Vec
    let squares: Vec<u32> = (0..5).map(|x| x * x).collect();
    println!("Vec: {:?}", squares);

    // Collect into HashSet (deduplicates)
    let words = vec!["apple", "banana", "apple", "cherry", "banana"];
    let set: HashSet<&&str> = words.iter().collect();
    println!("HashSet size: {}", set.len()); // 3 unique

    // Collect into HashMap from iterator of (K, V)
    let keys = ["a", "b", "c"];
    let values = [1u32, 2, 3];
    let map: HashMap<&&str, &u32> = keys.iter().zip(values.iter()).collect();
    println!("HashMap: {:?}", map);

    // Collect into String from chars
    let chars = ['R', 'u', 's', 't'];
    let s: String = chars.iter().collect();
    println!("String from chars: {}", s);

    // Collect into String from words with separator
    let ws = vec!["hello", "world", "rust"];
    let joined: String = ws.join(" ");
    println!("Joined: {}", joined);

    // Collect into BTreeMap (sorted keys)
    let bmap: BTreeMap<i32, i32> = (0..5).map(|x| (x, x * x)).collect();
    println!("BTreeMap: {:?}", bmap);

    // Collect Result<Vec<T>> from Iterator<Result<T>>
    let strs = ["1", "2", "3"];
    let nums: Result<Vec<i32>, _> = strs.iter().map(|s| s.parse::<i32>()).collect();
    println!("Result<Vec>: {:?}", nums);

    // Collect into LinkedList
    let ll: LinkedList<i32> = (1..=4).collect();
    println!("LinkedList: {:?}", ll);
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_collect_vec() {
        let v: Vec<i32> = (1..=5).collect();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_collect_hashset_dedup() {
        let set: HashSet<i32> = vec![1, 2, 2, 3, 3, 3].into_iter().collect();
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_collect_hashmap() {
        let map: HashMap<i32, i32> = (0..3).map(|x| (x, x*x)).collect();
        assert_eq!(map[&2], 4);
    }

    #[test]
    fn test_collect_string() {
        let s: String = ['a', 'b', 'c'].iter().collect();
        assert_eq!(s, "abc");
    }

    #[test]
    fn test_collect_result_vec() {
        let ok: Result<Vec<i32>, _> = ["1", "2", "3"].iter()
            .map(|s| s.parse::<i32>()).collect();
        assert_eq!(ok.unwrap(), vec![1, 2, 3]);
        let err: Result<Vec<i32>, _> = ["1", "x", "3"].iter()
            .map(|s| s.parse::<i32>()).collect();
        assert!(err.is_err());
    }
}
