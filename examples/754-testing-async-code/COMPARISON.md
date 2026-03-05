# OCaml vs Rust: Testing Async Code

## Async Test Setup

### Rust (tokio)
```rust
#[tokio::test]
async fn test_fetch() {
    let client = HttpClient::new();
    let result = client.fetch("https://example.com").await;
    assert!(result.is_ok());
}
```

### Rust (std-only with threads)
```rust
#[test]
fn test_fetch() {
    let client = HttpClient::new(Duration::from_secs(1));
    let result = client.fetch("https://example.com");
    assert!(result.is_ok());
}
```

### OCaml (Lwt)
```ocaml
let%lwt () =
  let%lwt response = Http_client.fetch "https://example.com" in
  assert (response.status = 200);
  Lwt.return ()
```

## Timeout Testing

### Rust
```rust
#[test]
fn test_timeout() {
    let client = HttpClient::new(Duration::from_millis(1));
    let result = client.fetch("https://slow.example.com");
    assert!(result.is_err());
}
```

### OCaml
```ocaml
let test_timeout () =
  let timeout = Lwt_unix.timeout 0.001 in
  match Lwt_main.run (Lwt.pick [timeout; fetch url]) with
  | exception Lwt_unix.Timeout -> ()
  | _ -> failwith "expected timeout"
```

## Retry with Backoff

### Rust
```rust
pub fn retry_with_backoff<F, T, E>(
    max_attempts: usize,
    initial_delay: Duration,
    mut f: F,
) -> Result<T, E> {
    let mut delay = initial_delay;
    for attempt in 1..=max_attempts {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) if attempt == max_attempts => return Err(e),
            Err(_) => {
                thread::sleep(delay);
                delay *= 2;
            }
        }
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Async runtime | Lwt, Async | tokio, async-std |
| Test attribute | ppx_lwt | `#[tokio::test]` |
| Channels | Lwt_mvar | mpsc, crossbeam |
| Timeout | `Lwt_unix.timeout` | `recv_timeout` |
| Parallelism | `Lwt.join` | `tokio::join!` |
