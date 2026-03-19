// 476. split(), splitn(), split_once()

#[cfg(test)]
mod tests {
    #[test]
    fn test_split() {
        assert_eq!("a,b,c".split(',').collect::<Vec<_>>(), ["a", "b", "c"]);
    }
    #[test]
    fn test_splitn() {
        let v: Vec<_> = "a:b:c:d".splitn(3, ':').collect();
        assert_eq!(v, ["a", "b", "c:d"]);
    }
    #[test]
    fn test_split_once() {
        assert_eq!("k=v".split_once('='), Some(("k", "v")));
        assert_eq!("noeq".split_once('='), None);
    }
    #[test]
    fn test_whitespace() {
        let w: Vec<_> = "  a  b  c  ".split_whitespace().collect();
        assert_eq!(w, ["a", "b", "c"]);
    }
}
