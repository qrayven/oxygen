mod error;
mod example;
pub mod serializer;
pub mod types;

pub use error::Error;
use std::ops::Deref;

mod prelude {
    pub use super::types::*;
    pub use crate::error::Error;
}

// We only use our own error type; no need for From conversions provided by the
// standard library's try! macro. This reduces lines of LLVM IR by 4%.
#[macro_export]
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

#[cfg(test)]
mod test {
    use crate::{serializer::ToDashValue, types::*};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[test]
    fn test_deserialize_from_json_str() {
        #[derive(Serialize, Deserialize)]
        struct ExampleDoc {
            id: String,
            cost: i64,
        }
    }

    #[test]
    fn test_serializer() {
        #[derive(Serialize, Deserialize, Default)]
        #[serde(rename_all = "camelCase")]
        struct ExampleStruct {
            id: Identifier,
            data_contract_id: Identifier,
            cost: i64,
            binary_data: Bytes,
            nested: Inner,
            #[serde(flatten)]
            data: DocumentValue,
        }
        let mut dynamic_data: HashMap<String, DocumentValue> = HashMap::new();
        dynamic_data.insert(
            String::from("dynamic_bytes"),
            DocumentValue::Bytes(vec![2u8; 32].into()),
        );
        dynamic_data.insert(
            String::from("dynamic_id"),
            DocumentValue::Identifier(Identifier::default()),
        );

        #[derive(Serialize, Debug, Deserialize, Default)]
        struct Inner {
            id: Identifier,
        }

        let example = ExampleStruct {
            id: Identifier {
                data: vec![0u8; 32],
            },
            binary_data: Bytes(vec![2u8; 32]),
            data: DocumentValue::Map(dynamic_data),
            ..Default::default()
        };

        let result = example
            .serialize(ToDashValue::default())
            .expect("dash value error");
        println!("result is {:#?}", result);

        let data = serde_json::to_string_pretty(&result).expect("json error");
        println!("json result is {}", data);

        let bytes = serde_cbor::to_vec(&result).expect("cbor error");
        println!("cbor result is {:?}", bytes);

        let bytes = serde_json::to_string_pretty(&example).expect("cbor error");
        println!("direct JSON is {}", bytes);
    }

    #[test]
    fn test_document_values() {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ExampleDocument {
            id: String,

            #[serde(flatten)]
            data: DocumentValue,
        }

        let dynamic_a: HashMap<String, DocumentValue> = vec![
            (
                String::from("property_a"),
                DocumentValue::String(String::from("value_a")),
            ),
            (
                String::from("property_b"),
                DocumentValue::String(String::from("value_b")),
            ),
        ]
        .into_iter()
        .collect();

        let dynamic_b: HashMap<String, DocumentValue> = vec![
            (
                String::from("property_b"),
                DocumentValue::String(String::from("value_b")),
            ),
            (
                String::from("property_a"),
                DocumentValue::String(String::from("value_a")),
            ),
        ]
        .into_iter()
        .collect();

        let d = ExampleDocument {
            id: String::from("aaa"),
            data: DocumentValue::Map(dynamic_a),
        };
        let d2 = ExampleDocument {
            id: String::from("aaa"),
            data: DocumentValue::Map(dynamic_b),
        };

        let data = serde_json::to_string_pretty(&d).unwrap();
        let bytes = serde_cbor::to_vec(&d).unwrap();
        let bytes_2 = serde_cbor::to_vec(&d2).unwrap();

        println!("result is {data}");
        assert_eq!(bytes, bytes_2);
    }
}

struct CustomJsonSerializer(pub serde_json::value::Serializer);

impl Deref for CustomJsonSerializer {
    type Target = serde_json::value::Serializer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
