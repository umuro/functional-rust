# 491: Path and PathBuf Handling

**Difficulty:** 1  **Level:** Beginner

Platform-aware filesystem paths — like `String`/`&str` but for paths, with OS separator handling.

## The Problem This Solves

In Python and JavaScript, file paths are plain strings. `os.path.join("/home/user", "file.txt")` works on Unix but breaks on Windows if you manually concatenate with `/`. The language doesn't enforce the distinction between "a string" and "a path."

Rust has dedicated types: `Path` (borrowed, like `&str`) and `PathBuf` (owned, like `String`). They know about platform path separators — `/` on Unix, `\` on Windows. Calling `.join()`, `.parent()`, `.file_name()`, `.extension()` — these work correctly on all platforms without you thinking about it.

More importantly, using `Path`/`PathBuf` in function signatures makes your API self-documenting. When a function takes `&Path`, it's clear it expects a filesystem path, not an arbitrary string. You also get protection from path traversal issues — constructing paths with `.join()` won't accidentally interpret `..` as you'd expect with string concatenation, and you can check `.is_absolute()`.

## The Intuition

`Path` is to `PathBuf` as `&str` is to `String`. `Path` is a borrowed reference to path data — you pass it around cheaply. `PathBuf` is the owned, growable version you can build up incrementally.

The mental model:
- Start with `PathBuf::from("/base")` when you own a path
- Use `path.push("component")` to append segments
- Use `path.join("component")` to get a new path without modifying the original
- Accept `&Path` in function parameters (most flexible — both `Path` and `PathBuf` coerce to it)

OCaml uses plain strings for paths. The Python equivalent is `pathlib.Path` — `Path` in Python is roughly `PathBuf` in Rust (it's the owned, mutable version).

## How It Works in Rust

```rust
use std::path::{Path, PathBuf};

// PathBuf — owned, growable
let mut p = PathBuf::from("/home");
p.push("user");        // /home/user
p.push("docs");        // /home/user/docs
p.push("file.txt");    // /home/user/docs/file.txt

// Path — borrowed view, decompose it
let path = Path::new("/home/user/docs/file.txt");
path.parent()       // Some("/home/user/docs")
path.file_name()    // Some("file.txt")
path.file_stem()    // Some("file")     ← without extension
path.extension()    // Some("txt")
path.is_absolute()  // true

// Components — iterate path segments
for component in path.components() {
    println!("{:?}", component);
}
// RootDir, Normal("home"), Normal("user"), Normal("docs"), Normal("file.txt")

// join — builds new PathBuf (non-mutating)
let base = Path::new("/home/user");
let full = base.join("projects").join("rust").join("main.rs");
// /home/user/projects/rust/main.rs

// with_extension — change or add extension
Path::new("report.txt").with_extension("pdf");  // report.pdf
Path::new("archive").with_extension("tar.gz");  // archive.tar.gz

// starts_with / ends_with — path prefix/suffix check
path.starts_with("/home")  // true
path.ends_with("file.txt") // true

// Accept &Path in functions — both Path and PathBuf coerce
fn print_ext(p: &Path) {
    if let Some(ext) = p.extension() {
        println!("{}", ext.to_string_lossy());
    }
}
print_ext(path);           // works with &Path
print_ext(&p);             // works with &PathBuf too (auto-coerce)
```

## What This Unlocks

- **Cross-platform path handling** — `.join()` uses the right separator automatically on all platforms.
- **Safe path decomposition** — extract filename, stem, extension, parent without string-splitting manually.
- **Type-safe APIs** — functions taking `&Path` can't accidentally receive a non-path string.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Path type | `string` (no dedicated type) | `Path` (borrowed) / `PathBuf` (owned) |
| Join path segments | `String.concat "/" parts` | `path.join("segment")` |
| Get filename | Manual `String.rindex_opt` | `path.file_name()` → `Option<&OsStr>` |
| Get extension | Manual `String.rindex_opt '.'` | `path.extension()` → `Option<&OsStr>` |
| Get parent dir | Manual split | `path.parent()` → `Option<&Path>` |
| Platform separators | Manual | Automatic — `/` on Unix, `\` on Windows |
| Append segment | String concat | `pathbuf.push("segment")` (mutates) |
