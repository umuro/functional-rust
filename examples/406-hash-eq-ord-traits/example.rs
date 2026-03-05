// Hash, Eq, Ord implementations in Rust
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, BTreeMap};

// Derive common case
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point { x: i32, y: i32 }

// Custom Ord for domain-specific ordering
#[derive(Debug, Clone, PartialEq, Eq)]
enum Priority { Low, Medium, High, Critical }

impl Priority {
    fn value(&self) -> u8 {
        match self { Priority::Low => 0, Priority::Medium => 1,
                     Priority::High => 2, Priority::Critical => 3 }
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering { self.value().cmp(&other.value()) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Task { name: String, priority: Priority, id: u32 }

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority first, then alphabetical name
        other.priority.cmp(&self.priority)
            .then(self.name.cmp(&other.name))
    }
}

fn main() {
    // Hash: use Point as HashMap key
    let mut map: HashMap<Point, String> = HashMap::new();
    map.insert(Point { x: 0, y: 0 }, "origin".to_string());
    map.insert(Point { x: 1, y: 2 }, "point A".to_string());
    println!("Origin: {:?}", map[&Point { x: 0, y: 0 }]);

    // HashSet with custom Eq + Hash
    let mut set: HashSet<Point> = HashSet::new();
    set.insert(Point { x: 1, y: 1 });
    set.insert(Point { x: 1, y: 1 }); // duplicate
    println!("Set size: {}", set.len()); // 1

    // Ord: sort tasks by priority
    let mut tasks = vec![
        Task { name: "Fix bug".to_string(), priority: Priority::Critical, id: 1 },
        Task { name: "Write docs".to_string(), priority: Priority::Low, id: 2 },
        Task { name: "Review PR".to_string(), priority: Priority::High, id: 3 },
        Task { name: "Deploy".to_string(), priority: Priority::High, id: 4 },
    ];
    tasks.sort();
    println!("\nTasks by priority:");
    for t in &tasks {
        println!("  [{:?}] {}", t.priority, t.name);
    }

    // BTreeMap (uses Ord)
    let mut btree: BTreeMap<Priority, Vec<String>> = BTreeMap::new();
    for t in &tasks {
        btree.entry(t.priority.clone()).or_default().push(t.name.clone());
    }
    println!("\nGrouped:");
    for (p, names) in &btree {
        println!("  {:?}: {:?}", p, names);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ord() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::Low < Priority::Medium);
    }

    #[test]
    fn test_point_hash() {
        let mut map = HashMap::new();
        map.insert(Point { x: 1, y: 2 }, 42);
        assert_eq!(map[&Point { x: 1, y: 2 }], 42);
    }

    #[test]
    fn test_task_sort() {
        let mut tasks = vec![
            Task { name: "B".to_string(), priority: Priority::Low, id: 1 },
            Task { name: "A".to_string(), priority: Priority::Critical, id: 2 },
        ];
        tasks.sort();
        assert_eq!(tasks[0].name, "A"); // Critical comes first
    }
}
