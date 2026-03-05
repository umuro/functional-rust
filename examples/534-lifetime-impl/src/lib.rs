//! Lifetimes in impl Blocks
//!
//! Lifetime annotations in impl blocks for structs with borrowed data.

/// A slice view — borrows from an underlying slice.
pub struct View<'a, T> {
    data: &'a [T],
}

impl<'a, T> View<'a, T> {
    /// Constructor: same 'a lifetime.
    pub fn new(data: &'a [T]) -> Self {
        View { data }
    }

    /// get: returns reference tied to 'a (the data's lifetime).
    pub fn get(&self, index: usize) -> Option<&'a T> {
        self.data.get(index)
    }

    /// Returns a sub-view with the same lifetime 'a.
    pub fn slice(&self, start: usize, end: usize) -> Option<View<'a, T>> {
        if start <= end && end <= self.data.len() {
            Some(View {
                data: &self.data[start..end],
            })
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a T> {
        self.data.iter()
    }
}

/// Buffer with reader — different lifetime patterns.
pub struct Buffer<'a> {
    content: &'a str,
    position: usize,
}

impl<'a> Buffer<'a> {
    pub fn new(content: &'a str) -> Self {
        Buffer {
            content,
            position: 0,
        }
    }

    /// Read n chars, return slice tied to 'a.
    pub fn read(&mut self, n: usize) -> &'a str {
        let end = (self.position + n).min(self.content.len());
        let result = &self.content[self.position..end];
        self.position = end;
        result
    }

    pub fn remaining(&self) -> &'a str {
        &self.content[self.position..]
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

/// Generic container with lifetime.
pub struct Container<'a, T> {
    items: Vec<&'a T>,
}

impl<'a, T> Container<'a, T> {
    pub fn new() -> Self {
        Container { items: Vec::new() }
    }

    pub fn add(&mut self, item: &'a T) {
        self.items.push(item);
    }

    pub fn get(&self, index: usize) -> Option<&'a T> {
        self.items.get(index).copied()
    }
}

impl<'a, T> Default for Container<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_basic() {
        let data = [1, 2, 3, 4, 5];
        let view = View::new(&data);
        assert_eq!(view.get(2), Some(&3));
        assert_eq!(view.len(), 5);
    }

    #[test]
    fn test_view_slice() {
        let data = [1, 2, 3, 4, 5];
        let view = View::new(&data);
        let sub = view.slice(1, 4).unwrap();
        assert_eq!(sub.len(), 3);
        assert_eq!(sub.get(0), Some(&2));
    }

    #[test]
    fn test_view_iter() {
        let data = [1, 2, 3];
        let view = View::new(&data);
        let sum: i32 = view.iter().sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_buffer_read() {
        let content = "Hello, World!";
        let mut buffer = Buffer::new(content);

        assert_eq!(buffer.read(5), "Hello");
        assert_eq!(buffer.read(2), ", ");
        assert_eq!(buffer.remaining(), "World!");
    }

    #[test]
    fn test_container() {
        let a = 1;
        let b = 2;
        let c = 3;

        let mut container: Container<i32> = Container::new();
        container.add(&a);
        container.add(&b);
        container.add(&c);

        assert_eq!(container.get(1), Some(&2));
    }
}
