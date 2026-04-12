📖 **[View on hightechmind.io →](https://hightechmind.io/rust/491-path-pathbuf)**

---

# Path and PathBuf
**Difficulty:** ⭐  
**Category:** Functional Programming  



`Path` and `PathBuf` are Rust's filesystem path types — analogous to `str`/`String` — providing OS-aware joining, extension extraction, parent navigation, and cross-platform normalisation.

## Problem Statement

File paths are not plain strings: `/` vs. `\` as separator, drive letters on Windows, UNC paths, relative vs. absolute, symlink resolution. Treating paths as `String` leads to bugs: manual `format!("{}/{}", dir, file)` fails on Windows, double-slash collisions, and no semantic operations like `parent()` or `extension()`. Rust provides `Path` (borrowed, like `&str`) and `PathBuf` (owned, like `String`) as first-class types that model the OS path semantics correctly on every platform.

## Learning Outcomes

- Build paths portably with `.join()` chaining
- Extract file extension with `.extension()` returning `Option<&OsStr>`
- Extract file stem (name without extension) with `.file_stem()`
- Navigate to parent directory with `.parent()` returning `Option<&Path>`
- Convert between `Path`/`PathBuf` and `&str`/`String` with `.to_str()` and `.to_path_buf()`

## Rust Application

`.join()` appends path components portably — on Windows it uses `\`, on Unix `/`:

```rust
let p = PathBuf::from("/a").join("b").join("c");
assert_eq!(p.to_str().unwrap(), "/a/b/c");
```

`.extension()` returns the part after the last `.` in the filename:

```rust
Path::new("f.txt").extension().unwrap()  // OsStr "txt"
Path::new("f.txt").file_stem().unwrap()  // OsStr "f"
```

`.parent()` returns the directory containing the path:

```rust
Path::new("/a/b/c").parent().unwrap()  // Path "/a/b"
```

## OCaml Approach

OCaml's standard library operates on plain strings for file paths. The `Filename` module provides portable operations:

```ocaml
Filename.concat "/a" "b"              (* "/a/b" *)
Filename.extension "f.txt"            (* ".txt" — includes the dot *)
Filename.chop_extension "f.txt"       (* "f" *)
Filename.dirname "/a/b/c"             (* "/a/b" *)
Filename.basename "/a/b/c"            (* "c" *)
```

OCaml's `Fpath` library (third-party) provides a type-safe path abstraction similar to Rust's `Path`/`PathBuf`.

## Key Differences

1. **Type safety**: Rust's `Path`/`PathBuf` are distinct types from `str`/`String`; OCaml's `Filename` functions accept and return plain `string`.
2. **Cross-platform separators**: Rust's `.join()` uses the OS separator automatically; OCaml's `Filename.concat` also handles this, but `^` concatenation does not.
3. **`OsStr` return types**: Rust's `extension()`/`file_stem()` return `Option<&OsStr>`, handling non-UTF-8 filenames; OCaml returns `string` (bytes, no encoding guarantee).
4. **`Deref` coercion**: `PathBuf` derefs to `Path`, enabling code that accepts `&Path` to work with `PathBuf` references; OCaml has no such coercion.

## Exercises

1. **Recursive directory scanner**: Write `fn list_rust_files(dir: &Path) -> Vec<PathBuf>` using `std::fs::read_dir` that returns all `.rs` files recursively.
2. **Path normalisation**: Write `normalize(p: &Path) -> PathBuf` that resolves `..` components without hitting the filesystem (lexical normalisation only).
3. **Cross-platform paths**: Write a test that constructs the same logical path using `.join()` and verify it serialises correctly on both Unix and Windows using `cfg!(target_os = "windows")`.
