#![allow(clippy::all)]
// 1049: Persistent HashMap — Functional Update
// Simulate persistence in Rust using clone-on-write or cheap cloning

use std::collections::HashMap;
use std::rc::Rc;

/// Simple persistent map via full clone (conceptual — not efficient)
/// Real persistent maps use structural sharing (HAMT, etc.)
#[derive(Clone, Debug)]
struct PersistentMap<K, V> {
    data: HashMap<K, V>,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> PersistentMap<K, V> {
    fn new() -> Self {
        PersistentMap {
            data: HashMap::new(),
        }
    }

    /// Insert returns a new version (old version unchanged)
    fn insert(&self, key: K, value: V) -> Self {
        let mut new_data = self.data.clone();
        new_data.insert(key, value);
        PersistentMap { data: new_data }
    }

    /// Remove returns a new version
    fn remove(&self, key: &K) -> Self {
        let mut new_data = self.data.clone();
        new_data.remove(key);
        PersistentMap { data: new_data }
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

fn persistence_demo() {
    let v1 = PersistentMap::new()
        .insert("a", 1)
        .insert("b", 2)
        .insert("c", 3);

    let v2 = v1.insert("d", 4); // v2 has a,b,c,d
    let v3 = v1.insert("b", 99); // v3 updates b in v1

    // All versions coexist
    assert_eq!(v1.get(&"b"), Some(&2));
    assert_eq!(v3.get(&"b"), Some(&99));
    assert_eq!(v2.len(), 4);
    assert_eq!(v1.len(), 3);
    assert!(!v1.contains_key(&"d"));
}

/// Version history using Rc for cheap sharing
fn version_history() {
    let mut versions: Vec<Rc<PersistentMap<&str, i32>>> = vec![Rc::new(PersistentMap::new())];

    fn update(
        versions: &mut Vec<Rc<PersistentMap<&'static str, i32>>>,
        f: fn(&PersistentMap<&'static str, i32>) -> PersistentMap<&'static str, i32>,
    ) {
        let current = versions.last().unwrap().clone();
        versions.push(Rc::new(f(&current)));
    }

    update(&mut versions, |m| m.insert("x", 10));
    update(&mut versions, |m| m.insert("y", 20));
    update(&mut versions, |m| m.insert("z", 30));
    update(&mut versions, |m| m.remove(&"y"));

    // Current state
    let current = versions.last().unwrap();
    assert_eq!(current.get(&"x"), Some(&10));
    assert!(!current.contains_key(&"y"));
    assert_eq!(current.get(&"z"), Some(&30));

    // Access past version (after adding y)
    let v2 = &versions[2];
    assert!(v2.contains_key(&"y"));
    assert_eq!(v2.get(&"y"), Some(&20));
}

/// Undo/redo with persistent maps
struct UndoState<K, V> {
    current: PersistentMap<K, V>,
    undo_stack: Vec<PersistentMap<K, V>>,
    redo_stack: Vec<PersistentMap<K, V>>,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> UndoState<K, V> {
    fn new() -> Self {
        UndoState {
            current: PersistentMap::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    fn apply<F: FnOnce(&PersistentMap<K, V>) -> PersistentMap<K, V>>(&mut self, f: F) {
        let old = self.current.clone();
        self.current = f(&self.current);
        self.undo_stack.push(old);
        self.redo_stack.clear();
    }

    fn undo(&mut self) -> bool {
        if let Some(prev) = self.undo_stack.pop() {
            let current = std::mem::replace(&mut self.current, prev);
            self.redo_stack.push(current);
            true
        } else {
            false
        }
    }

    fn redo(&mut self) -> bool {
        if let Some(next) = self.redo_stack.pop() {
            let current = std::mem::replace(&mut self.current, next);
            self.undo_stack.push(current);
            true
        } else {
            false
        }
    }
}

fn undo_redo_test() {
    let mut state: UndoState<&str, &str> = UndoState::new();
    state.apply(|m| m.insert("name", "Alice"));
    state.apply(|m| m.insert("age", "30"));

    assert_eq!(state.current.get(&"name"), Some(&"Alice"));
    assert_eq!(state.current.get(&"age"), Some(&"30"));

    state.undo();
    assert!(!state.current.contains_key(&"age"));
    assert_eq!(state.current.get(&"name"), Some(&"Alice"));

    state.redo();
    assert_eq!(state.current.get(&"age"), Some(&"30"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persistence() {
        persistence_demo();
    }

    #[test]
    fn test_versions() {
        version_history();
    }

    #[test]
    fn test_undo_redo() {
        undo_redo_test();
    }

    #[test]
    fn test_empty_undo() {
        let mut state: UndoState<&str, i32> = UndoState::new();
        assert!(!state.undo()); // Nothing to undo
        assert!(!state.redo()); // Nothing to redo
    }
}
