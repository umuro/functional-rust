// format_args! for zero-alloc formatting in Rust
use std::fmt::{self, Write};

// Custom Write target (stack-allocated buffer)
struct StackBuf {
    buf: [u8; 256],
    len: usize,
}

impl StackBuf {
    fn new() -> Self { StackBuf { buf: [0; 256], len: 0 } }
    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buf[..self.len]).unwrap_or("")
    }
}

impl fmt::Write for StackBuf {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let new_len = self.len + bytes.len();
        if new_len > self.buf.len() { return Err(fmt::Error); }
        self.buf[self.len..new_len].copy_from_slice(bytes);
        self.len = new_len;
        Ok(())
    }
}

// Logger that formats without allocating a String
struct Logger { prefix: &'static str }

impl Logger {
    fn log(&self, args: fmt::Arguments) {
        // Write directly to stderr without String allocation
        let mut buf = StackBuf::new();
        write!(buf, "[{}] ", self.prefix).ok();
        buf.write_fmt(args).ok();
        eprintln!("{}", buf.as_str());
    }
}

macro_rules! log_info {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log(format_args!($($arg)*))
    };
}

// format_args! enables writing to multiple sinks
fn write_to_all(args: fmt::Arguments) {
    // Write to stdout
    println!("stdout: {}", args);
    // Write to a String buffer
    let mut s = String::new();
    write!(s, "{}", args).unwrap();
    println!("  (captured: {:?})", s);
    // Write to stack buffer
    let mut buf = StackBuf::new();
    buf.write_fmt(args).ok();
    println!("  (stack buf: '{}')", buf.as_str());
}

// Deferred formatting
fn log_if<'a>(condition: bool, msg: fmt::Arguments<'a>) {
    if condition {
        println!("Condition true: {}", msg);
    }
    // If false, the format is never evaluated
}

fn main() {
    let logger = Logger { prefix: "INFO" };

    // format_args! — no heap allocation
    log_info!(logger, "Server started on port {}", 8080);
    log_info!(logger, "Request from {} user agent: {}", "127.0.0.1", "curl/7.0");

    // Direct format_args usage
    let name = "World";
    let n = 42;
    write_to_all(format_args!("Hello, {}! Value = {}", name, n));

    // Lazy evaluation — format only happens if condition is true
    log_if(true, format_args!("Count: {}", 100));
    log_if(false, format_args!("Expensive: {}", (0..1000000).sum::<i64>())); // lazy!

    // Stack buffer — truly zero allocation
    let mut buf = StackBuf::new();
    write!(buf, "Pi = {:.6}", std::f64::consts::PI).unwrap();
    println!("Stack formatted: {}", buf.as_str());

    // Demonstrate: format! vs format_args!
    let _with_alloc: String = format!("Hello, {}!", name); // allocates
    // format_args!("Hello, {}!", name)                    // no allocation — lazy
    println!("format! allocates, format_args! doesn't!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_buf() {
        let mut buf = StackBuf::new();
        write!(buf, "Hello, {}!", "Rust").unwrap();
        assert_eq!(buf.as_str(), "Hello, Rust!");
    }

    #[test]
    fn test_format_args_zero_alloc() {
        let mut buf = StackBuf::new();
        let args = format_args!("value = {}", 42);
        buf.write_fmt(args).unwrap();
        assert_eq!(buf.as_str(), "value = 42");
    }
}
