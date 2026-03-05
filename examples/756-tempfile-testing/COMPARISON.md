# OCaml vs Rust: Tempfile Testing

## RAII Temp Directory

### Rust
```rust
pub struct TempDir {
    path: PathBuf,
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
```

### OCaml
```ocaml
let with_temp_dir f =
  let dir = Filename.temp_dir "test" "" in
  Fun.protect ~finally:(fun () -> 
    Sys.command (Printf.sprintf "rm -rf %s" dir) |> ignore
  ) (fun () -> f dir)
```

## Creating Test Files

### Rust
```rust
let temp = TempDir::new("test").unwrap();
let path = temp.create_file("config.ini", "key=value\n").unwrap();
```

### OCaml
```ocaml
let () = with_temp_dir (fun dir ->
  let path = Filename.concat dir "config.ini" in
  Out_channel.write_all path ~data:"key=value\n"
)
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Cleanup | `Fun.protect ~finally` | `Drop` trait |
| Temp path | `Filename.temp_dir` | `std::env::temp_dir()` |
| File I/O | `Out_channel` | `std::fs` |
| Uniqueness | Random suffix | pid + timestamp |
