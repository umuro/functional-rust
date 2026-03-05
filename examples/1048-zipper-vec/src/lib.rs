// 1048: Vec Zipper — Cursor (Left, Focus, Right)
// A zipper provides O(1) local navigation and modification

/// A zipper over a sequence: left (reversed), focus, right
#[derive(Debug, Clone)]
struct Zipper<T> {
    left: Vec<T>,   // reversed: closest to focus is last
    focus: T,
    right: Vec<T>,
}

impl<T: Clone> Zipper<T> {
    /// Create from a non-empty slice
    fn from_slice(data: &[T]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }
        Some(Zipper {
            left: Vec::new(),
            focus: data[0].clone(),
            right: data[1..].to_vec(),
        })
    }

    /// Reconstruct the full sequence
    fn to_vec(&self) -> Vec<T> {
        let mut result: Vec<T> = self.left.iter().cloned().collect();
        result.push(self.focus.clone());
        result.extend(self.right.iter().cloned());
        result
    }

    /// Move focus right
    fn move_right(&mut self) -> bool {
        if self.right.is_empty() {
            return false;
        }
        let old_focus = std::mem::replace(&mut self.focus, self.right.remove(0));
        self.left.push(old_focus);
        true
    }

    /// Move focus left
    fn move_left(&mut self) -> bool {
        if self.left.is_empty() {
            return false;
        }
        let old_focus = std::mem::replace(&mut self.focus, self.left.pop().unwrap());
        self.right.insert(0, old_focus);
        true
    }

    /// Move to start
    fn move_to_start(&mut self) {
        while self.move_left() {}
    }

    /// Move to end
    fn move_to_end(&mut self) {
        while self.move_right() {}
    }

    /// Set focus value
    fn set(&mut self, value: T) {
        self.focus = value;
    }

    // Note: generic modify needs T: Default (see modify_safe below)

    /// Insert element to the right of focus
    fn insert_right(&mut self, value: T) {
        self.right.insert(0, value);
    }

    /// Insert element to the left of focus
    fn insert_left(&mut self, value: T) {
        self.left.push(value);
    }

    /// Delete element to the right of focus
    fn delete_right(&mut self) -> Option<T> {
        if self.right.is_empty() {
            None
        } else {
            Some(self.right.remove(0))
        }
    }

    fn focus(&self) -> &T {
        &self.focus
    }
}

// Safe modify without zeroed()
impl<T: Default> Zipper<T> {
    fn modify_safe<F: FnOnce(T) -> T>(&mut self, f: F) {
        let old = std::mem::take(&mut self.focus);
        self.focus = f(old);
    }
}

fn navigation_test() {
    let mut z = Zipper::from_slice(&[1, 2, 3, 4, 5]).unwrap();
    assert_eq!(*z.focus(), 1);

    assert!(z.move_right());
    assert_eq!(*z.focus(), 2);

    assert!(z.move_right());
    assert_eq!(*z.focus(), 3);

    assert!(z.move_left());
    assert_eq!(*z.focus(), 2);

    assert_eq!(z.to_vec(), vec![1, 2, 3, 4, 5]);
}

fn modification_test() {
    let mut z = Zipper::from_slice(&[1, 2, 3, 4, 5]).unwrap();
    z.move_right();
    z.move_right();
    z.set(99);
    assert_eq!(z.to_vec(), vec![1, 2, 99, 4, 5]);

    z.modify_safe(|x| x * 2);
    assert_eq!(*z.focus(), 198);
}

fn editor_test() {
    let mut z = Zipper::from_slice(&['h', 'e', 'l', 'o']).unwrap();
    z.move_right(); // e
    z.move_right(); // l
    z.insert_right('l'); // insert 'l' after current
    assert_eq!(z.to_vec(), vec!['h', 'e', 'l', 'l', 'o']);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation() { navigation_test(); }

    #[test]
    fn test_modification() { modification_test(); }

    #[test]
    fn test_editor() { editor_test(); }

    #[test]
    fn test_boundaries() {
        let mut z = Zipper::from_slice(&[1]).unwrap();
        assert!(!z.move_left());
        assert!(!z.move_right());
        assert_eq!(*z.focus(), 1);
    }

    #[test]
    fn test_move_to_extremes() {
        let mut z = Zipper::from_slice(&[1, 2, 3, 4, 5]).unwrap();
        z.move_to_end();
        assert_eq!(*z.focus(), 5);
        z.move_to_start();
        assert_eq!(*z.focus(), 1);
    }
}
