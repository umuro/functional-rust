//! Hash, Eq, and Ord Traits
//!
//! Traits for equality, ordering, and hashing — enabling HashMap/HashSet/BTreeMap keys.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::{Hash, Hasher};

/// A 2D point with derived Hash, Eq.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn distance_squared(&self, other: &Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

/// Priority levels with custom ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    fn value(&self) -> u8 {
        match self {
            Priority::Low => 0,
            Priority::Medium => 1,
            Priority::High => 2,
            Priority::Critical => 3,
        }
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

/// A task with custom ordering: higher priority first, then alphabetical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub name: String,
    pub priority: Priority,
    pub id: u32,
}

impl Task {
    pub fn new(name: &str, priority: Priority, id: u32) -> Self {
        Task {
            name: name.to_string(),
            priority,
            id,
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority first (reverse), then alphabetical by name
        other
            .priority
            .cmp(&self.priority)
            .then(self.name.cmp(&other.name))
    }
}

/// A case-insensitive string wrapper.
#[derive(Debug, Clone)]
pub struct CaseInsensitive(pub String);

impl PartialEq for CaseInsensitive {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for CaseInsensitive {}

impl Hash for CaseInsensitive {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the lowercase version for consistency with Eq
        self.0.to_lowercase().hash(state);
    }
}

/// Demonstrates HashMap with Point keys.
pub fn point_map_example() -> HashMap<Point, String> {
    let mut map = HashMap::new();
    map.insert(Point::new(0, 0), "origin".to_string());
    map.insert(Point::new(1, 0), "unit-x".to_string());
    map.insert(Point::new(0, 1), "unit-y".to_string());
    map
}

/// Demonstrates HashSet deduplication.
pub fn point_set_example(points: Vec<Point>) -> HashSet<Point> {
    points.into_iter().collect()
}

/// Sorts tasks by custom ordering.
pub fn sort_tasks(tasks: &mut [Task]) {
    tasks.sort();
}

/// Groups tasks by priority using BTreeMap.
pub fn group_by_priority(tasks: &[Task]) -> BTreeMap<Priority, Vec<String>> {
    let mut groups: BTreeMap<Priority, Vec<String>> = BTreeMap::new();
    for task in tasks {
        groups
            .entry(task.priority)
            .or_default()
            .push(task.name.clone());
    }
    groups
}

/// Demonstrates case-insensitive set.
pub fn case_insensitive_set(words: Vec<&str>) -> HashSet<CaseInsensitive> {
    words
        .into_iter()
        .map(|s| CaseInsensitive(s.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_eq() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(1, 2);
        let p3 = Point::new(2, 1);
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_point_hash_map() {
        let map = point_map_example();
        assert_eq!(map.get(&Point::new(0, 0)), Some(&"origin".to_string()));
        assert_eq!(map.get(&Point::new(1, 0)), Some(&"unit-x".to_string()));
    }

    #[test]
    fn test_point_set_dedup() {
        let points = vec![
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(1, 1), // duplicate
        ];
        let set = point_set_example(points);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_priority_ord() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
    }

    #[test]
    fn test_priority_sort() {
        let mut priorities = vec![Priority::Medium, Priority::Critical, Priority::Low];
        priorities.sort();
        assert_eq!(priorities, vec![Priority::Low, Priority::Medium, Priority::Critical]);
    }

    #[test]
    fn test_task_sort_by_priority() {
        let mut tasks = vec![
            Task::new("Low task", Priority::Low, 1),
            Task::new("Critical task", Priority::Critical, 2),
        ];
        sort_tasks(&mut tasks);
        assert_eq!(tasks[0].name, "Critical task");
    }

    #[test]
    fn test_task_sort_alphabetical_within_priority() {
        let mut tasks = vec![
            Task::new("Zebra", Priority::High, 1),
            Task::new("Apple", Priority::High, 2),
        ];
        sort_tasks(&mut tasks);
        assert_eq!(tasks[0].name, "Apple");
    }

    #[test]
    fn test_group_by_priority() {
        let tasks = vec![
            Task::new("A", Priority::High, 1),
            Task::new("B", Priority::Low, 2),
            Task::new("C", Priority::High, 3),
        ];
        let groups = group_by_priority(&tasks);
        assert_eq!(groups.get(&Priority::High), Some(&vec!["A".to_string(), "C".to_string()]));
        assert_eq!(groups.get(&Priority::Low), Some(&vec!["B".to_string()]));
    }

    #[test]
    fn test_case_insensitive_eq() {
        let a = CaseInsensitive("Hello".to_string());
        let b = CaseInsensitive("HELLO".to_string());
        let c = CaseInsensitive("hello".to_string());
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    #[test]
    fn test_case_insensitive_set() {
        let set = case_insensitive_set(vec!["Hello", "HELLO", "World"]);
        assert_eq!(set.len(), 2); // "Hello" and "HELLO" are the same
    }

    #[test]
    fn test_btree_ordered_iteration() {
        let mut set = BTreeSet::new();
        set.insert(Priority::High);
        set.insert(Priority::Low);
        set.insert(Priority::Critical);
        let order: Vec<_> = set.into_iter().collect();
        assert_eq!(order, vec![Priority::Low, Priority::High, Priority::Critical]);
    }
}
