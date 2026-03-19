#![allow(clippy::manual_is_multiple_of)]
#![allow(unused_variables)]
#![allow(clippy::match_like_matches)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::char_lit_as_u8)]
#![allow(clippy::while_let_loop)]
#![allow(clippy::manual_strip)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::io::{self, BufRead, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Offload blocking I/O to a thread (async-style)
fn read_string_async(content: String) -> impl FnOnce() -> String {
    move || {
        thread::sleep(Duration::from_millis(1)); // simulate I/O latency
        content
    }
}

fn spawn_io_task<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> mpsc::Receiver<T> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(f());
    });
    rx
}

fn process_text(text: &str) -> (usize, usize, usize) {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    (lines, words, chars)
}

fn write_to_buf(buf: &mut Vec<u8>, data: &[u8]) -> io::Result<usize> {
    buf.write(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process_text_counts() {
        let (l, w, c) = process_text("hello world\nfoo");
        assert_eq!(l, 2);
        assert_eq!(w, 3);
        assert_eq!(c, 15);
    }
    #[test]
    fn spawn_io_returns_value() {
        let rx = spawn_io_task(|| 42i32);
        assert_eq!(rx.recv().unwrap(), 42);
    }
    #[test]
    fn nonblocking_listener() {
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        l.set_nonblocking(true).unwrap();
        assert!(l.accept().is_err()); // WouldBlock or similar
    }
}
