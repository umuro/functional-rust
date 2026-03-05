// Procedural macros overview in Rust
// Since proc macros require a separate crate, we SIMULATE their output
// using macro_rules! and show what the generated code would look like.

// ===========================================================
// CONCEPT: What a proc macro crate would look like
// ===========================================================
//
// In a real proc macro crate (lib.rs with proc-macro = true):
//
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput};
//
// #[proc_macro_derive(Describe)]
// pub fn derive_describe(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = &input.ident;
//     let fields = if let syn::Data::Struct(data) = &input.data {
//         data.fields.iter().map(|f| &f.ident).collect::<Vec<_>>()
//     } else { vec![] };
//
//     let expanded = quote! {
//         impl Describe for #name {
//             fn describe(&self) -> String {
//                 format!("{} {{ {} }}",
//                     stringify!(#name),
//                     vec![ #( format!("{}: {:?}", stringify!(#fields), self.#fields) ),* ]
//                         .join(", "))
//             }
//         }
//     };
//     TokenStream::from(expanded)
// }
// ===========================================================

// Simulation: what #[derive(Describe)] would generate
trait Describe {
    fn describe(&self) -> String;
}

// Manual implementation (what the proc macro would generate):
struct Person { name: String, age: u32 }
struct Point3D { x: f64, y: f64, z: f64 }

impl Describe for Person {
    fn describe(&self) -> String {
        format!("Person {{ name: {:?}, age: {:?} }}", self.name, self.age)
    }
}

impl Describe for Point3D {
    fn describe(&self) -> String {
        format!("Point3D {{ x: {:?}, y: {:?}, z: {:?} }}", self.x, self.y, self.z)
    }
}

// Simulate with a macro_rules! version
macro_rules! derive_describe {
    ($type:ident { $($field:ident),+ $(,)? }) => {
        impl Describe for $type {
            fn describe(&self) -> String {
                let mut parts = vec![];
                $(
                    parts.push(format!("{}: {:?}", stringify!($field), self.$field));
                )+
                format!("{} {{ {} }}", stringify!($type), parts.join(", "))
            }
        }
    };
}

struct Config { host: String, port: u16 }
derive_describe!(Config { host, port });

fn main() {
    let p = Person { name: "Alice".to_string(), age: 30 };
    println!("{}", p.describe());

    let pt = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    println!("{}", pt.describe());

    let c = Config { host: "localhost".to_string(), port: 8080 };
    println!("{}", c.describe());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe() {
        let p = Person { name: "Bob".to_string(), age: 25 };
        let desc = p.describe();
        assert!(desc.contains("Person"));
        assert!(desc.contains("Bob"));
        assert!(desc.contains("25"));
    }
}
