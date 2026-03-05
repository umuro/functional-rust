/// 756: Testing with Temporary Files and Directories
/// RAII TempDir: auto-cleanup even on panic.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

// ── TempDir: RAII temp directory ──────────────────────────────────────────────

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    /// Create a new unique temp directory.
    pub fn new() -> io::Result<Self> {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let name = format!("rust_test_{}_{}", std::process::id(), id);
        let path = std::env::temp_dir().join(name);
        fs::create_dir_all(&path)?;
        Ok(TempDir { path })
    }

    /// The path to this temp directory.
    pub fn path(&self) -> &Path { &self.path }

    /// Create a file inside this temp dir.
    pub fn child(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }

    /// Create a subdirectory inside this temp dir.
    pub fn subdir(&self, name: &str) -> io::Result<PathBuf> {
        let p = self.path.join(name);
        fs::create_dir_all(&p)?;
        Ok(p)
    }
}

impl Drop for TempDir {
    /// Cleanup runs automatically — even on panic.
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

// ── Functions under test ───────────────────────────────────────────────────────

pub fn write_lines(path: &Path, lines: &[&str]) -> io::Result<()> {
    let content = lines.join("\n");
    fs::write(path, content)
}

pub fn count_lines(path: &Path) -> io::Result<usize> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().count())
}

pub fn append_line(path: &Path, line: &str) -> io::Result<()> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(f, "{}", line)
}

pub fn copy_file(src: &Path, dst: &Path) -> io::Result<u64> {
    fs::copy(src, dst)
}

fn main() {
    let dir = TempDir::new().expect("failed to create temp dir");
    println!("Temp dir: {}", dir.path().display());

    let file = dir.child("hello.txt");
    write_lines(&file, &["Hello", "World"]).unwrap();
    println!("Lines: {}", count_lines(&file).unwrap());

    append_line(&file, "Goodbye").unwrap();
    println!("After append: {}", count_lines(&file).unwrap());

    println!("TempDir will be cleaned up when dir goes out of scope.");
    // dir dropped here → cleanup
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_dir_is_created() {
        let dir = TempDir::new().unwrap();
        assert!(dir.path().exists(), "temp dir should exist");
        assert!(dir.path().is_dir(), "should be a directory");
    }

    #[test]
    fn temp_dir_cleaned_up_on_drop() {
        let path = {
            let dir = TempDir::new().unwrap();
            let p = dir.path().to_path_buf();
            assert!(p.exists());
            p   // dir dropped here
        };
        assert!(!path.exists(), "temp dir should be removed after drop");
    }

    #[test]
    fn write_and_read_file() {
        let dir = TempDir::new().unwrap();
        let file = dir.child("data.txt");

        write_lines(&file, &["line1", "line2", "line3"]).unwrap();

        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains("line1"));
        assert!(content.contains("line3"));
    }

    #[test]
    fn count_lines_correct() {
        let dir = TempDir::new().unwrap();
        let file = dir.child("lines.txt");
        write_lines(&file, &["a", "b", "c", "d"]).unwrap();
        assert_eq!(count_lines(&file).unwrap(), 4);
    }

    #[test]
    fn append_increases_line_count() {
        let dir = TempDir::new().unwrap();
        let file = dir.child("append.txt");
        write_lines(&file, &["first"]).unwrap();
        append_line(&file, "second").unwrap();
        append_line(&file, "third").unwrap();
        assert_eq!(count_lines(&file).unwrap(), 3);
    }

    #[test]
    fn copy_file_creates_duplicate() {
        let dir = TempDir::new().unwrap();
        let src = dir.child("src.txt");
        let dst = dir.child("dst.txt");

        fs::write(&src, "content").unwrap();
        copy_file(&src, &dst).unwrap();

        assert!(dst.exists());
        assert_eq!(
            fs::read_to_string(&src).unwrap(),
            fs::read_to_string(&dst).unwrap()
        );
    }

    #[test]
    fn subdir_is_created() {
        let dir = TempDir::new().unwrap();
        let sub = dir.subdir("nested/deep").unwrap();
        assert!(sub.exists());
        assert!(sub.is_dir());
    }

    #[test]
    fn each_test_has_isolated_temp_dir() {
        let d1 = TempDir::new().unwrap();
        let d2 = TempDir::new().unwrap();
        assert_ne!(d1.path(), d2.path(), "each TempDir should be unique");
    }
}
