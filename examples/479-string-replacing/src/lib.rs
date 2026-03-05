// 479. replace(), replacen()

#[cfg(test)]
mod tests {
    #[test] fn test_replace_all() { assert_eq!("aabaa".replace('a',"x"),"xxbxx"); }
    #[test] fn test_replacen()    { assert_eq!("aabaa".replacen('a',"x",2),"xxbaa"); }
    #[test] fn test_no_match()    { assert_eq!("hello".replace("xyz","abc"),"hello"); }
    #[test] fn test_retain()      { let mut s=String::from("h3llo"); s.retain(|c|c.is_alphabetic()); assert_eq!(s,"hllo"); }
}
