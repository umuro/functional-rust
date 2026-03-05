struct Slab<T> {
    entries: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T> Slab<T> {
    fn new() -> Self { Self { entries: Vec::new(), free: Vec::new() } }

    fn with_capacity(cap: usize) -> Self {
        Self { entries: Vec::with_capacity(cap), free: Vec::new() }
    }

    fn insert(&mut self, val: T) -> usize {
        if let Some(key) = self.free.pop() {
            self.entries[key] = Some(val);
            key
        } else {
            let key = self.entries.len();
            self.entries.push(Some(val));
            key
        }
    }

    fn get(&self, key: usize) -> Option<&T> {
        self.entries.get(key)?.as_ref()
    }

    fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        self.entries.get_mut(key)?.as_mut()
    }

    fn remove(&mut self, key: usize) -> Option<T> {
        let slot = self.entries.get_mut(key)?;
        let val = slot.take()?;
        self.free.push(key);
        Some(val)
    }

    fn len(&self) -> usize { self.entries.iter().filter(|e| e.is_some()).count() }
    fn contains(&self, key: usize) -> bool { self.get(key).is_some() }
    fn iter(&self) -> impl Iterator<Item=(usize, &T)> {
        self.entries.iter().enumerate().filter_map(|(i,e)| e.as_ref().map(|v|(i,v)))
    }
}

fn main() {
    let mut slab: Slab<String> = Slab::new();
    let k1 = slab.insert("hello".into());
    let k2 = slab.insert("world".into());
    let k3 = slab.insert("foo".into());
    println!("k1={k1}: {:?}", slab.get(k1));
    println!("k2={k2}: {:?}", slab.get(k2));
    slab.remove(k1);
    let k4 = slab.insert("reused".into());
    println!("k4={k4} (reused slot from k1={k1}): {:?}", slab.get(k4));
    println!("Active: {}", slab.len());
    for (k,v) in slab.iter() { println!("  [{k}] = {v}"); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn insert_get() {
        let mut s: Slab<i32> = Slab::new();
        let k = s.insert(42);
        assert_eq!(s.get(k), Some(&42));
    }
    #[test] fn remove_and_reuse() {
        let mut s: Slab<i32> = Slab::new();
        let k1 = s.insert(1); s.insert(2); s.remove(k1);
        let k3 = s.insert(3);
        assert_eq!(k3, k1); // slot reused
    }
    #[test] fn stable_keys() {
        let mut s: Slab<String> = Slab::new();
        let k = s.insert("stable".into());
        for _ in 0..100 { s.insert("filler".into()); }
        assert_eq!(s.get(k).unwrap(), "stable");
    }
}
