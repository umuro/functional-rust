// Builder pattern via macro in Rust

// Macro that generates a full builder for a struct
macro_rules! builder {
    (
        pub struct $name:ident {
            $(required: $req_field:ident : $req_ty:ty,)*
            $(optional: $opt_field:ident : $opt_ty:ty = $opt_default:expr,)*
        }
    ) => {
        // The struct itself
        #[derive(Debug)]
        pub struct $name {
            $($req_field: $req_ty,)*
            $($opt_field: $opt_ty,)*
        }

        // The builder struct
        #[derive(Default)]
        pub struct paste::paste!([<$name Builder>]) {
            $($req_field: Option<$req_ty>,)*
            $($opt_field: $opt_ty,)*
        }
    };
}

// Simpler, manual builder macro (without paste)
macro_rules! simple_builder {
    (
        struct $name:ident {
            $($field:ident : $ty:ty $(= $default:expr)?),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        struct $name {
            $($field: $ty,)*
        }

        struct paste_builder {
            $($field: Option<$ty>,)*
        }
    };
}

// Practical: hand-written builder with macro for setters
macro_rules! setters {
    ($($field:ident : $ty:ty),* $(,)?) => {
        $(
            pub fn $field(mut self, val: $ty) -> Self {
                self.$field = Some(val);
                self
            }
        )*
    };
}

#[derive(Debug)]
struct HttpRequest {
    url: String,
    method: String,
    timeout_ms: u32,
    max_retries: u8,
    headers: Vec<(String, String)>,
}

#[derive(Default)]
struct HttpRequestBuilder {
    url: Option<String>,
    method: Option<String>,
    timeout_ms: Option<u32>,
    max_retries: Option<u8>,
    headers: Vec<(String, String)>,
}

impl HttpRequestBuilder {
    setters!(url: String, method: String, timeout_ms: u32, max_retries: u8);

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    pub fn build(self) -> Result<HttpRequest, String> {
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
    fn builder() -> HttpRequestBuilder { HttpRequestBuilder::default() }
}

fn main() {
    let req = HttpRequest::builder()
        .url("https://api.example.com/data".to_string())
        .method("POST".to_string())
        .timeout_ms(10_000)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .build()
        .unwrap();

    println!("{:?}", req);
    println!("URL: {}", req.url);
    println!("Headers: {:?}", req.headers);

    // Missing required field
    let err = HttpRequest::builder().build();
    println!("Missing url: {:?}", err);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_success() {
        let req = HttpRequest::builder()
            .url("http://example.com".to_string())
            .build()
            .unwrap();
        assert_eq!(req.url, "http://example.com");
        assert_eq!(req.method, "GET"); // default
        assert_eq!(req.timeout_ms, 5000); // default
    }

    #[test]
    fn test_builder_missing_required() {
        let result = HttpRequest::builder().build();
        assert!(result.is_err());
    }
}
