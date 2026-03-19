//! Marker Traits

pub trait Serializable {}
pub trait Immutable {}
pub trait ThreadSafe: Send + Sync {}

#[derive(Clone)]
pub struct Config {
    pub name: String,
}
impl Serializable for Config {}
impl Immutable for Config {}

pub struct Counter {
    pub value: std::sync::atomic::AtomicU64,
}
impl ThreadSafe for Counter {}
unsafe impl Send for Counter {}
unsafe impl Sync for Counter {}

pub fn save<T: Serializable>(val: &T) -> String {
    "saved".to_string()
}
pub fn process_threadsafe<T: ThreadSafe>(val: &T) -> String {
    "processed".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serializable() {
        let c = Config {
            name: "test".into(),
        };
        assert_eq!(save(&c), "saved");
    }
    #[test]
    fn test_threadsafe() {
        let c = Counter {
            value: std::sync::atomic::AtomicU64::new(0),
        };
        assert_eq!(process_threadsafe(&c), "processed");
    }
    #[test]
    fn test_marker_has_no_methods() {
        let _c = Config { name: "x".into() }; /* Marker traits have no methods */
    }
}
