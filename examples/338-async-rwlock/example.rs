use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

struct SharedDb { data: RwLock<HashMap<String,i32>> }

impl SharedDb {
    fn new() -> Arc<Self> { Arc::new(Self{data:RwLock::new(HashMap::new())}) }
    fn read(&self, k: &str) -> Option<i32> { self.data.read().unwrap().get(k).copied() }
    fn write(&self, k: &str, v: i32) { self.data.write().unwrap().insert(k.to_string(),v); }
    fn update(&self, k: &str, f: impl Fn(i32)->i32) {
        if let Some(v) = self.data.write().unwrap().get_mut(k) { *v = f(*v); }
    }
}

fn main() {
    let db = SharedDb::new();
    db.write("x", 10); db.write("y", 20);
    println!("x={:?}, y={:?}", db.read("x"), db.read("y"));
    db.update("x", |v| v*2);
    println!("x after update: {:?}", db.read("x"));
    // Concurrent reads
    let handles: Vec<_> = (0..5).map(|_| { let db = Arc::clone(&db); thread::spawn(move || db.read("x")) }).collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("All concurrent reads: {results:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn read_write() { let db = SharedDb::new(); db.write("k",99); assert_eq!(db.read("k"),Some(99)); }
    #[test] fn missing_none() { let db = SharedDb::new(); assert_eq!(db.read("nope"),None); }
    #[test] fn concurrent_reads_ok() {
        let db = SharedDb::new(); db.write("k",7);
        let hs: Vec<_> = (0..10).map(|_| { let db = Arc::clone(&db); thread::spawn(move||db.read("k")) }).collect();
        assert!(hs.into_iter().all(|h| h.join().unwrap()==Some(7)));
    }
}
