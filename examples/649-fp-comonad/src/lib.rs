//! # Comonad
//!
//! Comonads are the dual of monads:
//! - `extract`: Pull a value out (dual of `pure`)
//! - `extend`: Apply contextual computation (dual of `bind`)
//! - `duplicate`: Nest contexts (dual of `join`)

/// Trait for comonads
pub trait Comonad: Sized {
    type Item;
    
    /// Extract the focused value
    fn extract(&self) -> Self::Item;
    
    /// Extend a computation to all positions
    fn extend<B, F>(self, f: F) -> Self
    where
        F: Fn(&Self) -> B,
        Self: ComonadOutput<B>;
    
    /// Duplicate: wrap in another layer
    fn duplicate(self) -> Self
    where
        Self: Clone + ComonadOutput<Self>,
    {
        self.extend(|w| w.clone())
    }
}

pub trait ComonadOutput<B>: Comonad {
    type Output: Comonad<Item = B>;
    fn extend_impl<F>(self, f: F) -> Self::Output
    where
        F: Fn(&Self) -> B;
}

// Approach 1: NonEmpty list comonad
#[derive(Debug, Clone, PartialEq)]
pub struct NonEmpty<T> {
    pub head: T,
    pub tail: Vec<T>,
}

impl<T: Clone> NonEmpty<T> {
    pub fn new(head: T, tail: Vec<T>) -> Self {
        NonEmpty { head, tail }
    }
    
    pub fn singleton(value: T) -> Self {
        NonEmpty { head: value, tail: Vec::new() }
    }
    
    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }
    
    pub fn to_vec(&self) -> Vec<T> {
        let mut v = vec![self.head.clone()];
        v.extend(self.tail.clone());
        v
    }
}

impl<T: Clone> Comonad for NonEmpty<T> {
    type Item = T;
    
    fn extract(&self) -> T {
        self.head.clone()
    }
    
    fn extend<B, F>(self, f: F) -> Self
    where
        F: Fn(&Self) -> B,
        Self: ComonadOutput<B>,
    {
        // This would need proper implementation with ComonadOutput
        // Simplified version:
        self
    }
}

// Approach 2: Store comonad (focus + context)
#[derive(Clone)]
pub struct Store<S, A> {
    peek: Box<dyn Fn(S) -> A>,
    pos: S,
}

impl<S: Clone + 'static, A: Clone + 'static> Store<S, A> {
    pub fn new<F: Fn(S) -> A + 'static>(f: F, pos: S) -> Self {
        Store { peek: Box::new(f), pos }
    }
    
    pub fn extract(&self) -> A {
        (self.peek)(self.pos.clone())
    }
    
    pub fn peek_at(&self, s: S) -> A {
        (self.peek)(s)
    }
    
    pub fn seek(self, s: S) -> Self {
        Store { peek: self.peek, pos: s }
    }
    
    pub fn seeks<F: FnOnce(S) -> S>(self, f: F) -> Self {
        let new_pos = f(self.pos);
        Store { peek: self.peek, pos: new_pos }
    }
}

// Approach 3: Zipper comonad (list with focus)
#[derive(Debug, Clone, PartialEq)]
pub struct Zipper<T> {
    left: Vec<T>,   // reversed
    focus: T,
    right: Vec<T>,
}

impl<T: Clone> Zipper<T> {
    pub fn new(left: Vec<T>, focus: T, right: Vec<T>) -> Self {
        Zipper { left, focus, right }
    }
    
    pub fn from_vec(mut v: Vec<T>) -> Option<Self> {
        if v.is_empty() {
            None
        } else {
            let focus = v.remove(0);
            Some(Zipper { left: Vec::new(), focus, right: v })
        }
    }
    
    pub fn extract(&self) -> T {
        self.focus.clone()
    }
    
    pub fn move_left(&self) -> Option<Self> {
        if self.left.is_empty() {
            None
        } else {
            let mut new_left = self.left.clone();
            let new_focus = new_left.pop()?;
            let mut new_right = vec![self.focus.clone()];
            new_right.extend(self.right.clone());
            Some(Zipper { left: new_left, focus: new_focus, right: new_right })
        }
    }
    
    pub fn move_right(&self) -> Option<Self> {
        if self.right.is_empty() {
            None
        } else {
            let mut new_right = self.right.clone();
            let new_focus = new_right.remove(0);
            let mut new_left = self.left.clone();
            new_left.push(self.focus.clone());
            Some(Zipper { left: new_left, focus: new_focus, right: new_right })
        }
    }
    
    pub fn to_vec(&self) -> Vec<T> {
        let mut v: Vec<T> = self.left.iter().rev().cloned().collect();
        v.push(self.focus.clone());
        v.extend(self.right.clone());
        v
    }
    
    /// Extend: apply function to all positions
    pub fn extend<F, B: Clone>(&self, f: F) -> Zipper<B>
    where
        F: Fn(&Zipper<T>) -> B,
    {
        let mut lefts = Vec::new();
        let mut current = self.clone();
        while let Some(prev) = current.move_left() {
            current = prev;
            lefts.push(f(&current));
        }
        lefts.reverse();
        
        let focus = f(self);
        
        let mut rights = Vec::new();
        let mut current = self.clone();
        while let Some(next) = current.move_right() {
            current = next;
            rights.push(f(&current));
        }
        
        Zipper { left: lefts, focus, right: rights }
    }
}

/// Moving average using zipper comonad
pub fn moving_average(window: usize) -> impl Fn(&Zipper<f64>) -> f64 {
    move |z: &Zipper<f64>| {
        let half = window / 2;
        let mut sum = z.extract();
        let mut count = 1;
        
        let mut left = z.clone();
        for _ in 0..half {
            if let Some(l) = left.move_left() {
                sum += l.extract();
                count += 1;
                left = l;
            } else {
                break;
            }
        }
        
        let mut right = z.clone();
        for _ in 0..half {
            if let Some(r) = right.move_right() {
                sum += r.extract();
                count += 1;
                right = r;
            } else {
                break;
            }
        }
        
        sum / count as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonempty_extract() {
        let ne = NonEmpty::new(42, vec![1, 2, 3]);
        assert_eq!(ne.extract(), 42);
    }

    #[test]
    fn test_nonempty_singleton() {
        let ne = NonEmpty::singleton(5);
        assert_eq!(ne.len(), 1);
        assert_eq!(ne.extract(), 5);
    }

    #[test]
    fn test_zipper_from_vec() {
        let z = Zipper::from_vec(vec![1, 2, 3, 4, 5]).unwrap();
        assert_eq!(z.extract(), 1);
        assert_eq!(z.to_vec(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_zipper_move_right() {
        let z = Zipper::from_vec(vec![1, 2, 3]).unwrap();
        let z2 = z.move_right().unwrap();
        assert_eq!(z2.extract(), 2);
        let z3 = z2.move_right().unwrap();
        assert_eq!(z3.extract(), 3);
        assert!(z3.move_right().is_none());
    }

    #[test]
    fn test_zipper_move_left() {
        let z = Zipper::from_vec(vec![1, 2, 3]).unwrap();
        let z2 = z.move_right().unwrap().move_right().unwrap();
        let z3 = z2.move_left().unwrap();
        assert_eq!(z3.extract(), 2);
    }

    #[test]
    fn test_zipper_extend() {
        let z = Zipper::from_vec(vec![1, 2, 3, 4, 5]).unwrap();
        let doubled = z.extend(|z| z.extract() * 2);
        assert_eq!(doubled.to_vec(), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_moving_average() {
        let z = Zipper::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        let z = z.move_right().unwrap().move_right().unwrap(); // focus on 3
        let avg = moving_average(3)(&z);
        assert!((avg - 3.0).abs() < 0.01); // (2+3+4)/3 = 3
    }

    #[test]
    fn test_store_extract() {
        let store = Store::new(|x: i32| x * 2, 5);
        assert_eq!(store.extract(), 10);
    }

    #[test]
    fn test_store_peek_at() {
        let store = Store::new(|x: i32| x * 2, 5);
        assert_eq!(store.peek_at(10), 20);
    }

    #[test]
    fn test_store_seek() {
        let store = Store::new(|x: i32| x * 2, 5);
        let store2 = store.seek(7);
        assert_eq!(store2.extract(), 14);
    }
}
