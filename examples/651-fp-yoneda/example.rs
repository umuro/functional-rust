// Yoneda Lemma in Rust - Map Fusion

struct YonedaVec<A> {
    run: Box<dyn FnOnce() -> Vec<A>>,
}

impl<A: 'static> YonedaVec<A> {
    fn lift(vec: Vec<A>) -> Self {
        YonedaVec { run: Box::new(move || vec) }
    }
    
    fn lower(self) -> Vec<A> {
        (self.run)()
    }
    
    fn map<B: 'static, F: FnOnce(A) -> B + 'static>(self, f: F) -> YonedaVec<B> {
        let run = self.run;
        YonedaVec { run: Box::new(move || run().into_iter().map(f).collect()) }
    }
}

fn main() {
    // Without fusion: 3 separate traversals
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5]
        .iter().map(|x| x + 1).collect::<Vec<_>>()
        .iter().map(|x| x * 2).collect::<Vec<_>>()
        .iter().map(|x| x - 1).collect();
    
    // With Yoneda: single fused traversal
    let v2 = YonedaVec::lift(vec![1, 2, 3, 4, 5])
        .map(|x| x + 1)
        .map(|x| x * 2)
        .map(|x| x - 1)
        .lower();
    
    println!("Result: {:?}", v2);
}
