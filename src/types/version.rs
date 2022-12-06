use serde::{Deserialize, Serialize, Serializer};

/// Type wrapper for version. For binary formats the version is omitted
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub u32);

impl From<u32> for Version {
    fn from(v: u32) -> Self {
        Version(v)
    }
}

impl From<u8> for Version {
    fn from(v: u8) -> Self {
        Version(v as u32)
    }
}
