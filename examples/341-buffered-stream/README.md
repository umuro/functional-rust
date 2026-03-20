📖 **[View on hightechmind.io →](https://hightechmind.io/rust/341-buffered-stream)**

---

# 341: Buffered Stream — BufReader and BufWriter
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Reading or writing one byte at a time with unbuffered I/O makes a system call for each operation — catastrophically slow for large files. `BufReader` and `BufWriter` add an in-memory buffer: reads fill the buffer in bulk (e.g., 8KB), and subsequent reads serve from the buffer without syscalls. Writers accumulate data in the buffer and flush in bulk. This optimization, crucial for text file processing and log writing, reduces system call overhead by orders of magnitude.

## Learning Outcomes

- Use `BufReader::new(reader)` to wrap any reader with a 8KB internal buffer
- Use `BufWriter::new(writer)` to buffer writes and flush in bulk
- Process files line-by-line using `BufRead::lines()` — lazy, buffered
- Understand that flushing on `BufWriter` drop may silently discard errors — call `flush()` explicitly

## Rust Application

`BufReader` wraps any `Read` type; `lines()` iterates lazily:

```rust
use std::io::{BufRead, BufReader, BufWriter, Write};

fn read_lines(input: &[u8]) -> Vec<String> {
    let reader = BufReader::new(input);
    reader.lines().filter_map(|l| l.ok()).collect()
}

fn write_lines(lines: &[&str]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut writer = BufWriter::new(&mut output);
    for line in lines {
        writeln!(writer, "{}", line).unwrap();
    }
    writer.flush().unwrap();  // Critical: flush explicitly
    output
}
```

## OCaml Approach

OCaml's `In_channel` uses buffered I/O by default. `In_channel.input_line` is the standard line-by-line reader:

```ocaml
let count_lines path =
  let ic = In_channel.open_text path in
  let count = ref 0 in
  (try while true do ignore (In_channel.input_line_exn ic); incr count done
   with End_of_file -> ());
  In_channel.close ic; !count
```

## Key Differences

1. **Default buffering**: OCaml's `In_channel` / `Out_channel` are buffered by default; Rust's `File` is unbuffered — `BufReader`/`BufWriter` must be added explicitly.
2. **Drop flush**: `BufWriter`'s `Drop` implementation calls `flush()`, but ignores errors — always call `flush()` explicitly when error handling matters.
3. **Buffer size**: Default buffer is 8KB; use `BufReader::with_capacity(size)` for tuned buffer sizes.
4. **Async buffered I/O**: `tokio::io::BufReader` / `BufWriter` are the async-aware equivalents for use with Tokio async I/O traits.

## Exercises

1. Benchmark reading a large file byte-by-byte vs line-by-line with `BufReader` — measure time and system call count.
2. Implement a log file writer that uses `BufWriter` with periodic explicit flushes every 1000 lines.
3. Use `BufReader::lines()` to process a CSV file lazily, parsing each line into a `Vec<String>` of fields.
