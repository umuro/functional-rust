#![allow(clippy::all)]
// Option::map is the functor operation for Option: transforms Some, passes None through.
pub fn double(opt: Option<i32>) -> Option<i32> {
    opt.map(|x| x * 2)
}

pub fn stringify(opt: Option<i32>) -> Option<String> {
    opt.map(|x| x.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_some() {
        assert_eq!(double(Some(21)), Some(42));
    }

    #[test]
    fn test_double_none() {
        assert_eq!(double(None), None);
    }

    #[test]
    fn test_stringify() {
        assert_eq!(stringify(Some(7)), Some("7".to_string()));
        assert_eq!(stringify(None), None);
    }

    #[test]
    fn test_functor_law_identity() {
        // map(id) == id
        let opt = Some(5);
        assert_eq!(opt.map(|x| x), opt);
    }

    #[test]
    fn test_functor_law_composition() {
        // map(f).map(g) == map(|x| g(f(x)))
        let opt = Some(3);
        let f = |x: i32| x + 1;
        let g = |x: i32| x * 2;
        assert_eq!(opt.map(f).map(g), opt.map(|x| g(f(x))));
    }
}
