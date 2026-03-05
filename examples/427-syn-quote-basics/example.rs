// syn + quote for proc macro AST — concept and demonstration

// ===========================================================
// REAL syn + quote USAGE (in a proc-macro crate):
//
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput, Data, Fields};
//
// #[proc_macro_derive(Getters)]
// pub fn derive_getters(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_name = &input.ident;
//
//     // Extract named fields
//     let fields = match &input.data {
//         Data::Struct(s) => match &s.fields {
//             Fields::Named(f) => &f.named,
//             _ => panic!("Only named fields supported"),
//         },
//         _ => panic!("Only structs supported"),
//     };
//
//     // Generate getter for each field
//     let getters = fields.iter().map(|f| {
//         let name = f.ident.as_ref().unwrap();
//         let ty = &f.ty;
//         quote! {
//             pub fn #name(&self) -> &#ty {
//                 &self.#name
//             }
//         }
//     });
//
//     // Wrap in impl block
//     let expanded = quote! {
//         impl #struct_name {
//             #(#getters)*
//         }
//     };
//
//     TokenStream::from(expanded)
// }
//
// Then usage:
// #[derive(Getters)]
// struct User { name: String, age: u32 }
// let u = User { name: "Alice".to_string(), age: 30 };
// println!("{}", u.name()); // generated getter
// ===========================================================

// Simulation: macro_rules! generates getters
macro_rules! derive_getters {
    (
        struct $name:ident {
            $($field:ident : $ty:ty),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        struct $name { $($field: $ty,)* }

        impl $name {
            $(
                pub fn $field(&self) -> &$ty { &self.$field }
            )*
        }
    };
}

// Simulation: generate Display impl from fields
macro_rules! derive_display {
    ($name:ident { $($field:ident),* $(,)? }) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, concat!(stringify!($name), " {{ "
                    $(, stringify!($field), ": {:?}, ")*
                    , "}}"),
                    $(&self.$field,)*
                )
            }
        }
    };
}

derive_getters!(
    struct User {
        name: String,
        age: u32,
        email: String,
    }
);

derive_getters!(
    struct Point {
        x: f64,
        y: f64,
    }
);

fn main() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    // Generated getters
    println!("name: {}", user.name());
    println!("age: {}", user.age());
    println!("email: {}", user.email());

    let point = Point { x: 3.14, y: 2.72 };
    println!("x: {}, y: {}", point.x(), point.y());

    println!("
syn parses Rust into typed AST:");
    println!("  DeriveInput.ident = struct name");
    println!("  DeriveInput.data = struct/enum/union body");
    println!("  quote! interpolates with #var, #(#vec)*");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_getters() {
        let u = User { name: "Bob".to_string(), age: 25, email: "b@b.com".to_string() };
        assert_eq!(u.name(), "Bob");
        assert_eq!(*u.age(), 25);
    }
}
