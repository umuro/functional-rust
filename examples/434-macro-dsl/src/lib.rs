#![allow(clippy::all)]
//! Domain Specific Languages with Macros
//!
//! Creating mini-languages in macros.

/// SQL-like query builder.
#[macro_export]
macro_rules! query {
    (SELECT $($col:ident),+ FROM $table:ident) => {{
        let cols: Vec<&str> = vec![$(stringify!($col)),+];
        format!("SELECT {} FROM {}", cols.join(", "), stringify!($table))
    }};
    (SELECT $($col:ident),+ FROM $table:ident WHERE $cond:expr) => {{
        let cols: Vec<&str> = vec![$(stringify!($col)),+];
        format!("SELECT {} FROM {} WHERE {}", cols.join(", "), stringify!($table), $cond)
    }};
}

/// HTML-like builder.
#[macro_export]
macro_rules! html {
    ($tag:ident { $content:expr }) => {
        format!("<{}>{}</{}>", stringify!($tag), $content, stringify!($tag))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_query_simple() {
        let q = query!(SELECT name, age FROM users);
        assert!(q.contains("SELECT"));
        assert!(q.contains("name"));
        assert!(q.contains("users"));
    }

    #[test]
    fn test_query_where() {
        let q = query!(SELECT id FROM items WHERE "active = true");
        assert!(q.contains("WHERE"));
        assert!(q.contains("active"));
    }

    #[test]
    fn test_html_div() {
        let h = html!(div { "Hello" });
        assert_eq!(h, "<div>Hello</div>");
    }

    #[test]
    fn test_html_span() {
        let h = html!(span { "World" });
        assert_eq!(h, "<span>World</span>");
    }

    #[test]
    fn test_query_multiple_cols() {
        let q = query!(SELECT a, b, c FROM table);
        assert!(q.contains("a, b, c"));
    }
}
