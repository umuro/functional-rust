fn main() {
    // Use the library from src/lib.rs
    println!("Example implementation in src/lib.rs");
    
    // Run any example tests
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(true, true);
        }
    }
}
