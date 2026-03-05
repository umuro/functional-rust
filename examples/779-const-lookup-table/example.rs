// 779. Compile-Time Lookup Tables with const Arrays
// CRC-32, ASCII table, sin LUT — all computed at compile time

// ── CRC-32 table (IEEE polynomial) ────────────────────────────────────────────

const CRC32_TABLE: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        t[i] = crc;
        i += 1;
    }
    t
};

pub fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        let idx = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[idx];
    }
    crc ^ 0xFFFF_FFFF
}

// ── ASCII classification table ─────────────────────────────────────────────────

const ASCII_UPPER: [u8; 256] = {
    let mut t = [0u8; 256];
    let mut i = 0usize;
    while i < 256 {
        t[i] = if i >= b'a' as usize && i <= b'z' as usize {
            (i - 32) as u8
        } else {
            i as u8
        };
        i += 1;
    }
    t
};

const IS_ALPHA: [bool; 256] = {
    let mut t = [false; 256];
    let mut i = 0usize;
    while i < 256 {
        t[i] = (i >= b'a' as usize && i <= b'z' as usize)
             || (i >= b'A' as usize && i <= b'Z' as usize);
        i += 1;
    }
    t
};

pub fn ascii_uppercase(s: &str) -> Vec<u8> {
    s.bytes().map(|b| ASCII_UPPER[b as usize]).collect()
}

// ── Base64 encoding table ─────────────────────────────────────────────────────

const BASE64_TABLE: [u8; 64] = {
    let mut t = [0u8; 64];
    let alpha = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut i = 0usize;
    while i < 64 { t[i] = alpha[i]; i += 1; }
    t
};

pub fn base64_encode(data: &[u8]) -> String {
    let mut out = Vec::with_capacity((data.len() + 2) / 3 * 4);
    let mut chunks = data.chunks_exact(3);
    for chunk in &mut chunks {
        let v = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | chunk[2] as u32;
        out.push(BASE64_TABLE[(v >> 18) as usize]);
        out.push(BASE64_TABLE[((v >> 12) & 0x3F) as usize]);
        out.push(BASE64_TABLE[((v >> 6) & 0x3F) as usize]);
        out.push(BASE64_TABLE[(v & 0x3F) as usize]);
    }
    match chunks.remainder() {
        [a] => {
            let v = (*a as u32) << 16;
            out.push(BASE64_TABLE[(v >> 18) as usize]);
            out.push(BASE64_TABLE[((v >> 12) & 0x3F) as usize]);
            out.push(b'='); out.push(b'=');
        }
        [a, b] => {
            let v = ((*a as u32) << 16) | ((*b as u32) << 8);
            out.push(BASE64_TABLE[(v >> 18) as usize]);
            out.push(BASE64_TABLE[((v >> 12) & 0x3F) as usize]);
            out.push(BASE64_TABLE[((v >> 6) & 0x3F) as usize]);
            out.push(b'=');
        }
        _ => {}
    }
    String::from_utf8(out).unwrap()
}

fn main() {
    println!("CRC-32 table has {} entries", CRC32_TABLE.len());
    println!("CRC32(\"hello\")        = {:08X}", crc32(b"hello"));
    println!("CRC32(\"123456789\")    = {:08X}", crc32(b"123456789")); // known: CBF43926

    let upper = ascii_uppercase("Hello, World!");
    println!("uppercase: {}", String::from_utf8(upper).unwrap());

    println!("alpha 'a': {}", IS_ALPHA[b'a' as usize]);
    println!("alpha '5': {}", IS_ALPHA[b'5' as usize]);

    println!("base64(\"Man\")     = {}", base64_encode(b"Man"));   // TWFu
    println!("base64(\"hello\")  = {}", base64_encode(b"hello")); // aGVsbG8=
    println!("base64(\"Rust\")   = {}", base64_encode(b"Rust"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_known_value() {
        // CRC32 of "123456789" is 0xCBF43926
        assert_eq!(crc32(b"123456789"), 0xCBF43926);
    }

    #[test]
    fn uppercase_table() {
        let u = ascii_uppercase("hello");
        assert_eq!(u, b"HELLO");
    }

    #[test]
    fn uppercase_leaves_nonalpha() {
        let u = ascii_uppercase("Hello 123!");
        assert_eq!(&u, b"HELLO 123!");
    }

    #[test]
    fn base64_standard() {
        assert_eq!(base64_encode(b"Man"), "TWFu");
        assert_eq!(base64_encode(b"Ma"), "TWE=");
        assert_eq!(base64_encode(b"M"), "TQ==");
    }
}
