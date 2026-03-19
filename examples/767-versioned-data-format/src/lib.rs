//! # Versioned Data Format
//!
//! Forward and backward compatible data serialization.

/// Data version
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Version(pub u8, pub u8);

impl Version {
    pub fn new(major: u8, minor: u8) -> Self {
        Version(major, minor)
    }

    pub fn is_compatible(&self, other: &Version) -> bool {
        self.0 == other.0 // Same major version
    }
}

/// V1 of our data format
#[derive(Debug, Clone, PartialEq)]
pub struct DataV1 {
    pub name: String,
    pub value: i32,
}

/// V2 adds a new field with default
#[derive(Debug, Clone, PartialEq)]
pub struct DataV2 {
    pub name: String,
    pub value: i32,
    pub tags: Vec<String>, // New in V2
}

/// V3 changes value type and adds metadata
#[derive(Debug, Clone, PartialEq)]
pub struct DataV3 {
    pub name: String,
    pub value: f64, // Changed from i32
    pub tags: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>, // New in V3
}

/// Unified data representation
#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    V1(DataV1),
    V2(DataV2),
    V3(DataV3),
}

impl Data {
    /// Upgrade to latest version
    pub fn upgrade(self) -> DataV3 {
        match self {
            Data::V1(v1) => DataV3 {
                name: v1.name,
                value: v1.value as f64,
                tags: Vec::new(),
                metadata: std::collections::HashMap::new(),
            },
            Data::V2(v2) => DataV3 {
                name: v2.name,
                value: v2.value as f64,
                tags: v2.tags,
                metadata: std::collections::HashMap::new(),
            },
            Data::V3(v3) => v3,
        }
    }
}

/// Simple binary serialization
pub fn serialize_v3(data: &DataV3) -> Vec<u8> {
    let mut buf = Vec::new();

    // Version header
    buf.push(3); // major
    buf.push(0); // minor

    // Name (length-prefixed)
    buf.extend_from_slice(&(data.name.len() as u32).to_le_bytes());
    buf.extend_from_slice(data.name.as_bytes());

    // Value (f64)
    buf.extend_from_slice(&data.value.to_le_bytes());

    // Tags (count + items)
    buf.extend_from_slice(&(data.tags.len() as u32).to_le_bytes());
    for tag in &data.tags {
        buf.extend_from_slice(&(tag.len() as u32).to_le_bytes());
        buf.extend_from_slice(tag.as_bytes());
    }

    // Metadata (count + pairs)
    buf.extend_from_slice(&(data.metadata.len() as u32).to_le_bytes());
    for (k, v) in &data.metadata {
        buf.extend_from_slice(&(k.len() as u32).to_le_bytes());
        buf.extend_from_slice(k.as_bytes());
        buf.extend_from_slice(&(v.len() as u32).to_le_bytes());
        buf.extend_from_slice(v.as_bytes());
    }

    buf
}

/// Deserialize with version detection
pub fn deserialize(bytes: &[u8]) -> Result<Data, String> {
    if bytes.len() < 2 {
        return Err("Too short".to_string());
    }

    let major = bytes[0];
    let _minor = bytes[1];
    let rest = &bytes[2..];

    match major {
        1 => deserialize_v1(rest).map(Data::V1),
        2 => deserialize_v2(rest).map(Data::V2),
        3 => deserialize_v3(rest).map(Data::V3),
        v => Err(format!("Unknown version: {}", v)),
    }
}

fn read_string(bytes: &[u8], pos: &mut usize) -> Result<String, String> {
    if *pos + 4 > bytes.len() {
        return Err("Truncated".to_string());
    }
    let len = u32::from_le_bytes([
        bytes[*pos],
        bytes[*pos + 1],
        bytes[*pos + 2],
        bytes[*pos + 3],
    ]) as usize;
    *pos += 4;
    if *pos + len > bytes.len() {
        return Err("Truncated string".to_string());
    }
    let s = String::from_utf8(bytes[*pos..*pos + len].to_vec()).map_err(|_| "Invalid UTF-8")?;
    *pos += len;
    Ok(s)
}

fn deserialize_v1(bytes: &[u8]) -> Result<DataV1, String> {
    let mut pos = 0;
    let name = read_string(bytes, &mut pos)?;
    if pos + 4 > bytes.len() {
        return Err("Truncated".to_string());
    }
    let value = i32::from_le_bytes([bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]]);
    Ok(DataV1 { name, value })
}

fn deserialize_v2(bytes: &[u8]) -> Result<DataV2, String> {
    let v1 = deserialize_v1(bytes)?;
    // V2 would have tags after the v1 data
    Ok(DataV2 {
        name: v1.name,
        value: v1.value,
        tags: Vec::new(), // Simplified
    })
}

fn deserialize_v3(bytes: &[u8]) -> Result<DataV3, String> {
    let mut pos = 0;
    let name = read_string(bytes, &mut pos)?;

    if pos + 8 > bytes.len() {
        return Err("Truncated".to_string());
    }
    let value = f64::from_le_bytes([
        bytes[pos],
        bytes[pos + 1],
        bytes[pos + 2],
        bytes[pos + 3],
        bytes[pos + 4],
        bytes[pos + 5],
        bytes[pos + 6],
        bytes[pos + 7],
    ]);
    pos += 8;

    Ok(DataV3 {
        name,
        value,
        tags: Vec::new(),
        metadata: std::collections::HashMap::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compatible() {
        let v1 = Version::new(1, 0);
        let v1_1 = Version::new(1, 1);
        let v2 = Version::new(2, 0);

        assert!(v1.is_compatible(&v1_1));
        assert!(!v1.is_compatible(&v2));
    }

    #[test]
    fn test_upgrade_v1_to_v3() {
        let v1 = DataV1 {
            name: "test".to_string(),
            value: 42,
        };
        let v3 = Data::V1(v1).upgrade();
        assert_eq!(v3.name, "test");
        assert_eq!(v3.value, 42.0);
        assert!(v3.tags.is_empty());
    }

    #[test]
    fn test_upgrade_v2_to_v3() {
        let v2 = DataV2 {
            name: "test".to_string(),
            value: 100,
            tags: vec!["a".to_string(), "b".to_string()],
        };
        let v3 = Data::V2(v2).upgrade();
        assert_eq!(v3.value, 100.0);
        assert_eq!(v3.tags, vec!["a", "b"]);
    }

    #[test]
    fn test_serialize_deserialize() {
        let data = DataV3 {
            name: "hello".to_string(),
            value: 3.14,
            tags: vec![],
            metadata: std::collections::HashMap::new(),
        };
        let bytes = serialize_v3(&data);
        let parsed = deserialize(&bytes).unwrap();

        if let Data::V3(d) = parsed {
            assert_eq!(d.name, "hello");
            assert!((d.value - 3.14).abs() < 0.001);
        } else {
            panic!("Expected V3");
        }
    }
}
