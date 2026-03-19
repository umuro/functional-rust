// 491. Path and PathBuf handling
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_join() {
        let p = PathBuf::from("/a").join("b").join("c");
        assert_eq!(p.to_str().unwrap(), "/a/b/c");
    }
    #[test]
    fn test_ext() {
        assert_eq!(Path::new("f.txt").extension().unwrap(), "txt");
    }
    #[test]
    fn test_stem() {
        assert_eq!(Path::new("f.txt").file_stem().unwrap(), "f");
    }
    #[test]
    fn test_parent() {
        assert_eq!(Path::new("/a/b/c").parent().unwrap(), Path::new("/a/b"));
    }
}
