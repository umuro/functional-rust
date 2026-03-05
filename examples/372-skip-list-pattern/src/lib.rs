//! Skip List Pattern
//!
//! Probabilistic data structure with O(log n) search using express lanes.

/// Skip list simulation using sorted vectors at multiple levels
pub struct SkipList {
    level0: Vec<i32>,
    level1: Vec<i32>,
    level2: Vec<i32>,
}

impl SkipList {
    pub fn new() -> Self {
        Self {
            level0: Vec::new(),
            level1: Vec::new(),
            level2: Vec::new(),
        }
    }

    fn rebuild_levels(&mut self) {
        self.level1 = self.level0.iter().step_by(2).copied().collect();
        self.level2 = self.level0.iter().step_by(4).copied().collect();
    }

    pub fn insert(&mut self, val: i32) {
        match self.level0.binary_search(&val) {
            Ok(_) => return,
            Err(i) => self.level0.insert(i, val),
        }
        self.rebuild_levels();
    }

    pub fn search(&self, val: i32) -> bool {
        // Use express lanes (top-down search)
        if self.level2.binary_search(&val).is_ok() {
            return true;
        }
        if self.level1.binary_search(&val).is_ok() {
            return true;
        }
        self.level0.binary_search(&val).is_ok()
    }

    pub fn delete(&mut self, val: i32) -> bool {
        if let Ok(i) = self.level0.binary_search(&val) {
            self.level0.remove(i);
            self.rebuild_levels();
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.level0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.level0.is_empty()
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.level0.clone()
    }
}

impl Default for SkipList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_search() {
        let mut sl = SkipList::new();
        for v in [3, 1, 4, 1, 5, 9, 2, 6] {
            sl.insert(v);
        }
        assert!(sl.search(5));
        assert!(sl.search(3));
        assert!(!sl.search(7));
    }

    #[test]
    fn test_delete() {
        let mut sl = SkipList::new();
        sl.insert(1);
        sl.insert(2);
        sl.insert(3);
        assert!(sl.delete(2));
        assert!(!sl.search(2));
        assert!(sl.search(1));
        assert!(sl.search(3));
    }

    #[test]
    fn test_sorted_order() {
        let mut sl = SkipList::new();
        for v in [5, 3, 7, 1, 9] {
            sl.insert(v);
        }
        assert_eq!(sl.to_vec(), vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_duplicates() {
        let mut sl = SkipList::new();
        sl.insert(1);
        sl.insert(1);
        sl.insert(1);
        assert_eq!(sl.len(), 1);
    }

    #[test]
    fn test_empty() {
        let sl = SkipList::new();
        assert!(sl.is_empty());
        assert!(!sl.search(1));
    }
}
