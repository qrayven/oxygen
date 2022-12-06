use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

use crate::types::DocumentValue;

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier {
    pub data: Vec<u8>,
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let id = IdInternal(&self.data);
        serializer.serialize_newtype_struct("identifier", &id)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let id_string: &str = Deserialize::deserialize(deserializer)?;
            let id_bytes = bs58::decode(id_string)
                .into_vec()
                .map_err(|e| serde::de::Error::custom(e.to_string()))?;
            Ok(Self::from(id_bytes))
        } else {
            let data: DocumentValue = Deserialize::deserialize(deserializer)?;
            if let DocumentValue::Bytes(bytes) = data {
                return Ok(Identifier::from(bytes.0));
            }
            Err(serde::de::Error::custom(format!(
                "expected bytes, got: {:?}",
                data
            )))
        }
    }
}

pub(crate) struct IdInternal<'a>(pub &'a [u8]);

impl<'a> Serialize for IdInternal<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&bs58::encode(self.0).into_string())
        } else {
            serializer.serialize_bytes(self.0)
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", bs58::encode(&self.data).into_string())
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "identifier_base58({})",
            bs58::encode(&self.data).into_string()
        )
    }
}

impl From<Vec<u8>> for Identifier {
    fn from(v: Vec<u8>) -> Self {
        Identifier { data: v }
    }
}
