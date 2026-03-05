//! # Testing Async Code
//!
//! Patterns for testing async code (using threads as std-only analog).

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// A response from our "async" service
#[derive(Debug, PartialEq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

/// Simulated async HTTP client (using threads)
pub struct HttpClient {
    pub timeout: Duration,
}

impl HttpClient {
    pub fn new(timeout: Duration) -> Self {
        HttpClient { timeout }
    }

    /// Fetch a URL (simulated with thread + channel)
    pub fn fetch(&self, url: &str) -> Result<Response, String> {
        let (tx, rx) = mpsc::channel();
        let url = url.to_string();

        thread::spawn(move || {
            // Simulate network delay
            thread::sleep(Duration::from_millis(10));

            let response = if url.contains("404") {
                Response {
                    status: 404,
                    body: "Not Found".to_string(),
                }
            } else if url.contains("error") {
                return; // Simulate timeout by not sending
            } else {
                Response {
                    status: 200,
                    body: format!("Response from {}", url),
                }
            };

            let _ = tx.send(response);
        });

        rx.recv_timeout(self.timeout)
            .map_err(|_| "Request timed out".to_string())
    }
}

/// A service that depends on the HTTP client
pub struct ApiService {
    client: HttpClient,
}

impl ApiService {
    pub fn new(client: HttpClient) -> Self {
        ApiService { client }
    }

    pub fn get_user(&self, id: u64) -> Result<String, String> {
        let url = format!("https://api.example.com/users/{}", id);
        let response = self.client.fetch(&url)?;

        if response.status == 200 {
            Ok(response.body)
        } else {
            Err(format!("HTTP {}", response.status))
        }
    }
}

/// Retry with backoff
pub fn retry_with_backoff<F, T, E>(
    max_attempts: usize,
    initial_delay: Duration,
    mut f: F,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
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
    unreachable!()
}

/// Parallel fetch multiple URLs
pub fn fetch_all(client: &HttpClient, urls: &[&str]) -> Vec<Result<Response, String>> {
    let handles: Vec<_> = urls
        .iter()
        .map(|url| {
            let url = url.to_string();
            let timeout = client.timeout;
            thread::spawn(move || {
                let c = HttpClient::new(timeout);
                c.fetch(&url)
            })
        })
        .collect();

    handles
        .into_iter()
        .map(|h| h.join().unwrap_or(Err("Thread panicked".to_string())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_success() {
        let client = HttpClient::new(Duration::from_secs(1));
        let result = client.fetch("https://example.com");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 200);
    }

    #[test]
    fn test_fetch_404() {
        let client = HttpClient::new(Duration::from_secs(1));
        let result = client.fetch("https://example.com/404");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 404);
    }

    #[test]
    fn test_fetch_timeout() {
        let client = HttpClient::new(Duration::from_millis(1));
        let result = client.fetch("https://example.com/error");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("timed out"));
    }

    #[test]
    fn test_api_service_get_user() {
        let client = HttpClient::new(Duration::from_secs(1));
        let service = ApiService::new(client);
        let result = service.get_user(123);
        assert!(result.is_ok());
    }

    #[test]
    fn test_retry_succeeds_first_try() {
        let mut attempts = 0;
        let result = retry_with_backoff(3, Duration::from_millis(1), || {
            attempts += 1;
            Ok::<_, &str>(42)
        });
        assert_eq!(result, Ok(42));
        assert_eq!(attempts, 1);
    }

    #[test]
    fn test_retry_succeeds_after_failures() {
        let mut attempts = 0;
        let result = retry_with_backoff(3, Duration::from_millis(1), || {
            attempts += 1;
            if attempts < 3 {
                Err("transient error")
            } else {
                Ok(42)
            }
        });
        assert_eq!(result, Ok(42));
        assert_eq!(attempts, 3);
    }

    #[test]
    fn test_fetch_all_parallel() {
        let client = HttpClient::new(Duration::from_secs(1));
        let urls = ["https://a.com", "https://b.com", "https://c.com"];
        let results = fetch_all(&client, &urls);
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
    }
}
