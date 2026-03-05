// Custom #[derive(MyTrait)] — concept and simulation

// ===========================================================
// REAL PROC MACRO (separate crate, shown for reference):
//
// #[proc_macro_derive(Builder, attributes(builder))]
// pub fn derive_builder(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = &input.ident;
//     let builder_name = Ident::new(&format!("{}Builder", name), name.span());
//
//     let fields = match &input.data {
//         Data::Struct(DataStruct { fields: Fields::Named(f), .. }) => &f.named,
//         _ => panic!("Builder only supports named structs"),
//     };
//
//     let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
//     let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
//
//     quote! {
//         struct #builder_name {
//             #( #field_names: Option<#field_types> ),*
//         }
//         impl #builder_name {
//             #( pub fn #field_names(mut self, v: #field_types) -> Self {
//                 self.#field_names = Some(v); self
//             } )*
//             pub fn build(self) -> Result<#name, String> {
//                 Ok(#name {
//                     #( #field_names: self.#field_names.ok_or(stringify!(#field_names))? ),*
//                 })
//             }
//         }
//     }
// }
// ===========================================================

// Simulation: macro_rules! mimics what the proc macro would generate

macro_rules! derive_builder_sim {
    (
        struct $name:ident {
            $($field:ident : $ty:ty),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        struct $name { $($field: $ty,)* }

        paste::paste! {
            struct [<$name Builder>] { $($field: Option<$ty>,)* }
        }
    };
}

// Manual simulation of proc macro output for DatabaseConfig
#[derive(Debug, Clone)]
struct DatabaseConfig {
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
    max_pool: u32,
}

// What #[derive(Builder)] would generate:
struct DatabaseConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    database: Option<String>,
    username: Option<String>,
    password: Option<String>,
    max_pool: Option<u32>,
}

impl DatabaseConfigBuilder {
    fn new() -> Self {
        DatabaseConfigBuilder {
            host: None, port: None, database: None,
            username: None, password: None, max_pool: None,
        }
    }
    fn host(mut self, v: impl Into<String>) -> Self { self.host = Some(v.into()); self }
    fn port(mut self, v: u16) -> Self { self.port = Some(v); self }
    fn database(mut self, v: impl Into<String>) -> Self { self.database = Some(v.into()); self }
    fn username(mut self, v: impl Into<String>) -> Self { self.username = Some(v.into()); self }
    fn password(mut self, v: impl Into<String>) -> Self { self.password = Some(v.into()); self }
    fn max_pool(mut self, v: u32) -> Self { self.max_pool = Some(v); self }

    fn build(self) -> Result<DatabaseConfig, String> {
        Ok(DatabaseConfig {
            host: self.host.ok_or("host is required")?,
            port: self.port.unwrap_or(5432),
            database: self.database.ok_or("database is required")?,
            username: self.username.ok_or("username is required")?,
            password: self.password.unwrap_or_default(),
            max_pool: self.max_pool.unwrap_or(10),
        })
    }
}

impl DatabaseConfig {
    fn builder() -> DatabaseConfigBuilder { DatabaseConfigBuilder::new() }
}

fn main() {
    let config = DatabaseConfig::builder()
        .host("localhost")
        .database("myapp")
        .username("admin")
        .password("secret")
        .max_pool(20)
        .build()
        .unwrap();

    println!("{:?}", config);
    println!("{}:{}/{}", config.host, config.port, config.database);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let c = DatabaseConfig::builder()
            .host("db.example.com")
            .database("prod")
            .username("app")
            .build()
            .unwrap();
        assert_eq!(c.host, "db.example.com");
        assert_eq!(c.port, 5432); // default
    }

    #[test]
    fn test_builder_missing_required() {
        let r = DatabaseConfig::builder().build();
        assert!(r.is_err());
    }
}
