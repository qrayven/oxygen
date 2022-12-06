use anyhow::Result;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{
    serializer::ToDashValue,
    types::{DocumentValue, Identifier, StaticBytes, Version},
};

type DataContract = String;
type Metadata = String;

#[derive(Serialize, Deserialize, Debug, Clone, TypedBuilder)]
pub struct Document {
    #[serde(rename = "$protocolVersion", default)]
    #[builder(default = Version(1))]
    protocol_version: Version,

    #[serde(rename = "$id")]
    #[builder(setter(into))]
    id: Identifier,

    #[serde(rename = "$type")]
    document_type: String,

    #[serde(rename = "$revision")]
    #[builder(default = 0)]
    revision: u32,

    #[serde(rename = "$dataContractId")]
    #[builder(setter(into))]
    data_contract_id: Identifier,

    #[serde(rename = "$ownerId")]
    #[builder(setter(into))]
    owner_id: Identifier,

    #[serde(rename = "$createdAt", skip_serializing_if = "Option::is_none")]
    #[builder(default=None)]
    created_at: Option<i64>,

    #[serde(rename = "$updatedAt", skip_serializing_if = "Option::is_none")]
    #[builder(default=None)]
    updated_at: Option<i64>,

    #[serde(flatten)]
    data: DocumentValue,

    #[serde(skip)]
    #[builder(default)]
    data_contract: DataContract,

    #[serde(skip)]
    #[builder(default)]
    metadata: Option<Metadata>,

    #[serde(skip)]
    #[builder(default)]
    entropy: StaticBytes<32>,
}

impl Document {
    pub fn to_json(&self) -> Result<String> {
        let result = serde_json::to_string_pretty(self)?;
        Ok(result)
    }

    pub fn from_json(&self, data: impl AsRef<str>) -> Result<Document> {
        let result: Document = serde_json::from_str(data.as_ref())?;
        Ok(result)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let dynamic_value = self.serialize(ToDashValue::default().with_skip_version(true))?;
        let result = serde_cbor::to_vec(&dynamic_value)?;
        Ok(result)
    }

    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Document> {
        let document: Self = serde_cbor::from_reader(bytes.as_ref())?;
        Ok(document)
    }
}

#[cfg(test)]
mod test {
    use super::Document;
    use crate::types::{DocumentValue, Identifier, Version};

    #[test]
    fn test_document_builder() {
        let document = Document::builder()
            .owner_id(vec![10_u8; 32])
            .id(vec![11_u8; 32])
            .document_type(String::from("something"))
            .data_contract_id(vec![10_u8; 32])
            .data(DocumentValue::Null)
            .build();

        println!("result is {:#?}", document);
    }

    #[test]
    fn test_serialize_to_json() {
        let document_bytes = hex::decode("01000000a7632469645820715d3d65756024a2de0ab1b2bb1e83b5ef297bf0c6fa616aad5c887e4f10def9646e616d656543757469656524747970656c6e696365446f63756d656e7468246f776e657249645820b6bf374d302fbe2b511b43e23d033f965e2e33a024c7419db07533d4ba7d708e69247265766973696f6e016a246372656174656441741b00000181b40fa1fb6f2464617461436f6e7472616374496458207abc5f9ab4bcd0612ed6cacec204dd6d7411a56127d4248af1eadacb93525da2").expect("no error");
        let (_, document_bytes) = document_bytes.split_at(4);

        let document: Document = serde_cbor::from_reader(document_bytes).expect("no error");
        println!("result is {:#?}", document);

        let bytes = document.to_bytes().expect("no error");
        assert_eq!(document_bytes, bytes);
    }
}
