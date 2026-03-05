//! # Tempfile Testing
//!
//! Testing with temporary files and directories.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// A temporary directory that cleans up on drop
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    /// Create a new temporary directory
    pub fn new(prefix: &str) -> std::io::Result<Self> {
        let path = std::env::temp_dir().join(format!(
            "{}-{}-{}",
            prefix,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&path)?;
        Ok(TempDir { path })
    }

    /// Get the path to the temp directory
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Create a file in the temp directory
    pub fn create_file(&self, name: &str, content: &str) -> std::io::Result<PathBuf> {
        let file_path = self.path.join(name);
        let mut file = File::create(&file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(file_path)
    }

    /// Read a file from the temp directory
    pub fn read_file(&self, name: &str) -> std::io::Result<String> {
        let file_path = self.path.join(name);
        let mut content = String::new();
        File::open(&file_path)?.read_to_string(&mut content)?;
        Ok(content)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Process a config file
pub fn process_config(path: &Path) -> std::io::Result<Vec<(String, String)>> {
    let content = fs::read_to_string(path)?;
    let mut pairs = Vec::new();
    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            pairs.push((key.trim().to_string(), value.trim().to_string()));
        }
    }
    Ok(pairs)
}

/// Write a config file
pub fn write_config(path: &Path, pairs: &[(String, String)]) -> std::io::Result<()> {
    let content: String = pairs
        .iter()
        .map(|(k, v)| format!("{}={}\n", k, v))
        .collect();
    fs::write(path, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_dir_created() {
        let temp = TempDir::new("test").unwrap();
        assert!(temp.path().exists());
    }

    #[test]
    fn test_temp_dir_cleanup() {
        let path = {
            let temp = TempDir::new("cleanup").unwrap();
            temp.path().to_path_buf()
        };
        // After drop, directory should be gone
        assert!(!path.exists());
    }

    #[test]
    fn test_create_and_read_file() {
        let temp = TempDir::new("rw").unwrap();
        temp.create_file("test.txt", "Hello, World!").unwrap();
        let content = temp.read_file("test.txt").unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[test]
    fn test_process_config() {
        let temp = TempDir::new("config").unwrap();
        let path = temp.create_file("config.ini", "key1=value1\nkey2=value2\n").unwrap();
        let pairs = process_config(&path).unwrap();
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0], ("key1".to_string(), "value1".to_string()));
    }

    #[test]
    fn test_write_config() {
        let temp = TempDir::new("write").unwrap();
        let path = temp.path().join("out.ini");
        let pairs = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
        ];
        write_config(&path, &pairs).unwrap();
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("a=1"));
        assert!(content.contains("b=2"));
    }

    #[test]
    fn test_roundtrip() {
        let temp = TempDir::new("roundtrip").unwrap();
        let path = temp.path().join("config.ini");
        let original = vec![
            ("host".to_string(), "localhost".to_string()),
            ("port".to_string(), "8080".to_string()),
        ];
        write_config(&path, &original).unwrap();
        let loaded = process_config(&path).unwrap();
        assert_eq!(original, loaded);
    }
}
