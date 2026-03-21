📖 **[View on hightechmind.io →](https://hightechmind.io/rust/756-tempfile-testing)**

---

# 756-tempfile-testing — Tempfile Testing
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Code that reads or writes files cannot be tested with in-memory mocks alone — you need real filesystem semantics. Temporary directories solve this: they provide real file paths, get cleaned up on test completion, and are isolated per test process. Without proper cleanup, failing tests leave debris in `/tmp` that accumulates over time. The `Drop`-based `TempDir` type ensures cleanup even when tests panic.

## Learning Outcomes

- Implement a `TempDir` RAII type that creates a unique temporary directory on construction
- Use `Drop` to recursively delete the directory after the test completes
- Create and read files within the `TempDir` for realistic filesystem testing
- Generate unique directory names using PID + nanosecond timestamp to avoid collisions
- Test file-processing code (CSV reading, log rotation, config loading) against real files

## Rust Application

`TempDir::new(prefix)` creates a directory under `std::env::temp_dir()` with a unique name `{prefix}-{pid}-{nanos}`. `create_file` writes content and returns the path. `read_file` reads content back. `Drop` calls `fs::remove_dir_all` to clean up. Tests exercise writing multiple files, reading them back, verifying content, and checking that the directory is removed after the `TempDir` drops. A `process_files` integration test reads CSV-like data from real temp files.

## OCaml Approach

OCaml's `Filename.temp_dir` and `Filename.temp_file` create temporary files. Cleanup requires explicit `Sys.remove` or `FileUtil.rm` from the `fileutils` library. Jane Street's `Core.Unix` provides `with_temp_dir` as a bracket that guarantees cleanup even on exception. The `Bos` library wraps filesystem operations with typed paths and safer error handling.

## Key Differences

1. **Cleanup guarantee**: Rust's `TempDir::Drop` guarantees cleanup even on panic; OCaml requires explicit `try ... finally` or a `with_temp_dir` bracket function.
2. **Uniqueness**: Rust uses PID + nanoseconds; OCaml's `Filename.temp_file` uses a similar internal counter.
3. **Ecosystem**: Rust's `tempfile` crate is the standard — it uses OS-provided secure temp creation; OCaml uses `Filename.temp_dir` from stdlib.
4. **Parallel tests**: Rust's parallel tests each get their own `TempDir` with unique names; OCaml's sequential tests can reuse the same named temp dir safely.

## Exercises

1. Extend `TempDir` with a `create_subdir(name)` method that creates a subdirectory and returns its path, for testing code that reads from a directory tree.
2. Implement a `log_rotation_test` that creates 5 log files, runs a rotation function (rename oldest, create new), and verifies the final directory contains exactly 5 files with correct names.
3. Write a test for a config file parser that loads from a real `TempDir`/config.toml file, including testing error behavior when the file is missing or malformed.
