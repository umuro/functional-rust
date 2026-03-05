//! Coherence and Orphan Rules

// Can't impl Display for i32 (not our crate)
// Can't impl foreign trait for foreign type

// Newtype wrapper to work around orphan rule
pub struct Wrapper<T>(pub T);

impl std::fmt::Display for Wrapper<Vec<i32>> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }
}

// Our trait can be implemented for foreign types
pub trait Describable { fn describe(&self) -> String; }
impl Describable for i32 { fn describe(&self) -> String { format!("integer: {}", self) } }
impl Describable for String { fn describe(&self) -> String { format!("string: {}", self) } }
impl<T: Describable> Describable for Vec<T> {
    fn describe(&self) -> String { format!("vec of {} items", self.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_wrapper_display() { let w = Wrapper(vec![1,2,3]); assert_eq!(format!("{}", w), "[1, 2, 3]"); }
    #[test] fn test_i32_describe() { assert!(42.describe().contains("integer")); }
    #[test] fn test_string_describe() { assert!("hi".to_string().describe().contains("string")); }
    #[test] fn test_vec_describe() { assert!(vec![1,2,3].describe().contains("3 items")); }
}
