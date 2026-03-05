// Coherence and orphan rules in Rust
use std::fmt;

// Our own trait
trait Summarize {
    fn summarize(&self) -> String;
}

// Our own type
struct Article { title: String, content: String }

// VALID: our trait + our type
impl Summarize for Article {
    fn summarize(&self) -> String {
        format!("{}: {}...", self.title, &self.content[..self.content.len().min(50)])
    }
}

// VALID: our trait + foreign type (String, Vec<i32>)
impl Summarize for String {
    fn summarize(&self) -> String { format!(""{}"", &self[..self.len().min(20)]) }
}

impl Summarize for Vec<i32> {
    fn summarize(&self) -> String { format!("{} integers", self.len()) }
}

// NOT VALID: foreign trait (Display) + foreign type (i64) — would fail!
// impl fmt::Display for i64 {}  // ERROR: orphan rule violation

// Workaround: newtype wrapper
struct Wrapper(Vec<i32>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }
}

// Coherence: only ONE impl per (Trait, Type) pair
// impl Summarize for Article { ... } // Would be a compile error if we had two

fn print_summary(item: &impl Summarize) {
    println!("{}", item.summarize());
}

fn main() {
    let article = Article {
        title: "Rust Rocks".to_string(),
        content: "Rust is a systems programming language focused on safety.".to_string(),
    };
    print_summary(&article);
    print_summary(&"Hello World".to_string());
    print_summary(&vec![1, 2, 3, 4, 5]);

    // Newtype bypasses orphan rule:
    let w = Wrapper(vec![10, 20, 30]);
    println!("Wrapper Display: {}", w);

    println!("\nCoherence: one impl per (Trait, Type) — enforced at compile time");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_our_trait_our_type() {
        let a = Article { title: "T".to_string(), content: "C".to_string() };
        assert!(a.summarize().contains("T"));
    }

    #[test]
    fn test_our_trait_foreign_type() {
        let s = "hello".to_string();
        assert!(s.summarize().contains("hello"));
    }

    #[test]
    fn test_newtype_display() {
        let w = Wrapper(vec![1, 2]);
        assert_eq!(w.to_string(), "[1, 2]");
    }
}
