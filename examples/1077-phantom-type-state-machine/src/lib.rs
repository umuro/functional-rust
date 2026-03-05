//! Phantom Type State Machine — File Handle
//!
//! Uses phantom types to enforce state transitions at compile time.
//! In OCaml, phantom type parameters constrain which operations are valid.
//! In Rust, we use the same pattern with zero-sized type markers.

use std::marker::PhantomData;

// ── Solution 1: Idiomatic Rust — phantom type markers ──

/// State marker: file is open (zero-sized, exists only at type level)
pub struct Opened;
/// State marker: file is closed
pub struct Closed;

/// A file handle parameterized by its state.
/// The `PhantomData<State>` makes the compiler track the state
/// without any runtime cost.
///
/// OCaml equivalent: `type 'state handle = { name: string; content: string list }`
pub struct FileHandle<State> {
    name: String,
    content: Vec<String>,
    _state: PhantomData<State>,
}

/// Open a file — returns a handle in the `Opened` state.
/// OCaml: `val open_file : string -> opened handle`
pub fn open_file(name: &str) -> FileHandle<Opened> {
    FileHandle {
        name: name.to_string(),
        content: vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ],
        _state: PhantomData,
    }
}

impl FileHandle<Opened> {
    /// Read a line — only available when the file is open.
    /// OCaml: `val read_line : opened handle -> int -> string`
    pub fn read_line(&self, n: usize) -> Option<&str> {
        self.content.get(n).map(|s| s.as_str())
    }

    /// Close the file — consumes the open handle, returns a closed one.
    /// This is the key insight: after closing, the old handle is gone.
    /// OCaml: `val close_file : opened handle -> closed handle`
    pub fn close(self) -> FileHandle<Closed> {
        FileHandle {
            name: self.name,
            content: vec![],
            _state: PhantomData,
        }
    }

    /// Get the file name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl FileHandle<Closed> {
    /// Get the file name even after closing
    pub fn name(&self) -> &str {
        &self.name
    }
}

// ── Solution 2: Trait-based approach ──
//
// Uses traits to gate operations instead of inherent impls.

/// Marker trait for states that allow reading
pub trait Readable {}
impl Readable for Opened {}
// Closed does NOT implement Readable

/// Generic read function — only compiles for Readable states
pub fn read_generic<S: Readable>(handle: &FileHandle<S>, n: usize) -> Option<&str> {
    handle.content.get(n).map(|s| s.as_str())
}

// ── Solution 3: Enum-based (runtime check, for comparison) ──
//
// Shows why phantom types are superior — enum checks happen at runtime.

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

pub struct RuntimeFileHandle {
    pub name: String,
    pub content: Vec<String>,
    pub state: FileState,
}

impl RuntimeFileHandle {
    pub fn open(name: &str) -> Self {
        Self {
            name: name.to_string(),
            content: vec![
                "line1".to_string(),
                "line2".to_string(),
                "line3".to_string(),
            ],
            state: FileState::Open,
        }
    }

    /// Returns Err if file is closed — runtime check instead of compile-time
    pub fn read_line(&self, n: usize) -> Result<&str, &'static str> {
        if self.state == FileState::Closed {
            return Err("cannot read from closed file");
        }
        self.content
            .get(n)
            .map(|s| s.as_str())
            .ok_or("line index out of range")
    }

    pub fn close(&mut self) {
        self.state = FileState::Closed;
        self.content.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_and_read() {
        let f = open_file("data.txt");
        assert_eq!(f.read_line(0), Some("line1"));
        assert_eq!(f.read_line(1), Some("line2"));
        assert_eq!(f.read_line(2), Some("line3"));
    }

    #[test]
    fn test_read_out_of_bounds() {
        let f = open_file("data.txt");
        assert_eq!(f.read_line(99), None);
    }

    #[test]
    fn test_close_returns_closed_handle() {
        let f = open_file("data.txt");
        let closed = f.close();
        // After closing, we can still get the name
        assert_eq!(closed.name(), "data.txt");
        // But we CANNOT call read_line — it won't compile:
        // closed.read_line(0);  // ERROR: no method `read_line` on FileHandle<Closed>
    }

    #[test]
    fn test_generic_read_on_opened() {
        let f = open_file("test.txt");
        assert_eq!(read_generic(&f, 0), Some("line1"));
    }

    #[test]
    fn test_runtime_handle_read_after_close() {
        let mut f = RuntimeFileHandle::open("data.txt");
        assert_eq!(f.read_line(0), Ok("line1"));
        f.close();
        assert_eq!(f.read_line(0), Err("cannot read from closed file"));
    }

    #[test]
    fn test_file_name_persists_after_close() {
        let f = open_file("important.txt");
        assert_eq!(f.name(), "important.txt");
        let closed = f.close();
        assert_eq!(closed.name(), "important.txt");
    }
}
