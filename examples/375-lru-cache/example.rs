use std::collections::{HashMap, VecDeque};

struct LruCache<K: Clone + Eq + std::hash::Hash, V> {
    map: HashMap<K, V>,
    order: VecDeque<K>, // front = most recent, back = least recent
    capacity: usize,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> LruCache<K, V> {
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self { map: HashMap::new(), order: VecDeque::new(), capacity }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Move to front (most recent)
            self.order.retain(|k| k != key);
            self.order.push_front(key.clone());
            self.map.get(key)
        } else { None }
    }

    fn put(&mut self, key: K, val: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() >= self.capacity {
            // Evict least recently used (back of deque)
            if let Some(lru_key) = self.order.pop_back() {
                self.map.remove(&lru_key);
            }
        }
        self.map.insert(key.clone(), val);
        self.order.push_front(key);
    }

    fn len(&self) -> usize { self.map.len() }
    fn contains(&self, key: &K) -> bool { self.map.contains_key(key) }
}

fn main() {
    let mut cache: LruCache<&str, i32> = LruCache::new(3);
    cache.put("a", 1); cache.put("b", 2); cache.put("c", 3);
    println!("get a: {:?}", cache.get(&"a")); // a is now MRU
    cache.put("d", 4); // b is LRU, gets evicted
    println!("a: {:?}", cache.get(&"a"));
    println!("b: {:?}", cache.get(&"b")); // evicted
    println!("c: {:?}", cache.get(&"c"));
    println!("d: {:?}", cache.get(&"d"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn basic_lru() {
        let mut c: LruCache<i32,i32> = LruCache::new(2);
        c.put(1,1); c.put(2,2);
        assert_eq!(c.get(&1), Some(&1));
        c.put(3,3); // 2 evicted
        assert_eq!(c.get(&2), None);
        assert_eq!(c.get(&1), Some(&1));
        assert_eq!(c.get(&3), Some(&3));
    }
    #[test] fn update_existing() {
        let mut c: LruCache<&str,i32> = LruCache::new(2);
        c.put("a",1); c.put("b",2);
        c.put("a",10); // update, a becomes MRU
        c.put("c",3); // b evicted
        assert_eq!(c.get(&"a"), Some(&10));
        assert_eq!(c.get(&"b"), None);
    }
}
