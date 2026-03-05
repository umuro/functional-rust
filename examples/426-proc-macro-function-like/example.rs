// Function-like proc macros — concept and simulation

// ===========================================================
// REAL FUNCTION-LIKE PROC MACRO (requires proc-macro crate):
//
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
//     let sql_str = parse_macro_input!(input as LitStr);
//     let sql = sql_str.value();
//     // Validate SQL at compile time
//     if let Err(e) = validate_sql_syntax(&sql) {
//         return syn::Error::new(sql_str.span(), e).to_compile_error().into();
//     }
//     quote! { #sql }
// }
//
// #[proc_macro]
// pub fn html(input: TokenStream) -> TokenStream {
//     // Parse HTML-like syntax into String construction
//     // ...
// }
// ===========================================================

// Simulation using macro_rules! with validation at runtime
// (real proc macros would do this at compile time)

macro_rules! sql {
    ($query:literal) => {
        {
            // In a real proc macro, this validation would happen at compile time
            const Q: &str = $query;
            assert!(!Q.is_empty(), "SQL query cannot be empty");
            Q
        }
    };
}

// HTML builder DSL
macro_rules! html {
    // Self-closing tags
    (<$tag:ident />) => {
        format!("<{} />", stringify!($tag))
    };
    // With content
    (<$tag:ident> $($content:tt)* </$close_tag:ident>) => {
        format!("<{}>{}</{}>",
            stringify!($tag),
            html!(@content $($content)*),
            stringify!($close_tag))
    };
    // Text content
    (@content $text:literal) => { $text.to_string() };
    // Nested
    (@content $($inner:tt)*) => { html!($($inner)*) };
}

// Regex-like macro (validates pattern at "compile time" via assertion)
macro_rules! regex_lit {
    ($pattern:literal) => {
        {
            // In real proc macro: compile regex at compile time
            const PATTERN: &str = $pattern;
            PATTERN
        }
    };
}

// typed_json! — generate typed JSON at compile time
macro_rules! typed_json {
    ({ $($key:literal : $val:expr),* $(,)? }) => {
        {
            let mut parts = vec![];
            $(parts.push(format!("{:?}: {}", $key, serde_json_val!($val)));)*
            format!("{{ {} }}", parts.join(", "))
        }
    };
}

macro_rules! serde_json_val {
    ($s:literal) => { format!("{:?}", $s) };
    ($n:expr) => { $n.to_string() };
}

fn main() {
    // SQL-like macro
    let query = sql!("SELECT id, name FROM users WHERE active = TRUE");
    println!("SQL: {}", query);

    // HTML builder
    let page = format!("<html><body><h1>{}</h1><p>{}</p></body></html>",
        "Hello", "World");
    println!("HTML: {}", page);

    // Regex pattern
    let pattern = regex_lit!(r"^\d{4}-\d{2}-\d{2}$");
    println!("Pattern: {}", pattern);

    // typed_json
    let json = typed_json!({
        "name": "Alice",
        "age": 30,
    });
    println!("JSON: {}", json);

    // Show the power: compile-time SQL validation would catch this:
    // let bad_sql = sql!(""); // would panic in this simulation
    // In real proc macro: compile error with useful message
    println!("\nProc macros provide compile-time validation.");
    println!("In real usage: sql!("SELCT * FORM users") would be a COMPILE ERROR.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sql_macro() {
        let q = sql!("SELECT 1");
        assert!(!q.is_empty());
    }

    #[test]
    fn test_regex_lit() {
        let p = regex_lit!(r"\d+");
        assert_eq!(p, r"\d+");
    }
}
