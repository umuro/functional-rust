// Default method implementations in Rust
use std::fmt;

trait Collection {
    type Item: PartialEq + fmt::Debug + Clone;

    // Core methods — must implement
    fn items(&self) -> &[Self::Item];

    // Default methods — free from core
    fn is_empty(&self) -> bool {
        self.items().is_empty()
    }

    fn len(&self) -> usize {
        self.items().len()
    }

    fn contains(&self, item: &Self::Item) -> bool {
        self.items().contains(item)
    }

    fn count_if(&self, predicate: impl Fn(&Self::Item) -> bool) -> usize {
        self.items().iter().filter(|x| predicate(x)).count()
    }

    fn any(&self, predicate: impl Fn(&Self::Item) -> bool) -> bool {
        self.items().iter().any(predicate)
    }

    fn all(&self, predicate: impl Fn(&Self::Item) -> bool) -> bool {
        self.items().iter().all(predicate)
    }

    fn first(&self) -> Option<&Self::Item> {
        self.items().first()
    }

    fn last(&self) -> Option<&Self::Item> {
        self.items().last()
    }

    fn to_vec(&self) -> Vec<Self::Item> {
        self.items().to_vec()
    }

    fn describe(&self) -> String where Self::Item: fmt::Display {
        let s: Vec<String> = self.items().iter().map(|x| x.to_string()).collect();
        format!("[{}]", s.join(", "))
    }
}

struct IntVec(Vec<i32>);
struct StrVec(Vec<String>);

impl Collection for IntVec {
    type Item = i32;
    fn items(&self) -> &[i32] { &self.0 }
}

impl Collection for StrVec {
    type Item = String;
    fn items(&self) -> &[String] { &self.0 }
    // Override just one default:
    fn is_empty(&self) -> bool { self.0.is_empty() }
}

fn main() {
    let v = IntVec(vec![1, 2, 3, 4, 5]);
    println!("is_empty: {}", v.is_empty());
    println!("len: {}", v.len());
    println!("contains 3: {}", v.contains(&3));
    println!("any > 4: {}", v.any(|x| *x > 4));
    println!("all > 0: {}", v.all(|x| *x > 0));
    println!("count > 2: {}", v.count_if(|x| *x > 2));
    println!("describe: {}", v.describe());

    let sv = StrVec(vec!["hello".into(), "world".into()]);
    println!("StrVec contains 'hello': {}", sv.contains(&"hello".to_string()));
    println!("StrVec first: {:?}", sv.first());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let v = IntVec(vec![10, 20, 30]);
        assert_eq!(v.len(), 3);
        assert!(!v.is_empty());
        assert!(v.contains(&20));
        assert!(v.any(|x| *x > 25));
        assert!(!v.all(|x| *x > 15));
    }
}
