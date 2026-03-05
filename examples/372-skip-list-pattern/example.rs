use std::collections::BTreeMap;

// Skip list concept: we use BTreeMap as the backing store with level simulation
// Real skip list needs unsafe Rust or Rc<RefCell<>> for linked nodes

struct SkipListSimulation {
    // Level 0: all elements (base level)
    level0: Vec<i32>,
    // Level 1: every ~2nd element (express lane)
    level1: Vec<i32>,
    // Level 2: every ~4th element (super express lane)
    level2: Vec<i32>,
}

impl SkipListSimulation {
    fn new() -> Self { Self { level0: Vec::new(), level1: Vec::new(), level2: Vec::new() } }

    fn rebuild_levels(&mut self) {
        self.level1 = self.level0.iter().step_by(2).copied().collect();
        self.level2 = self.level0.iter().step_by(4).copied().collect();
    }

    fn insert(&mut self, val: i32) {
        match self.level0.binary_search(&val) {
            Ok(_) => return, // already exists
            Err(i) => self.level0.insert(i, val),
        }
        self.rebuild_levels();
    }

    fn search(&self, val: i32) -> bool {
        // Search using express lanes (top-down)
        let start = match self.level2.binary_search(&val) {
            Ok(_) => return true,
            Err(i) => if i > 0 { self.level2[i-1] } else { i32::MIN },
        };
        let start1 = match self.level1.binary_search(&val) {
            Ok(_) => return true,
            Err(i) => if i > 0 { self.level1[i-1].max(start) } else { start },
        };
        // Final scan in level0
        let begin = self.level0.partition_point(|&x| x < start1);
        for &x in &self.level0[begin..] {
            if x == val { return true; }
            if x > val { break; }
        }
        false
    }

    fn delete(&mut self, val: i32) -> bool {
        if let Ok(i) = self.level0.binary_search(&val) {
            self.level0.remove(i);
            self.rebuild_levels();
            true
        } else { false }
    }
}

fn main() {
    let mut sl = SkipListSimulation::new();
    for v in [5,3,7,1,9,4,6,2,8,0,10] { sl.insert(v); }
    println!("Level 0: {:?}", sl.level0);
    println!("Level 1 (express): {:?}", sl.level1);
    println!("Level 2 (super): {:?}", sl.level2);
    println!("Search 7: {}", sl.search(7));
    println!("Search 11: {}", sl.search(11));
    sl.delete(5);
    println!("After delete 5: {:?}", sl.level0);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn insert_search() {
        let mut sl = SkipListSimulation::new();
        for v in [3,1,4,1,5,9,2,6] { sl.insert(v); }
        assert!(sl.search(5)); assert!(!sl.search(7));
    }
    #[test] fn delete() {
        let mut sl = SkipListSimulation::new();
        sl.insert(3); sl.insert(5); sl.insert(7);
        assert!(sl.delete(5)); assert!(!sl.search(5));
    }
    #[test] fn sorted_order() {
        let mut sl = SkipListSimulation::new();
        for v in [5,2,8,1,9,3] { sl.insert(v); }
        assert_eq!(sl.level0, vec![1,2,3,5,8,9]);
    }
}
