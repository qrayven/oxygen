use std::{cmp::Ordering, collections::HashMap, ops::Index};

use anyhow::Context;
use itertools::Itertools;
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Serialize,
};

use crate::{
    tri,
    types::{Bytes, Identifier, StaticBytes},
};

#[derive(Clone, Debug, PartialEq)]
pub enum DocumentValue {
    Bool(bool),
    String(String),
    Float(f64),
    Integer(i64),
    UInteger(u64),
    Version(u32),
    Map(HashMap<String, DocumentValue>),
    Array(Vec<DocumentValue>),
    Identifier(Identifier),
    Bytes(Bytes),
    StaticBytes(StaticBytes),
    Null,
}

impl Default for DocumentValue {
    fn default() -> Self {
        DocumentValue::Null
    }
}

impl Serialize for DocumentValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Bool(b) => serializer.serialize_bool(*b),
            Self::String(t) => serializer.serialize_str(t),
            Self::Bytes(b) => b.serialize(serializer),
            Self::StaticBytes(b) => b.serialize(serializer),
            Self::Float(f) => serializer.serialize_f64(*f),
            Self::Integer(i) => serializer.serialize_i64(*i),
            Self::UInteger(u) => serializer.serialize_u64(*u),
            Self::Version(v) => serializer.serialize_u32(*v),
            Self::Identifier(id) => Identifier::serialize(id, serializer),
            Self::Array(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for element in array {
                    seq.serialize_element(element)?;
                }
                seq.end()
            }

            Self::Map(map) => {
                let mut m = serializer.serialize_map(Some(map.len()))?;
                let sorted = map.iter().sorted_by(|a, b| {
                    // We now for sure that the keys are always text, since `insert()`
                    // methods accepts only types that can be converted into a string
                    let key_a = a.0.as_bytes();
                    let key_b = b.0.as_bytes();

                    let len_comparison = key_a.len().cmp(&key_b.len());

                    match len_comparison {
                        Ordering::Less => Ordering::Less,
                        Ordering::Equal => key_a.cmp(key_b),
                        Ordering::Greater => Ordering::Greater,
                    }
                });

                for (key, value) in sorted {
                    m.serialize_entry(&key, &value)?;
                }
                m.end()
            }
            Self::Null => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for DocumentValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = DocumentValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("any valid Dash value")
            }

            // so we could try transform it into something
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(DocumentValue::Bool(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(DocumentValue::Integer(v))
            }

            #[cfg(any(feature = "std", feature = "alloc"))]
            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[cfg(any(feature = "std", feature = "alloc"))]
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
                Ok(DocumentValue::String(value))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(DocumentValue::Null)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(DocumentValue::Null)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(DocumentValue::Bytes(Bytes(v.to_vec())))
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = tri!(visitor.next_element()) {
                    vec.push(elem);
                }
                Ok(DocumentValue::Array(vec))
            }

            #[cfg(any(feature = "std", feature = "alloc"))]
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut map: HashMap<String, DocumentValue> =
                    HashMap::with_capacity(visitor.size_hint().unwrap_or(0));

                while let Some((key, value)) = visitor.next_entry()? {
                    map.insert(key, value);
                }

                Ok(DocumentValue::Map(map))
            }
        }
        deserializer.deserialize_any(ValueVisitor)
    }
}

impl DocumentValue {
    pub fn get<'a, I: Into<DashValueIndex<'a>>>(&self, idx: I) -> Option<&DocumentValue> {
        let index = idx.into();
        match index {
            DashValueIndex::Int(i) => match self {
                DocumentValue::Array(ref a) => a.get(i),
                _ => None,
            },

            DashValueIndex::String(w) => match self {
                DocumentValue::Map(ref map) => map.get(w),
                _ => None,
            },
        }
    }

    pub fn get_mut<'a, I: Into<DashValueIndex<'a>>>(
        &mut self,
        idx: I,
    ) -> Option<&mut DocumentValue> {
        let index = idx.into();
        match index {
            DashValueIndex::Int(i) => match self {
                DocumentValue::Array(ref mut a) => a.get_mut(i),
                _ => None,
            },

            DashValueIndex::String(w) => match self {
                DocumentValue::Map(ref mut map) => map.get_mut(w),
                _ => None,
            },
        }
    }
}

pub enum DashValueIndex<'a> {
    String(&'a str),
    Int(usize),
}

impl<'a> From<&'a str> for DashValueIndex<'a> {
    fn from(v: &'a str) -> Self {
        Self::String(v)
    }
}

impl<'a> From<usize> for DashValueIndex<'a> {
    fn from(v: usize) -> Self {
        Self::Int(v)
    }
}

impl<'a, I> Index<I> for DocumentValue
where
    I: Into<DashValueIndex<'a>>,
{
    type Output = DocumentValue;
    fn index(&self, index: I) -> &Self::Output {
        let index: DashValueIndex = index.into();
        match index {
            DashValueIndex::Int(idx) => match self {
                DocumentValue::Array(arr) => &arr[idx],
                _ => panic!("document value isn't a array"),
            },
            DashValueIndex::String(key) => match self {
                DocumentValue::Map(map) => map.get(key).unwrap(),
                _ => panic!("document isn't a  map"),
            },
        }
    }
}

#[cfg(feature = "serde_json_value")]
impl TryFrom<serde_json::Value> for DocumentValue {
    type Error = anyhow::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        value
            .serialize(crate::serializer::ToDashValue::default())
            .context("conversion of serde json value to Dash value failed")
    }
}

#[cfg(test)]
mod test {
    use super::DocumentValue;
    use serde_json::json;

    #[test]
    fn indexing() {
        let json_value: DocumentValue = json!({
            "alpha" : {
                "bravo" : [
                    "bravo_value"
                ]
            }

        })
        .try_into()
        .expect("no error");

        assert_eq!(
            DocumentValue::String("bravo_value".into()),
            json_value["alpha"]["bravo"][0]
        );
    }
}
