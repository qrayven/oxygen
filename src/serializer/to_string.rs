pub struct ToStringSerializer;

use std::fmt::Display;

use serde::Serialize;

use super::unsupported::Unsupported;
use crate::error::Error;

type Result<K> = std::result::Result<K, Error>;

fn key_must_be_a_string() -> Error {
    Error::serialization("key must be an error")
}

impl serde::Serializer for ToStringSerializer {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = Unsupported<String>;
    type SerializeTuple = Unsupported<String>;
    type SerializeTupleStruct = Unsupported<String>;
    type SerializeTupleVariant = Unsupported<String>;
    type SerializeMap = Unsupported<String>;
    type SerializeStruct = Unsupported<String>;
    type SerializeStructVariant = Unsupported<String>;

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<String> {
        Ok(variant.to_owned())
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_bool(self, _value: bool) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_i8(self, value: i8) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_i16(self, value: i16) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_i32(self, value: i32) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_i64(self, value: i64) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_u8(self, value: u8) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_u16(self, value: u16) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_u32(self, value: u32) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_u64(self, value: u64) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_f32(self, _value: f32) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_f64(self, _value: f64) -> Result<String> {
        Err(key_must_be_a_string())
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<String> {
        Ok({
            let mut s = String::new();
            s.push(value);
            s
        })
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<String> {
        Ok(value.to_owned())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit(self) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_none(self) -> Result<String> {
        Err(key_must_be_a_string())
    }

    fn serialize_some<T>(self, _value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(key_must_be_a_string())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(key_must_be_a_string())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(key_must_be_a_string())
    }

    fn collect_str<T>(self, value: &T) -> Result<String>
    where
        T: ?Sized + Display,
    {
        Ok(value.to_string())
    }
}
