// Example 110: Cell<T> — Interior Mutability for Copy Types
//
// Cell<T> allows mutation through a shared reference (&T).
// It works only with Copy types and avoids runtime borrow-check overhead
// by never handing out references to the interior — values are only moved
// in and out with `set` / `get`.

use std::cell::Cell;

// ── Approach 1: Simple counter ────────────────────────────────────────────────

pub fn counter_demo() -> u32 {
    let counter = Cell::new(0u32);
    counter.set(counter.get() + 1);
    counter.set(counter.get() + 1);
    counter.get()
}

// ── Approach 2: Mutable field inside an otherwise-immutable struct ────────────

pub struct Config {
    pub name: String,
    pub call_count: Cell<u32>,
}

impl Config {
    pub fn new(name: &str) -> Self {
        Config {
            name: name.to_string(),
            call_count: Cell::new(0),
        }
    }

    // Takes `&self` (shared reference) yet increments the counter.
    pub fn use_it(&self) {
        self.call_count.set(self.call_count.get() + 1);
    }

    pub fn count(&self) -> u32 {
        self.call_count.get()
    }
}

// ── Approach 3: Lazy / cached computation ────────────────────────────────────

pub struct CachedSquare {
    input: i32,
    cache: Cell<Option<i32>>,
}

impl CachedSquare {
    pub fn new(input: i32) -> Self {
        CachedSquare {
            input,
            cache: Cell::new(None),
        }
    }

    pub fn get(&self) -> i32 {
        match self.cache.get() {
            Some(v) => v,
            None => {
                let v = self.input * self.input;
                self.cache.set(Some(v));
                v
            }
        }
    }
}

// ── Approach 4: Cell as a flag (bool) ─────────────────────────────────────────

pub fn toggle_demo() -> (bool, bool) {
    let flag = Cell::new(false);
    let before = flag.get();
    flag.set(!flag.get());
    let after = flag.get();
    (before, after)
}

fn main() {
    // Approach 1: counter through shared ref
    let count = counter_demo();
    println!("counter_demo() = {count}");

    // Approach 2: Config with selectively mutable call_count
    let cfg = Config::new("default");
    let r1 = &cfg;
    let r2 = &cfg; // two shared refs simultaneously — allowed!
    r1.use_it();
    r2.use_it();
    println!("Config '{}' used {} times", cfg.name, cfg.count());

    // Approach 3: lazy cached square
    let cs = CachedSquare::new(7);
    println!("CachedSquare(7).get() = {}", cs.get());
    println!("CachedSquare(7).get() again = {} (from cache)", cs.get());

    // Approach 4: bool toggle
    let (before, after) = toggle_demo();
    println!("toggle: before={before}, after={after}");

    // Cell::replace — swap and return old value
    let c = Cell::new(42i32);
    let old = c.replace(99);
    println!("replace(99): old={old}, new={}", c.get());
}

/* Output:
   counter_demo() = 2
   Config 'default' used 2 times
   CachedSquare(7).get() = 49
   CachedSquare(7).get() again = 49 (from cache)
   toggle: before=false, after=true
   replace(99): old=42, new=99
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increments_via_shared_ref() {
        let c = Cell::new(0u32);
        let r = &c;
        r.set(r.get() + 10);
        r.set(r.get() + 5);
        assert_eq!(c.get(), 15);
    }

    #[test]
    fn test_counter_demo_returns_two() {
        assert_eq!(counter_demo(), 2);
    }

    #[test]
    fn test_config_call_count_through_shared_ref() {
        let cfg = Config::new("test");
        let r1 = &cfg;
        let r2 = &cfg;
        r1.use_it();
        r2.use_it();
        r1.use_it();
        assert_eq!(cfg.count(), 3);
    }

    #[test]
    fn test_config_starts_at_zero() {
        let cfg = Config::new("fresh");
        assert_eq!(cfg.count(), 0);
    }

    #[test]
    fn test_cached_square_computed_once() {
        let cs = CachedSquare::new(7);
        assert_eq!(cs.get(), 49);
        assert_eq!(cs.get(), 49);
        assert_eq!(cs.cache.get(), Some(49));
    }

    #[test]
    fn test_cached_square_negative_input() {
        let cs = CachedSquare::new(-4);
        assert_eq!(cs.get(), 16);
    }

    #[test]
    fn test_toggle_demo() {
        let (before, after) = toggle_demo();
        assert!(!before);
        assert!(after);
    }

    #[test]
    fn test_cell_replace() {
        let c = Cell::new(42i32);
        let old = c.replace(99);
        assert_eq!(old, 42);
        assert_eq!(c.get(), 99);
    }
}
