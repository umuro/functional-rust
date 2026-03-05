// 492. OsStr and OsString
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::env;


#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_osstr_roundtrip() { let s="hello"; let os=OsStr::new(s); assert_eq!(os.to_str(),Some(s)); }
    #[test] fn test_path_ext()        { let p=Path::new("f.rs"); assert_eq!(p.extension(),Some(OsStr::new("rs"))); }
    #[test] fn test_os_string()       { let s=OsString::from("hi"); assert_eq!(s.to_string_lossy(),"hi"); }
}
