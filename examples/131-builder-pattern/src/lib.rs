#![allow(clippy::all)]
// Example 131: Builder Pattern with Typestate
//
// The typestate builder encodes which required fields have been set directly in
// the type parameters. `UserBuilder<Missing, Missing>` has no `build()` method.
// `UserBuilder<Present, Present>` does. Forgetting a required field is a
// *compile-time* error, not a runtime panic or a `Result` error.

use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// Marker types — zero-sized, carry only type information
// ---------------------------------------------------------------------------

/// A required field that has not yet been provided.
pub struct Missing;
/// A required field that has been provided.
pub struct Present;

// ---------------------------------------------------------------------------
// Approach 1: UserBuilder — name + email required, age optional
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: Option<u32>,
}

/// A builder whose type parameters `N` and `E` track whether `name` and
/// `email` have been set. Both must be `Present` before `build()` is callable.
pub struct UserBuilder<N, E> {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
    _phantom: PhantomData<(N, E)>,
}

// --- Initial state: nothing set ---

impl UserBuilder<Missing, Missing> {
    pub fn new() -> Self {
        UserBuilder {
            name: None,
            email: None,
            age: None,
            _phantom: PhantomData,
        }
    }
}

impl Default for UserBuilder<Missing, Missing> {
    fn default() -> Self {
        Self::new()
    }
}

// --- Setting `name` transitions N: Missing → Present ---

impl<E> UserBuilder<Missing, E> {
    /// Providing a name transitions the builder from `Missing` to `Present`
    /// for the name slot. The email slot state `E` is preserved unchanged.
    pub fn name(self, name: &str) -> UserBuilder<Present, E> {
        UserBuilder {
            name: Some(name.to_string()),
            email: self.email,
            age: self.age,
            _phantom: PhantomData,
        }
    }
}

// --- Setting `email` transitions E: Missing → Present ---

impl<N> UserBuilder<N, Missing> {
    /// Providing an email transitions the builder from `Missing` to `Present`
    /// for the email slot. The name slot state `N` is preserved unchanged.
    pub fn email(self, email: &str) -> UserBuilder<N, Present> {
        UserBuilder {
            name: self.name,
            email: Some(email.to_string()),
            age: self.age,
            _phantom: PhantomData,
        }
    }
}

// --- Optional field: `age` is available in all states ---

impl<N, E> UserBuilder<N, E> {
    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
}

// --- `build()` only exists when both N = Present and E = Present ---

impl UserBuilder<Present, Present> {
    /// Infallible: the types guarantee that `name` and `email` are both `Some`.
    pub fn build(self) -> User {
        User {
            // SAFETY: Present state guarantees these fields were set.
            name: self.name.expect("Present guarantees name is Some"),
            email: self.email.expect("Present guarantees email is Some"),
            age: self.age,
        }
    }
}

// ---------------------------------------------------------------------------
// Approach 2: HttpRequestBuilder — url + method required, body optional
// ---------------------------------------------------------------------------
// A second demonstration of the same pattern with different required fields.

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub body: Option<String>,
}

pub struct HttpRequestBuilder<U, M> {
    url: Option<String>,
    method: Option<String>,
    body: Option<String>,
    _phantom: PhantomData<(U, M)>,
}

impl HttpRequestBuilder<Missing, Missing> {
    pub fn new() -> Self {
        HttpRequestBuilder {
            url: None,
            method: None,
            body: None,
            _phantom: PhantomData,
        }
    }
}

impl Default for HttpRequestBuilder<Missing, Missing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M> HttpRequestBuilder<Missing, M> {
    pub fn url(self, url: &str) -> HttpRequestBuilder<Present, M> {
        HttpRequestBuilder {
            url: Some(url.to_string()),
            method: self.method,
            body: self.body,
            _phantom: PhantomData,
        }
    }
}

impl<U> HttpRequestBuilder<U, Missing> {
    pub fn method(self, method: &str) -> HttpRequestBuilder<U, Present> {
        HttpRequestBuilder {
            url: self.url,
            method: Some(method.to_string()),
            body: self.body,
            _phantom: PhantomData,
        }
    }
}

impl<U, M> HttpRequestBuilder<U, M> {
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }
}

impl HttpRequestBuilder<Present, Present> {
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            url: self.url.expect("Present guarantees url is Some"),
            method: self.method.expect("Present guarantees method is Some"),
            body: self.body,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- UserBuilder tests ---

    #[test]
    fn test_user_with_required_fields_only() {
        let user = UserBuilder::new()
            .name("Alice")
            .email("alice@example.com")
            .build();
        assert_eq!(
            user,
            User {
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
                age: None,
            }
        );
    }

    #[test]
    fn test_user_with_all_fields() {
        let user = UserBuilder::new()
            .name("Bob")
            .email("bob@example.com")
            .age(30)
            .build();
        assert_eq!(
            user,
            User {
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
                age: Some(30),
            }
        );
    }

    #[test]
    fn test_user_email_before_name_order_independent() {
        // The builder accepts fields in any order — both chains compile.
        let user_a = UserBuilder::new()
            .name("Carol")
            .email("carol@example.com")
            .build();
        let user_b = UserBuilder::new()
            .email("carol@example.com")
            .name("Carol")
            .build();
        assert_eq!(user_a, user_b);
    }

    #[test]
    fn test_user_age_can_be_set_at_any_point_in_chain() {
        let user = UserBuilder::new()
            .age(25)
            .name("Dave")
            .email("dave@example.com")
            .build();
        assert_eq!(user.age, Some(25));
        assert_eq!(user.name, "Dave");
    }

    #[test]
    fn test_user_default_is_same_as_new() {
        // Compile-time check: UserBuilder::default() produces Missing, Missing.
        let user = UserBuilder::default()
            .name("Eve")
            .email("eve@example.com")
            .build();
        assert_eq!(user.name, "Eve");
    }

    // --- HttpRequestBuilder tests ---

    #[test]
    fn test_http_get_request_no_body() {
        let req = HttpRequestBuilder::new()
            .url("https://api.example.com/users")
            .method("GET")
            .build();
        assert_eq!(
            req,
            HttpRequest {
                url: "https://api.example.com/users".to_string(),
                method: "GET".to_string(),
                body: None,
            }
        );
    }

    #[test]
    fn test_http_post_request_with_body() {
        let req = HttpRequestBuilder::new()
            .method("POST")
            .url("https://api.example.com/users")
            .body(r#"{"name":"Alice"}"#)
            .build();
        assert_eq!(req.method, "POST");
        assert_eq!(req.body, Some(r#"{"name":"Alice"}"#.to_string()));
    }

    #[test]
    fn test_http_builder_order_independent() {
        let req_a = HttpRequestBuilder::new()
            .url("https://example.com")
            .method("DELETE")
            .build();
        let req_b = HttpRequestBuilder::new()
            .method("DELETE")
            .url("https://example.com")
            .build();
        assert_eq!(req_a, req_b);
    }
}
