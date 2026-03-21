#![allow(dead_code)]
#![allow(clippy::all)]
// 1050: String Interning — Dedup Strings to IDs
// Map strings to unique integer IDs for O(1) comparison

use std::collections::HashMap;

/// Interned string handle — cheap to copy and compare
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Symbol(usize);

/// String interner: maps strings ↔ unique IDs
struct Interner {
    to_id: HashMap<String, Symbol>,
    to_str: Vec<String>,
}

impl Interner {
    fn new() -> Self {
        Interner {
            to_id: HashMap::new(),
            to_str: Vec::new(),
        }
    }

    /// Intern a string: returns existing ID or assigns a new one
    fn intern(&mut self, s: &str) -> Symbol {
        if let Some(&id) = self.to_id.get(s) {
            return id;
        }
        let id = Symbol(self.to_str.len());
        self.to_str.push(s.to_string());
        self.to_id.insert(s.to_string(), id);
        id
    }

    /// Resolve a symbol back to its string
    fn resolve(&self, sym: Symbol) -> Option<&str> {
        self.to_str.get(sym.0).map(|s| s.as_str())
    }

    /// Number of interned strings
    fn len(&self) -> usize {
        self.to_str.len()
    }
}

fn basic_interning() {
    let mut interner = Interner::new();
    let id1 = interner.intern("hello");
    let id2 = interner.intern("world");
    let id3 = interner.intern("hello"); // Same as id1

    assert_eq!(id1, id3); // Same string → same ID
    assert_ne!(id1, id2); // Different strings → different IDs
    assert_eq!(interner.resolve(id1), Some("hello"));
    assert_eq!(interner.resolve(id2), Some("world"));
    assert_eq!(interner.len(), 2); // Only 2 unique strings
}

fn fast_comparison() {
    let mut interner = Interner::new();
    let words = ["the", "cat", "sat", "on", "the", "mat", "the", "cat"];
    let ids: Vec<Symbol> = words.iter().map(|w| interner.intern(w)).collect();

    // Compare IDs instead of strings: integer comparison is O(1)
    let the_id = ids[0];
    let count = ids.iter().filter(|&&id| id == the_id).count();
    assert_eq!(count, 3); // "the" appears 3 times

    // Frequency counting with symbols is faster than with strings
    let mut freq: HashMap<Symbol, usize> = HashMap::new();
    for &id in &ids {
        *freq.entry(id).or_insert(0) += 1;
    }
    assert_eq!(freq[&the_id], 3);
}

fn symbol_table() {
    let mut interner = Interner::new();
    let vars = ["x", "y", "x", "z", "y", "x"];
    let interned: Vec<Symbol> = vars.iter().map(|v| interner.intern(v)).collect();

    // Only 3 unique symbols
    assert_eq!(interner.len(), 3);

    // Dedup using a set of symbols
    let mut unique: Vec<Symbol> = interned.clone();
    unique.sort_by_key(|s| s.0);
    unique.dedup();
    assert_eq!(unique.len(), 3);

    // Resolve back to strings
    let names: Vec<&str> = unique
        .iter()
        .filter_map(|&sym| interner.resolve(sym))
        .collect();
    assert_eq!(names.len(), 3);
}

/// Symbols work great as HashMap keys (faster than String keys)
fn symbol_as_key() {
    let mut interner = Interner::new();
    let mut values: HashMap<Symbol, i32> = HashMap::new();

    let x = interner.intern("x");
    let y = interner.intern("y");

    values.insert(x, 42);
    values.insert(y, 99);

    assert_eq!(values[&x], 42);
    assert_eq!(values[&y], 99);

    // Symbol is Copy — no cloning needed
    let key = x;
    assert_eq!(values[&key], 42);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        basic_interning();
    }

    #[test]
    fn test_fast_compare() {
        fast_comparison();
    }

    #[test]
    fn test_symbols() {
        symbol_table();
    }

    #[test]
    fn test_symbol_key() {
        symbol_as_key();
    }

    #[test]
    fn test_empty_string() {
        let mut interner = Interner::new();
        let empty = interner.intern("");
        assert_eq!(interner.resolve(empty), Some(""));
    }

    #[test]
    fn test_symbol_is_copy() {
        let mut interner = Interner::new();
        let sym = interner.intern("test");
        let copy = sym; // Copy, not move
        assert_eq!(sym, copy); // Both still valid
    }
}
