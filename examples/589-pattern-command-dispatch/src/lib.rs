//! # Command Dispatch Pattern
//!
//! Use enums to represent commands and pattern matching for dispatch.

use std::collections::HashMap;

/// Commands that can be executed on a key-value store.
#[derive(Debug, Clone, PartialEq)]
pub enum Cmd {
    Set(String, i64),
    Remove(String),
    Increment(String, i64),
    Decrement(String, i64),
    Clear,
    Get(String),
}

/// A simple key-value store with command history.
#[derive(Debug, Default, Clone)]
pub struct Store {
    data: HashMap<String, i64>,
    history: Vec<Cmd>,
}

impl Store {
    /// Create a new empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Execute a command, modifying the store.
    pub fn execute(&mut self, cmd: Cmd) -> Option<i64> {
        let result = match &cmd {
            Cmd::Set(k, v) => {
                self.data.insert(k.clone(), *v);
                Some(*v)
            }
            Cmd::Remove(k) => self.data.remove(k),
            Cmd::Increment(k, d) => {
                let entry = self.data.entry(k.clone()).or_default();
                *entry += d;
                Some(*entry)
            }
            Cmd::Decrement(k, d) => {
                let entry = self.data.entry(k.clone()).or_default();
                *entry -= d;
                Some(*entry)
            }
            Cmd::Clear => {
                self.data.clear();
                None
            }
            Cmd::Get(k) => self.data.get(k).copied(),
        };
        self.history.push(cmd);
        result
    }

    /// Get a value from the store.
    pub fn get(&self, k: &str) -> Option<i64> {
        self.data.get(k).copied()
    }

    /// Get all keys.
    pub fn keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    /// Get command history.
    pub fn history(&self) -> &[Cmd] {
        &self.history
    }
}

/// Pure command application (no side effects).
pub fn apply(mut data: HashMap<String, i64>, cmd: &Cmd) -> HashMap<String, i64> {
    match cmd {
        Cmd::Set(k, v) => {
            data.insert(k.clone(), *v);
        }
        Cmd::Remove(k) => {
            data.remove(k);
        }
        Cmd::Increment(k, d) => {
            *data.entry(k.clone()).or_default() += d;
        }
        Cmd::Decrement(k, d) => {
            *data.entry(k.clone()).or_default() -= d;
        }
        Cmd::Clear => {
            data.clear();
        }
        Cmd::Get(_) => {} // Read-only, no change
    }
    data
}

/// Replay commands to reconstruct state.
pub fn replay(commands: &[Cmd]) -> HashMap<String, i64> {
    commands.iter().fold(HashMap::new(), |acc, c| apply(acc, c))
}

/// Validate a command before execution.
pub fn validate(cmd: &Cmd) -> Result<(), &'static str> {
    match cmd {
        Cmd::Set(k, _)
        | Cmd::Remove(k)
        | Cmd::Increment(k, _)
        | Cmd::Decrement(k, _)
        | Cmd::Get(k) => {
            if k.is_empty() {
                Err("Key cannot be empty")
            } else {
                Ok(())
            }
        }
        Cmd::Clear => Ok(()),
    }
}

/// Describe a command for logging.
pub fn describe(cmd: &Cmd) -> String {
    match cmd {
        Cmd::Set(k, v) => format!("SET {} = {}", k, v),
        Cmd::Remove(k) => format!("REMOVE {}", k),
        Cmd::Increment(k, d) => format!("INCR {} by {}", k, d),
        Cmd::Decrement(k, d) => format!("DECR {} by {}", k, d),
        Cmd::Clear => "CLEAR".into(),
        Cmd::Get(k) => format!("GET {}", k),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get() {
        let mut store = Store::new();
        store.execute(Cmd::Set("x".into(), 42));
        assert_eq!(store.get("x"), Some(42));
    }

    #[test]
    fn test_remove() {
        let mut store = Store::new();
        store.execute(Cmd::Set("x".into(), 1));
        store.execute(Cmd::Remove("x".into()));
        assert_eq!(store.get("x"), None);
    }

    #[test]
    fn test_increment() {
        let mut store = Store::new();
        store.execute(Cmd::Increment("n".into(), 1));
        store.execute(Cmd::Increment("n".into(), 2));
        assert_eq!(store.get("n"), Some(3));
    }

    #[test]
    fn test_decrement() {
        let mut store = Store::new();
        store.execute(Cmd::Set("n".into(), 10));
        store.execute(Cmd::Decrement("n".into(), 3));
        assert_eq!(store.get("n"), Some(7));
    }

    #[test]
    fn test_clear() {
        let mut store = Store::new();
        store.execute(Cmd::Set("x".into(), 1));
        store.execute(Cmd::Set("y".into(), 2));
        store.execute(Cmd::Clear);
        assert!(store.keys().is_empty());
    }

    #[test]
    fn test_history() {
        let mut store = Store::new();
        store.execute(Cmd::Set("x".into(), 1));
        store.execute(Cmd::Increment("x".into(), 5));
        assert_eq!(store.history().len(), 2);
    }

    #[test]
    fn test_replay() {
        let commands = vec![
            Cmd::Set("x".into(), 1),
            Cmd::Set("y".into(), 2),
            Cmd::Increment("x".into(), 9),
            Cmd::Remove("y".into()),
        ];
        let state = replay(&commands);
        assert_eq!(state.get("x"), Some(&10));
        assert_eq!(state.get("y"), None);
    }

    #[test]
    fn test_validate() {
        assert!(validate(&Cmd::Set("key".into(), 1)).is_ok());
        assert!(validate(&Cmd::Set("".into(), 1)).is_err());
        assert!(validate(&Cmd::Clear).is_ok());
    }

    #[test]
    fn test_describe() {
        assert_eq!(describe(&Cmd::Set("x".into(), 42)), "SET x = 42");
        assert_eq!(describe(&Cmd::Clear), "CLEAR");
    }

    #[test]
    fn test_pure_apply() {
        let mut data = HashMap::new();
        data = apply(data, &Cmd::Set("a".into(), 5));
        data = apply(data, &Cmd::Increment("a".into(), 3));
        assert_eq!(data.get("a"), Some(&8));
    }
}
