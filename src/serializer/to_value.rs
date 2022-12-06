use serde::Serialize;
use std::fmt::Display;

use crate::error::Error;
use crate::types::{DocumentValue as Value, Identifier, Version};

use super::{map::SerializeMap, unsupported::Unsupported, vec::SerializeVec};

type Result<K> = std::result::Result<K, Error>;

#[derive(Default)]
pub struct ToDashValue {
    skip_version: bool,
}

impl ToDashValue {
    pub fn with_skip_version(mut self, ignore_version: bool) -> Self {
        self.skip_version = ignore_version;
        self
    }
}

// Serializer whose output is a `Value`
impl serde::Serializer for ToDashValue {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeVec;
    type SerializeTupleStruct = Unsupported<Value>;
    type SerializeTupleVariant = Unsupported<Value>;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeMap;
    type SerializeStructVariant = Unsupported<Value>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Value> {
        Ok(Value::Bool(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i64(self, value: i64) -> Result<Value> {
        Ok(Value::Integer(value))
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<Value> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<Value> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<Value> {
        self.serialize_u64(value as u64)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<Value> {
        Ok(Value::UInteger(value))
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<Value> {
        self.serialize_f64(value as f64)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<Value> {
        Ok(Value::Float(value))
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Value> {
        let mut s = String::new();
        s.push(value);
        Ok(Value::String(s))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Value> {
        Ok(Value::String(value.to_owned()))
    }

    // ? how to avoid the another allocation?
    fn serialize_bytes(self, value: &[u8]) -> Result<Value> {
        Ok(Value::Bytes(value.to_owned().into()))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Value> {
        Ok(Value::Null)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        if name == "Version" {
            match value.serialize(self)? {
                Value::UInteger(u) => {
                    return Ok(Value::Version(u as u32));
                }
                data => {
                    panic!("expected Value::Bytes, got: {data:#?}")
                }
            }
        }
        if name == "StaticBytes" {
            match value.serialize(self)? {
                Value::StaticBytes(b) => {
                    return Ok(Value::StaticBytes(b));
                }
                data => {
                    panic!("expected Value::StaticBytes, got: {data:#?}")
                }
            }
        }
        if name == "Bytes" {
            match value.serialize(self)? {
                Value::Bytes(b) => {
                    return Ok(Value::Bytes(b));
                }
                data => {
                    panic!("expected Value::Bytes, got: {data:#?}")
                }
            }
        }
        if name == "identifier" {
            match value.serialize(self)? {
                Value::Bytes(b) => {
                    return Ok(Value::Identifier(Identifier { data: b.0 }));
                }
                data => {
                    panic!("expected Value::Bytes, got: {data:#?}")
                }
            }
        }
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::unsupported("new type variant"))
    }

    #[inline]
    fn serialize_none(self) -> Result<Value> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeVec {
            vec: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::unsupported("tuple struct isn't supported yet"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::unsupported("tuple variant isn't supported yet"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap::new(self.skip_version))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::unsupported("struct variant is not supported"))
    }

    fn collect_str<T>(self, value: &T) -> Result<Value>
    where
        T: ?Sized + Display,
    {
        Ok(Value::String(value.to_string()))
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn skip_version() {
        #[derive(Serialize, Deserialize, Debug, Default)]
        struct Example {
            version: Version,
        }

        let example = Example::default();
        let serialized = example
            .serialize(ToDashValue::default().with_skip_version(true))
            .expect("no errors");

        assert!(serialized.get("version").is_none())
    }

    #[test]
    fn keep_version() {
        #[derive(Serialize, Deserialize, Debug, Default)]
        struct Example {
            version: Version,
        }

        let example = Example::default();
        let serialized = example
            .serialize(ToDashValue::default().with_skip_version(false))
            .expect("no errors");

        assert_eq!(Some(&Value::Version(0)), serialized.get("version"))
    }
}
