type Result<K> = std::result::Result<K, Error>;
use std::collections::HashMap;

use serde::Serialize;

use super::to_string::ToStringSerializer;
use super::to_value::ToDashValue;

use crate::error::Error;
use crate::types::DocumentValue as Value;

#[derive(Default)]
pub struct SerializeMap {
    skip_version: bool,
    map: HashMap<String, Value>,
    next_key: Option<String>,
}

impl SerializeMap {
    pub fn new(ignore_version: bool) -> Self {
        Self {
            skip_version: ignore_version,
            ..Default::default()
        }
    }
}

impl serde::ser::SerializeMap for SerializeMap {
    type Error = Error;
    type Ok = Value;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.next_key = Some(key.serialize(ToStringSerializer)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        let key = self.next_key.take();
        // Panic because this indicates a bug in the program rather than an
        // expected failure.
        let key = key.expect("serialize_value called before serialize_key");
        let new_value = value.serialize(ToDashValue::default())?;

        if matches!(new_value, Value::Version(_)) && self.skip_version {
            return Ok(());
        }

        self.map.insert(key, new_value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Value::Map(self.map))
    }
}

impl serde::ser::SerializeStruct for SerializeMap {
    type Error = Error;
    type Ok = Value;
    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        serde::ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Value::Map(self.map))
    }
}
