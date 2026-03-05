//! # Traversal Optics
//! Focus on multiple elements.

pub fn traverse_vec<A: Clone, B>(xs: &[A], f: impl Fn(&A) -> Option<B>) -> Option<Vec<B>> {
    xs.iter().map(f).collect()
}

pub fn over_all<A: Clone>(xs: &mut [A], f: impl Fn(&A) -> A) {
    for x in xs.iter_mut() { *x = f(x); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_traverse() {
        let xs = vec![1, 2, 3];
        let result = traverse_vec(&xs, |x| Some(x * 2));
        assert_eq!(result, Some(vec![2, 4, 6]));
    }
}
