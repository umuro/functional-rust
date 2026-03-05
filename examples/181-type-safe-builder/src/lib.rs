// Example 181: Type-Safe SQL-like Query Builder
// Enforce SELECT before WHERE at compile time using phantom types

use std::marker::PhantomData;

// === Approach 1: Type-state builder ===

struct NoSelect;
struct HasSelect;
struct NoFrom;
struct HasFrom;
struct NoWhere;
struct HasWhere;

struct Query<S, F, W> {
    select: Option<String>,
    from: Option<String>,
    where_: Option<String>,
    order_by: Option<String>,
    _s: PhantomData<(S, F, W)>,
}

impl Query<NoSelect, NoFrom, NoWhere> {
    fn new() -> Self {
        Query {
            select: None, from: None, where_: None, order_by: None,
            _s: PhantomData,
        }
    }
}

impl<F, W> Query<NoSelect, F, W> {
    fn select(self, cols: &str) -> Query<HasSelect, F, W> {
        Query {
            select: Some(cols.to_string()),
            from: self.from, where_: self.where_, order_by: self.order_by,
            _s: PhantomData,
        }
    }
}

impl<W> Query<HasSelect, NoFrom, W> {
    fn from(self, table: &str) -> Query<HasSelect, HasFrom, W> {
        Query {
            select: self.select,
            from: Some(table.to_string()),
            where_: self.where_, order_by: self.order_by,
            _s: PhantomData,
        }
    }
}

impl Query<HasSelect, HasFrom, NoWhere> {
    fn where_(self, cond: &str) -> Query<HasSelect, HasFrom, HasWhere> {
        Query {
            select: self.select, from: self.from,
            where_: Some(cond.to_string()),
            order_by: self.order_by,
            _s: PhantomData,
        }
    }
}

impl<W> Query<HasSelect, HasFrom, W> {
    fn order_by(mut self, col: &str) -> Self {
        self.order_by = Some(col.to_string());
        self
    }

    fn build(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}",
            self.select.as_ref().unwrap(),
            self.from.as_ref().unwrap());
        if let Some(w) = &self.where_ {
            sql.push_str(&format!(" WHERE {}", w));
        }
        if let Some(o) = &self.order_by {
            sql.push_str(&format!(" ORDER BY {}", o));
        }
        sql
    }
}

// === Approach 2: Trait-based builder with associated types ===

trait BuilderState {}
trait CanAddFrom: BuilderState {}
trait CanAddWhere: BuilderState {}
trait CanBuild: BuilderState {}

struct Selected;
struct FromAdded;
struct WhereAdded;

impl BuilderState for Selected {}
impl BuilderState for FromAdded {}
impl BuilderState for WhereAdded {}
impl CanAddFrom for Selected {}
impl CanAddWhere for FromAdded {}
impl CanBuild for FromAdded {}
impl CanBuild for WhereAdded {}

struct QueryBuilder<S: BuilderState> {
    parts: Vec<String>,
    _state: PhantomData<S>,
}

impl QueryBuilder<Selected> {
    fn select(cols: &str) -> Self {
        QueryBuilder {
            parts: vec![format!("SELECT {}", cols)],
            _state: PhantomData,
        }
    }
}

impl<S: CanAddFrom> QueryBuilder<S> {
    fn from(mut self, table: &str) -> QueryBuilder<FromAdded> {
        self.parts.push(format!("FROM {}", table));
        QueryBuilder { parts: self.parts, _state: PhantomData }
    }
}

impl<S: CanAddWhere> QueryBuilder<S> {
    fn where_clause(mut self, cond: &str) -> QueryBuilder<WhereAdded> {
        self.parts.push(format!("WHERE {}", cond));
        QueryBuilder { parts: self.parts, _state: PhantomData }
    }
}

impl<S: CanBuild> QueryBuilder<S> {
    fn build(&self) -> String {
        self.parts.join(" ")
    }
}

// === Approach 3: Runtime builder for comparison ===

#[derive(Default)]
struct FluentQuery {
    select: Option<String>,
    from: Option<String>,
    where_: Option<String>,
}

impl FluentQuery {
    fn select(mut self, cols: &str) -> Self { self.select = Some(cols.into()); self }
    fn from(mut self, table: &str) -> Self { self.from = Some(table.into()); self }
    fn where_(mut self, cond: &str) -> Self { self.where_ = Some(cond.into()); self }

    fn build(&self) -> Result<String, &'static str> {
        match (&self.select, &self.from) {
            (Some(s), Some(f)) => {
                let mut sql = format!("SELECT {} FROM {}", s, f);
                if let Some(w) = &self.where_ { sql.push_str(&format!(" WHERE {}", w)); }
                Ok(sql)
            }
            _ => Err("SELECT and FROM are required"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_state_basic() {
        let sql = Query::new().select("*").from("users").build();
        assert_eq!(sql, "SELECT * FROM users");
    }

    #[test]
    fn test_type_state_where() {
        let sql = Query::new().select("name").from("users").where_("age > 18").build();
        assert_eq!(sql, "SELECT name FROM users WHERE age > 18");
    }

    #[test]
    fn test_type_state_order() {
        let sql = Query::new().select("*").from("users").order_by("name").build();
        assert_eq!(sql, "SELECT * FROM users ORDER BY name");
    }

    #[test]
    fn test_trait_builder() {
        let sql = QueryBuilder::select("*").from("t").build();
        assert_eq!(sql, "SELECT * FROM t");
    }

    #[test]
    fn test_trait_builder_where() {
        let sql = QueryBuilder::select("a").from("b").where_clause("c=1").build();
        assert_eq!(sql, "SELECT a FROM b WHERE c=1");
    }

    #[test]
    fn test_fluent_ok() {
        let r = FluentQuery::default().select("*").from("t").build();
        assert!(r.is_ok());
    }

    #[test]
    fn test_fluent_missing() {
        let r = FluentQuery::default().build();
        assert!(r.is_err());
    }
}
