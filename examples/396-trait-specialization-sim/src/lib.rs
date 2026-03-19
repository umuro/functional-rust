//! Simulating Trait Specialization

pub trait Process {
    fn process(&self) -> String;
}

impl<T: std::fmt::Debug> Process for T {
    fn process(&self) -> String {
        format!("Debug: {:?}", self)
    }
}

pub trait FastProcess: Process {
    fn fast_process(&self) -> String;
}
impl FastProcess for i32 {
    fn fast_process(&self) -> String {
        format!("Fast i32: {}", self)
    }
}
impl FastProcess for String {
    fn fast_process(&self) -> String {
        format!("Fast String: {}", self)
    }
}

pub fn process_any<T: Process>(val: &T) -> String {
    val.process()
}
pub fn process_fast<T: FastProcess>(val: &T) -> String {
    val.fast_process()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic() {
        assert!(process_any(&vec![1, 2, 3]).contains("Debug"));
    }
    #[test]
    fn test_i32_fast() {
        assert!(process_fast(&42i32).contains("Fast i32"));
    }
    #[test]
    fn test_string_fast() {
        assert!(process_fast(&"hello".to_string()).contains("Fast String"));
    }
    #[test]
    fn test_i32_generic() {
        assert!(process_any(&42i32).contains("Debug"));
    }
}
