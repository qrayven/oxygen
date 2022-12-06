// type Result<K> = std::result::Result<K, DocumentError>;
use std::marker::PhantomData;

use serde::Serialize;

use crate::error::Error;

pub struct Unsupported<O> {
    _phantom: PhantomData<O>,
}

impl<O> serde::ser::SerializeSeq for Unsupported<O> {
    type Error = Error;
    type Ok = O;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<O> serde::ser::SerializeTuple for Unsupported<O> {
    type Error = Error;
    type Ok = O;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<O> serde::ser::SerializeTupleVariant for Unsupported<O> {
    type Error = Error;
    type Ok = O;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<O> serde::ser::SerializeStructVariant for Unsupported<O> {
    type Error = Error;
    type Ok = O;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<O> serde::ser::SerializeTupleStruct for Unsupported<O> {
    type Error = Error;
    type Ok = O;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<O> serde::ser::SerializeMap for Unsupported<O> {
    type Error = Error;
    type Ok = O;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<O> serde::ser::SerializeStruct for Unsupported<O> {
    type Error = Error;
    type Ok = O;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}
