// 921: Async I/O — Tokio version
// Native async I/O using tokio::io, tokio::net, tokio::fs

use tokio::io::AsyncWriteExt;

/// Process text statistics
fn process_text(text: &str) -> (usize, usize, usize) {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    (lines, words, chars)
}

/// Async "file read" simulation using tokio::spawn
async fn async_read(content: String) -> String {
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    content
}

/// Spawn an async I/O task
async fn spawn_io_task<T: Send + 'static>(
    f: impl std::future::Future<Output = T> + Send + 'static,
) -> T {
    tokio::spawn(f).await.unwrap()
}

/// Non-blocking TCP listener using tokio
async fn nonblocking_listener() -> bool {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Use tokio::select! with timeout for non-blocking accept
    let result = tokio::select! {
        _ = listener.accept() => true,
        _ = tokio::time::sleep(std::time::Duration::from_millis(5)) => false,
    };

    result // false — no connections
}

/// Parallel async I/O tasks
async fn parallel_io_tasks() -> Vec<(usize, usize, usize)> {
    let texts = vec![
        "first line\nsecond line",
        "alpha beta gamma delta",
        "one\ntwo\nthree\nfour\nfive",
    ];

    let handles: Vec<_> = texts
        .into_iter()
        .map(|s| {
            let s = s.to_string();
            tokio::spawn(async move { process_text(&s) })
        })
        .collect();

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

/// Async write to buffer
async fn async_write_to_buf(data: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.write_all(data).await.unwrap();
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_text_counts() {
        let (l, w, c) = process_text("hello world\nfoo");
        assert_eq!(l, 2);
        assert_eq!(w, 3);
        assert_eq!(c, 15);
    }

    #[tokio::test]
    async fn test_spawn_io_returns_value() {
        let result = spawn_io_task(async { 42i32 }).await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_async_read() {
        let text = async_read("hello world".to_string()).await;
        assert_eq!(text, "hello world");
    }

    #[tokio::test]
    async fn test_nonblocking_listener() {
        let got_connection = nonblocking_listener().await;
        assert!(!got_connection); // no connections
    }

    #[tokio::test]
    async fn test_parallel_io() {
        let results = parallel_io_tasks().await;
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], (2, 4, 22)); // "first line\nsecond line"
    }

    #[tokio::test]
    async fn test_async_write() {
        let buf = async_write_to_buf(b"hello").await;
        assert_eq!(buf, b"hello");
    }
}
