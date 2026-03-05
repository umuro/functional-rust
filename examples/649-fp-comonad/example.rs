// Comonad in Rust - Zipper example

#[derive(Debug, Clone)]
struct Zipper<T> {
    left: Vec<T>,
    focus: T,
    right: Vec<T>,
}

impl<T: Clone> Zipper<T> {
    fn extract(&self) -> T { self.focus.clone() }
    
    fn move_left(&self) -> Option<Self> {
        let mut left = self.left.clone();
        left.pop().map(|f| {
            let mut right = vec![self.focus.clone()];
            right.extend(self.right.clone());
            Zipper { left, focus: f, right }
        })
    }
    
    fn move_right(&self) -> Option<Self> {
        if self.right.is_empty() { return None; }
        let mut right = self.right.clone();
        let focus = right.remove(0);
        let mut left = self.left.clone();
        left.push(self.focus.clone());
        Some(Zipper { left, focus, right })
    }
    
    fn extend<F, B: Clone>(&self, f: F) -> Zipper<B>
    where F: Fn(&Zipper<T>) -> B {
        // Apply f at every position
        let focus = f(self);
        Zipper { left: vec![], focus, right: vec![] }
    }
}

fn main() {
    let z = Zipper { left: vec![1, 2], focus: 3, right: vec![4, 5] };
    println!("Focus: {}", z.extract());
    
    if let Some(right) = z.move_right() {
        println!("After move right: {}", right.extract());
    }
}
