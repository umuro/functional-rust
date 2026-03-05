//! # 534. Lifetimes in impl Blocks
//! Lifetime annotations in impl blocks for structs with borrowed data.

/// A slice view — borrows from an underlying slice
struct View<'a, T> {
    data: &'a [T],
}

impl<'a, T> View<'a, T> {
    /// Constructor: no extra lifetime needed — same 'a
    fn new(data: &'a [T]) -> Self {
        View { data }
    }

    /// get: returns reference tied to 'a (the data's lifetime), NOT self
    fn get(&self, index: usize) -> Option<&'a T> {
        self.data.get(index)
    }

    /// Returns a sub-view with the same lifetime 'a
    fn slice(&self, start: usize, end: usize) -> Option<View<'a, T>> {
        let end = end.min(self.data.len());
        if start > end { return None; }
        Some(View { data: &self.data[start..end] })
    }

    fn len(&self) -> usize { self.data.len() }
    fn is_empty(&self) -> bool { self.data.is_empty() }
    fn iter(&self) -> std::slice::Iter<'a, T> { self.data.iter() }
}

impl<'a, T: std::fmt::Debug> View<'a, T> {
    fn debug_print(&self) {
        println!("View({} items): {:?}", self.len(), self.data);
    }
}

/// Method that returns reference tied to argument, not self
struct Formatter<'a> {
    prefix: &'a str,
}

impl<'a> Formatter<'a> {
    fn new(prefix: &'a str) -> Self { Formatter { prefix } }

    /// Returns a reference to self.prefix — tied to 'a (longer lifetime)
    fn get_prefix(&self) -> &'a str {
        self.prefix // 'a, not the borrow-of-self lifetime
    }

    /// Format: returns owned String — no lifetime issue
    fn format(&self, s: &str) -> String {
        format!("{}{}", self.prefix, s)
    }
}

fn main() {
    let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let view = View::new(&data);
    view.debug_print();

    println!("view[2] = {:?}", view.get(2));
    println!("view[99] = {:?}", view.get(99));

    if let Some(sub) = view.slice(2, 7) {
        sub.debug_print();
        println!("sub[0] = {:?}", sub.get(0));

        // Key: sub.get() returns &'a T — valid even after sub is dropped
        let elem = sub.get(1);
        drop(sub); // sub dropped — but elem is still valid (tied to data, not sub)
        println!("elem after sub drop: {:?}", elem);
    }

    // Formatter: prefix reference outlives self
    let prefix = String::from(">> ");
    let fmt;
    {
        let f = Formatter::new(&prefix);
        fmt = f.get_prefix(); // returns &'a str tied to prefix, not f
        println!("prefix from formatter: {}", fmt);
    } // f dropped — but fmt only borrows prefix!
    println!("prefix still valid after formatter drop: {}", fmt);
    println!("formatted: {}", Formatter::new(&prefix).format("hello"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_get() {
        let v = vec![1, 2, 3];
        let view = View::new(&v);
        assert_eq!(view.get(0), Some(&1));
        assert_eq!(view.get(10), None);
    }

    #[test]
    fn test_view_slice() {
        let v = vec![10, 20, 30, 40, 50];
        let view = View::new(&v);
        let sub = view.slice(1, 4).unwrap();
        assert_eq!(sub.len(), 3);
        assert_eq!(sub.get(0), Some(&20));
    }

    #[test]
    fn test_formatter_prefix_outlives_formatter() {
        let prefix = String::from("prefix:");
        let result;
        {
            let f = Formatter::new(&prefix);
            result = f.get_prefix();
        } // f dropped
        assert_eq!(result, "prefix:"); // still valid
    }
}
