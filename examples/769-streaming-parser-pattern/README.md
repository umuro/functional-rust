📖 **[View on hightechmind.io →](https://hightechmind.io/rust/769-streaming-parser-pattern)**

---

# 769: Streaming Parser for Large Inputs

**Difficulty:** 4  **Level:** Advanced

Parse a large input incrementally as an `Iterator`, processing one record at a time without loading the whole dataset into memory.

## The Problem This Solves

Loading a 10GB log file into a `Vec<Record>` before processing any of it is often not an option — it may exceed available memory, or you may only need a handful of records that match a filter. Streaming parsers solve this by yielding one parsed record at a time, letting downstream code decide whether to collect, filter, aggregate, or early-exit.

Beyond memory, streaming composition is compositional: `stream.filter(...).map(...).take(100)` stops parsing at 100 records — no wasted work. This is the same design as `BufRead`-based line iteration in the standard library, and the foundation of streaming data pipelines in production Rust code.

## The Intuition

Make the parser implement `Iterator` directly. Each call to `next()` reads the next non-empty line from the underlying `BufRead`, tries to parse it, and returns `Some(Ok(record))`, `Some(Err(error))`, or `None` (EOF). The caller never sees a `Vec<Record>` — they get a lazy sequence.

The reusable `line_buf: String` is cleared and refilled each call, so only one line is in memory at a time regardless of input size.

## How It Works in Rust

**The streaming struct** — wraps any `BufRead`:
```rust
pub struct RecordStream<R: BufRead> {
    reader:      R,
    line_buf:    String,  // reused across calls
    line_num:    usize,
    skip_errors: bool,
}
```
Generic over `R: BufRead` — works with files, network sockets, in-memory `Cursor`, or anything else.

**`Iterator` implementation** — one line per `next()` call:
```rust
impl<R: BufRead> Iterator for RecordStream<R> {
    type Item = Result<Record, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.line_buf.clear();
            match self.reader.read_line(&mut self.line_buf) {
                Ok(0)  => return None,  // EOF
                Ok(_)  => {}
                Err(e) => return Some(Err(ParseError { line: self.line_num, message: e.to_string() })),
            }
            self.line_num += 1;
            let trimmed = self.line_buf.trim();
            if trimmed.is_empty() { continue; }  // skip blank lines

            match Self::parse_line(trimmed) {
                Some(rec) => return Some(Ok(rec)),
                None if self.skip_errors => continue,
                None => return Some(Err(ParseError { line: self.line_num, message: format!("bad line: {trimmed}") })),
            }
        }
    }
}
```
The `loop` with `continue` skips blank lines and (optionally) bad lines without returning `None` prematurely.

**Composition with standard iterator adaptors:**
```rust
// Collect only valid records
let valid: Vec<Record> = stream_str(data)
    .skip_errors()
    .filter_map(|r| r.ok())
    .collect();

// Aggregate without collecting
let total: f64 = stream_str(data)
    .skip_errors()
    .filter_map(|r| r.ok())
    .map(|r| r.value)
    .sum();

// Early exit after first 10 high-value records
let top10: Vec<Record> = stream_str(data)
    .skip_errors()
    .filter_map(|r| r.ok())
    .filter(|r| r.value >= 90.0)
    .take(10)
    .collect();
```
These chains parse only as many lines as needed — `take(10)` stops the iterator after 10 matches.

**Builder pattern for configuration:**
```rust
pub fn skip_errors(mut self) -> Self {
    self.skip_errors = true;
    self
}
```
Fluent configuration before iteration begins.

## What This Unlocks

- **Constant-memory processing** — parse a file of any size with O(1) memory per record.
- **Lazy evaluation** — combine `filter`, `map`, `take` without parsing the whole file.
- **Error handling per record** — `Result<Record, ParseError>` lets callers decide: collect errors, skip them, or abort.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lazy sequence | `Seq.t` / `Stream.t` | `impl Iterator<Item = ...>` |
| Line-by-line reading | `input_line` in loop | `BufRead::read_line` into reusable `String` |
| Skip and continue | `Seq.filter` or `try_with` | `continue` inside `Iterator::next` loop |
| Error per item | `result` in sequence | `Item = Result<Record, ParseError>` |
