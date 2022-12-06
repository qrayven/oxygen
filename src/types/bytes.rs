use std::{
    fmt::{write, Debug, Display},
    ops::Deref,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes(pub Vec<u8>);

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&base64::encode(&self.0))
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

impl Deref for Bytes {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", base64::encode(&self.0))
    }
}

impl<const S: usize> From<[u8; S]> for Bytes {
    fn from(d: [u8; S]) -> Self {
        Bytes(d.to_vec())
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(v: Vec<u8>) -> Self {
        Bytes(v)
    }
}

impl Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bytes_base64({})", base64::encode(&self.0))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StaticBytes<const N: usize = 32>(pub [u8; N]);

impl<const N: usize> Serialize for StaticBytes<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&base64::encode(self.0))
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

impl<const N: usize> Deref for StaticBytes<N> {
    type Target = [u8; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Display for StaticBytes<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", base64::encode(self.0))
    }
}

impl<const S: usize> From<[u8; S]> for StaticBytes<S> {
    fn from(d: [u8; S]) -> Self {
        StaticBytes(d)
    }
}

impl<const S: usize> Debug for StaticBytes<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bytes_base64({})", base64::encode(self.0))
    }
}

impl<const N: usize> Default for StaticBytes<N> {
    fn default() -> Self {
        StaticBytes([0_u8; N])
    }
}
