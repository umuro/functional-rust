#![allow(clippy::all)]
//! # Natural Transformations
//! Transform between functors while preserving structure.

pub fn option_to_vec<A>(opt: Option<A>) -> Vec<A> {
    opt.into_iter().collect()
}

pub fn vec_to_option<A>(v: Vec<A>) -> Option<A> {
    v.into_iter().next()
}

pub fn result_to_option<A, E>(r: Result<A, E>) -> Option<A> {
    r.ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_option_to_vec() {
        assert_eq!(option_to_vec(Some(1)), vec![1]);
        assert_eq!(option_to_vec(None::<i32>), vec![]);
    }
}
