use serde::Serialize;

use super::to_value::ToDashValue;
use crate::error::Error;
use crate::types::DocumentValue as Value;

type Result<K> = std::result::Result<K, Error>;

pub struct SerializeVec {
    pub vec: Vec<Value>,
}

impl serde::ser::SerializeSeq for SerializeVec {
    type Ok = Value;
    type Error = Error;

    // in case of sequence we use serializer
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(value.serialize(ToDashValue::default())?);
        Ok(())
    }

    fn end(self) -> Result<Value> {
        Ok(Value::Array(self.vec))
    }
}

impl serde::ser::SerializeTuple for SerializeVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }
}
