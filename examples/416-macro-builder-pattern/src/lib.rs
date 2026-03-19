//! Macro-Generated Builder Pattern
//!
//! Using macros to reduce boilerplate in builder patterns.

/// Generate setter methods for builder fields.
#[macro_export]
macro_rules! builder_setters {
    ($($field:ident : $ty:ty),* $(,)?) => {
        $(
            pub fn $field(mut self, val: $ty) -> Self {
                self.$field = Some(val);
                self
            }
        )*
    };
}

/// Generate a builder with required and optional fields.
#[macro_export]
macro_rules! define_builder {
    (
        $vis:vis struct $name:ident {
            $(required $req:ident : $req_ty:ty,)*
            $(optional $opt:ident : $opt_ty:ty = $default:expr,)*
        }
    ) => {
        #[derive(Debug, Clone)]
        $vis struct $name {
            $($req: $req_ty,)*
            $($opt: $opt_ty,)*
        }

        paste::item! {
            #[derive(Default)]
            $vis struct [<$name Builder>] {
                $($req: Option<$req_ty>,)*
                $($opt: Option<$opt_ty>,)*
            }
        }
    };
}

/// HTTP Request for demonstration.
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub timeout_ms: u32,
    pub max_retries: u8,
    pub headers: Vec<(String, String)>,
}

/// Builder for HttpRequest.
#[derive(Default)]
pub struct HttpRequestBuilder {
    url: Option<String>,
    method: Option<String>,
    timeout_ms: Option<u32>,
    max_retries: Option<u8>,
    headers: Vec<(String, String)>,
}

impl HttpRequestBuilder {
    builder_setters!(url: String, method: String, timeout_ms: u32, max_retries: u8);

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    pub fn build(self) -> Result<HttpRequest, &'static str> {
        Ok(HttpRequest {
            url: self.url.ok_or("url is required")?,
            method: self.method.unwrap_or_else(|| "GET".to_string()),
            timeout_ms: self.timeout_ms.unwrap_or(5000),
            max_retries: self.max_retries.unwrap_or(3),
            headers: self.headers,
        })
    }
}

impl HttpRequest {
    pub fn builder() -> HttpRequestBuilder {
        HttpRequestBuilder::default()
    }
}

/// Email message builder example.
#[derive(Debug, Clone)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub cc: Vec<String>,
}

#[derive(Default)]
pub struct EmailBuilder {
    to: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    cc: Vec<String>,
}

impl EmailBuilder {
    builder_setters!(to: String, subject: String, body: String);

    pub fn cc(mut self, addr: &str) -> Self {
        self.cc.push(addr.to_string());
        self
    }

    pub fn build(self) -> Result<Email, &'static str> {
        Ok(Email {
            to: self.to.ok_or("to is required")?,
            subject: self.subject.ok_or("subject is required")?,
            body: self.body.unwrap_or_default(),
            cc: self.cc,
        })
    }
}

impl Email {
    pub fn builder() -> EmailBuilder {
        EmailBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_builder() {
        let req = HttpRequest::builder()
            .url("https://api.example.com".to_string())
            .build()
            .unwrap();
        assert_eq!(req.url, "https://api.example.com");
        assert_eq!(req.method, "GET");
    }

    #[test]
    fn test_http_request_with_options() {
        let req = HttpRequest::builder()
            .url("https://api.example.com".to_string())
            .method("POST".to_string())
            .timeout_ms(10000)
            .build()
            .unwrap();
        assert_eq!(req.method, "POST");
        assert_eq!(req.timeout_ms, 10000);
    }

    #[test]
    fn test_http_request_with_headers() {
        let req = HttpRequest::builder()
            .url("https://api.example.com".to_string())
            .header("Authorization", "Bearer token")
            .header("Content-Type", "application/json")
            .build()
            .unwrap();
        assert_eq!(req.headers.len(), 2);
    }

    #[test]
    fn test_http_request_missing_url() {
        let result = HttpRequest::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_email_builder() {
        let email = Email::builder()
            .to("user@example.com".to_string())
            .subject("Hello".to_string())
            .body("World".to_string())
            .build()
            .unwrap();
        assert_eq!(email.to, "user@example.com");
        assert_eq!(email.subject, "Hello");
    }

    #[test]
    fn test_email_with_cc() {
        let email = Email::builder()
            .to("user@example.com".to_string())
            .subject("Test".to_string())
            .cc("cc1@example.com")
            .cc("cc2@example.com")
            .build()
            .unwrap();
        assert_eq!(email.cc.len(), 2);
    }

    #[test]
    fn test_email_missing_required() {
        let result = Email::builder().to("user@example.com".to_string()).build();
        assert!(result.is_err());
    }
}
