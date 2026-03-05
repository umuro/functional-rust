// Drop and RAII patterns in Rust

struct FileHandle {
    name: String,
    is_open: bool,
}

impl FileHandle {
    fn open(name: &str) -> Self {
        println!("Opening: {}", name);
        FileHandle { name: name.to_string(), is_open: true }
    }

    fn read(&self) -> Option<String> {
        if self.is_open {
            Some(format!("Contents of {}", self.name))
        } else {
            None
        }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        if self.is_open {
            println!("Closing: {}", self.name);
            self.is_open = false;
        }
    }
}

// RAII guard pattern
struct LockGuard<'a> {
    resource: &'a str,
    lock_id: u32,
}

impl<'a> LockGuard<'a> {
    fn acquire(resource: &'a str) -> Self {
        let id = 42; // simulated lock ID
        println!("Acquired lock #{} on '{}'", id, resource);
        LockGuard { resource, lock_id: id }
    }
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        println!("Released lock #{} on '{}'", self.lock_id, self.resource);
    }
}

// Timer for profiling
use std::time::Instant;
struct Timer { name: String, start: Instant }

impl Timer {
    fn new(name: &str) -> Self {
        println!("Starting timer: {}", name);
        Timer { name: name.to_string(), start: Instant::now() }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("Timer '{}' elapsed: {:?}", self.name, self.start.elapsed());
    }
}

fn main() {
    println!("=== FileHandle RAII ===");
    {
        let f = FileHandle::open("data.txt");
        println!("{:?}", f.read());
    } // Drop called here — file closed

    println!("\n=== Drop order ===");
    {
        let _a = FileHandle::open("a.txt");
        let _b = FileHandle::open("b.txt");
        let _c = FileHandle::open("c.txt");
    } // Drops in reverse: c, b, a

    println!("\n=== Lock guard ===");
    {
        let _guard = LockGuard::acquire("database");
        println!("Working with database...");
    } // guard dropped, lock released

    println!("\n=== Explicit early drop ===");
    let f2 = FileHandle::open("early.txt");
    println!("{:?}", f2.read());
    std::mem::drop(f2); // explicitly drop before end of scope
    println!("After explicit drop");

    println!("\n=== Timer RAII ===");
    {
        let _t = Timer::new("computation");
        let _sum: u64 = (1..=1000).sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_handle() {
        let f = FileHandle::open("test.txt");
        assert!(f.is_open);
        assert!(f.read().is_some());
    }

    #[test]
    fn test_explicit_drop() {
        let f = FileHandle::open("drop_test.txt");
        let name = f.name.clone();
        std::mem::drop(f);
        // After drop, we can't access f anymore
        assert_eq!(name, "drop_test.txt");
    }
}
