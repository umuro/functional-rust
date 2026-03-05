// 487. String interning pattern
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Interner {
    table: HashMap<String, Arc<str>>,
}

impl Interner {
    pub fn new() -> Self { Interner { table: HashMap::new() } }
    pub fn intern(&mut self, s: &str) -> Arc<str> {
        if let Some(v) = self.table.get(s) { return Arc::clone(v); }
        let arc: Arc<str> = Arc::from(s);
        self.table.insert(s.to_string(), Arc::clone(&arc));
        arc
    }
    pub fn len(&self) -> usize { self.table.len() }
}

// Thread-safe interner
pub struct SyncInterner(Mutex<Interner>);
impl SyncInterner {
    pub fn new() -> Self { SyncInterner(Mutex::new(Interner::new())) }
    pub fn intern(&self, s: &str) -> Arc<str> { self.0.lock().unwrap().intern(s) }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_intern_same()    { let mut i=Interner::new(); let a=i.intern("hi"); let b=i.intern("hi"); assert!(Arc::ptr_eq(&a,&b)); }
    #[test] fn test_intern_diff()    { let mut i=Interner::new(); let a=i.intern("hi"); let b=i.intern("ho"); assert!(!Arc::ptr_eq(&a,&b)); }
    #[test] fn test_intern_size()    { let mut i=Interner::new(); i.intern("a"); i.intern("b"); i.intern("a"); assert_eq!(i.len(),2); }
}
