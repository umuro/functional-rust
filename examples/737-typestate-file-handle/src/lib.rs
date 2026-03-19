#![allow(clippy::all)]
use std::io::{self, Read, Write};
/// 737: File Handle Typestate — Open / Closed / ReadOnly
/// Demonstrates compile-time permission encoding for file handles.
use std::marker::PhantomData;

// ── Permission markers ────────────────────────────────────────────────────────

pub struct Closed;
pub struct ReadWrite;
pub struct ReadOnly;

// ── File Handle ────────────────────────────────────────────────────────────────

pub struct FileHandle<Mode> {
    path: String,
    content: Vec<u8>, // simulates in-memory file for this example
    pos: usize,
    _mode: PhantomData<Mode>,
}

impl FileHandle<Closed> {
    /// Create a new closed handle.
    pub fn new(path: impl Into<String>) -> Self {
        FileHandle {
            path: path.into(),
            content: Vec::new(),
            pos: 0,
            _mode: PhantomData,
        }
    }

    /// Open for reading and writing.
    pub fn open_rw(self) -> io::Result<FileHandle<ReadWrite>> {
        println!("Opening '{}' read-write", self.path);
        Ok(FileHandle {
            path: self.path,
            content: self.content,
            pos: 0,
            _mode: PhantomData,
        })
    }

    /// Open existing content as read-only.
    pub fn open_ro(self, initial: Vec<u8>) -> io::Result<FileHandle<ReadOnly>> {
        println!("Opening '{}' read-only", self.path);
        Ok(FileHandle {
            path: self.path,
            content: initial,
            pos: 0,
            _mode: PhantomData,
        })
    }
}

impl FileHandle<ReadWrite> {
    pub fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
        self.content.extend_from_slice(data);
        println!("Wrote {} bytes to '{}'", data.len(), self.path);
        Ok(())
    }

    pub fn read_to_string(&mut self) -> io::Result<String> {
        let s = String::from_utf8_lossy(&self.content[self.pos..]).into_owned();
        self.pos = self.content.len();
        Ok(s)
    }

    /// Downgrade to read-only (cannot write anymore)
    pub fn into_readonly(self) -> FileHandle<ReadOnly> {
        println!("Downgrading '{}' to read-only", self.path);
        FileHandle {
            path: self.path,
            content: self.content,
            pos: self.pos,
            _mode: PhantomData,
        }
    }

    /// Close the handle — transitions to Closed.
    pub fn close(self) -> FileHandle<Closed> {
        println!("Closing '{}'", self.path);
        FileHandle {
            path: self.path,
            content: Vec::new(),
            pos: 0,
            _mode: PhantomData,
        }
    }
}

impl FileHandle<ReadOnly> {
    pub fn read_to_string(&mut self) -> io::Result<String> {
        let s = String::from_utf8_lossy(&self.content[self.pos..]).into_owned();
        self.pos = self.content.len();
        Ok(s)
    }

    pub fn close(self) -> FileHandle<Closed> {
        println!("Closing '{}' (read-only)", self.path);
        FileHandle {
            path: self.path,
            content: Vec::new(),
            pos: 0,
            _mode: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_then_read() {
        let handle = FileHandle::<Closed>::new("test.txt");
        let mut rw = handle.open_rw().unwrap();
        rw.write_all(b"hello world").unwrap();
        let s = rw.read_to_string().unwrap();
        assert_eq!(s, "hello world");
        rw.close();
    }

    #[test]
    fn downgrade_to_readonly() {
        let handle = FileHandle::<Closed>::new("test.txt");
        let mut rw = handle.open_rw().unwrap();
        rw.write_all(b"data").unwrap();
        let mut ro = rw.into_readonly();
        let s = ro.read_to_string().unwrap();
        assert_eq!(s, "data");
        ro.close();
    }

    #[test]
    fn open_ro_with_initial_content() {
        let handle = FileHandle::<Closed>::new("test.txt");
        let mut ro = handle.open_ro(b"preloaded".to_vec()).unwrap();
        let s = ro.read_to_string().unwrap();
        assert_eq!(s, "preloaded");
        ro.close();
    }
}
