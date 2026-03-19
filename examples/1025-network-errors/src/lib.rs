// 1025: Network Error Classification (Simulated)
// Classifying and handling network-like errors

use std::fmt;

#[derive(Debug)]
enum NetError {
    Timeout { seconds: f64 },
    ConnectionRefused(String),
    DnsResolutionFailed(String),
    TlsError(String),
    HttpError { status: u16, body: String },
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetError::Timeout { seconds } => write!(f, "timeout after {:.1}s", seconds),
            NetError::ConnectionRefused(host) => write!(f, "connection refused: {}", host),
            NetError::DnsResolutionFailed(host) => write!(f, "DNS failed: {}", host),
            NetError::TlsError(msg) => write!(f, "TLS error: {}", msg),
            NetError::HttpError { status, body } => write!(f, "HTTP {}: {}", status, body),
        }
    }
}
impl std::error::Error for NetError {}

impl NetError {
    fn is_retryable(&self) -> bool {
        match self {
            NetError::Timeout { .. } => true,
            NetError::ConnectionRefused(_) => true,
            NetError::DnsResolutionFailed(_) => false,
            NetError::TlsError(_) => false,
            NetError::HttpError { status, .. } => *status >= 500,
        }
    }

    fn is_client_error(&self) -> bool {
        matches!(self, NetError::HttpError { status, .. } if *status >= 400 && *status < 500)
    }
}

// Simulated network call
fn fetch(url: &str) -> Result<String, NetError> {
    match url {
        "" => Err(NetError::DnsResolutionFailed("empty url".into())),
        "http://timeout" => Err(NetError::Timeout { seconds: 30.0 }),
        "http://refused" => Err(NetError::ConnectionRefused("refused:80".into())),
        "http://500" => Err(NetError::HttpError {
            status: 500,
            body: "Internal Server Error".into(),
        }),
        "http://404" => Err(NetError::HttpError {
            status: 404,
            body: "Not Found".into(),
        }),
        url => Ok(format!("response from {}", url)),
    }
}

// Retry logic
fn fetch_with_retry(url: &str, max_retries: u32) -> Result<String, NetError> {
    let mut last_error = None;
    for attempt in 0..=max_retries {
        match fetch(url) {
            Ok(response) => return Ok(response),
            Err(e) if e.is_retryable() && attempt < max_retries => {
                last_error = Some(e);
                // In real code: sleep with exponential backoff
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err(last_error.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert!(fetch("http://example.com").is_ok());
    }

    #[test]
    fn test_timeout() {
        let err = fetch("http://timeout").unwrap_err();
        assert!(matches!(err, NetError::Timeout { .. }));
        assert!(err.is_retryable());
    }

    #[test]
    fn test_connection_refused() {
        let err = fetch("http://refused").unwrap_err();
        assert!(matches!(err, NetError::ConnectionRefused(_)));
        assert!(err.is_retryable());
    }

    #[test]
    fn test_dns_not_retryable() {
        let err = fetch("").unwrap_err();
        assert!(matches!(err, NetError::DnsResolutionFailed(_)));
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_http_500_retryable() {
        let err = fetch("http://500").unwrap_err();
        assert!(err.is_retryable());
        assert!(!err.is_client_error());
    }

    #[test]
    fn test_http_404_not_retryable() {
        let err = fetch("http://404").unwrap_err();
        assert!(!err.is_retryable());
        assert!(err.is_client_error());
    }

    #[test]
    fn test_retry_success() {
        let result = fetch_with_retry("http://example.com", 3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_retry_exhausted() {
        let result = fetch_with_retry("http://timeout", 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_no_retry_on_client_error() {
        let result = fetch_with_retry("http://404", 3);
        assert!(result.is_err()); // should fail immediately, no retries
    }

    #[test]
    fn test_display() {
        let err = NetError::Timeout { seconds: 5.0 };
        assert_eq!(err.to_string(), "timeout after 5.0s");

        let err = NetError::HttpError {
            status: 503,
            body: "Unavailable".into(),
        };
        assert_eq!(err.to_string(), "HTTP 503: Unavailable");
    }
}
