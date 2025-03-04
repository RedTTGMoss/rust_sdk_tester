#![allow(dead_code)]

use extism_pdk::{FromBytes, Json, ToBytes};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

// Document API SUB
static ACCESSOR_API_DOCUMENT_METADATA: &str = "api_document_metadata";
static ACCESSOR_API_DOCUMENT_CONTENT: &str = "api_document_content";

// Collection API SUB
static ACCESSOR_API_COLLECTION_METADATA: &str = "api_collection_metadata";

// API
pub static ACCESSOR_API_DOCUMENT: &str = "api_document";
pub static ACCESSOR_API_COLLECTION: &str = "api_collection";

// Document Standalone SUB
static ACCESSOR_STANDALONE_DOCUMENT_METADATA: &str = "document_metadata";
static ACCESSOR_STANDALONE_DOCUMENT_CONTENT: &str = "document_content";

// Collection Standalone SUB
static ACCESSOR_STANDALONE_COLLECTION_METADATA: &str = "collection_metadata";

// Standalone
pub static ACCESSOR_STANDALONE_DOCUMENT: &str = "document";
pub static ACCESSOR_STANDALONE_COLLECTION: &str = "collection";

static ACCESSOR_STANDALONE_METADATA: &str = "metadata";
static ACCESSOR_STANDALONE_CONTENT: &str = "content";

pub enum AccessorType {
    ApiItem,
    StandaloneItem,
}

pub enum AccessorSubType {
    Document,
    Collection,
}

#[derive(FromBytes, ToBytes, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[encoding(Json)]
pub struct Accessor {
    pub r#type: String,
    pub uuid: Option<String>,
    pub id: Option<i64>,
}

impl Display for Accessor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Accessor(T: {}, U: {:?}, I: {:?})",
            self.r#type, self.uuid, self.id
        )
    }
}

impl Accessor {
    pub fn api_document_metadata(uuid: String) -> Self {
        Self {
            r#type: ACCESSOR_API_DOCUMENT_METADATA.to_string(),
            uuid: Some(uuid),
            id: None,
        }
    }

    pub fn api_document_content(uuid: String) -> Self {
        Self {
            r#type: ACCESSOR_API_DOCUMENT_CONTENT.to_string(),
            uuid: Some(uuid),
            id: None,
        }
    }

    pub fn api_collection_metadata(uuid: String) -> Self {
        Self {
            r#type: ACCESSOR_API_COLLECTION_METADATA.to_string(),
            uuid: Some(uuid),
            id: None,
        }
    }

    pub fn api_document(uuid: String) -> Self {
        Self {
            r#type: ACCESSOR_API_DOCUMENT.to_string(),
            uuid: Some(uuid),
            id: None,
        }
    }

    pub fn api_collection(uuid: String) -> Self {
        Self {
            r#type: ACCESSOR_API_COLLECTION.to_string(),
            uuid: Some(uuid),
            id: None,
        }
    }

    // STANDALONE

    pub fn standalone_document_metadata(uuid: String) -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_DOCUMENT_METADATA.to_string(),
            uuid: Some(uuid),
            id: None,
        }
    }

    pub fn standalone_document_content(uuid: String) -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_DOCUMENT_CONTENT.to_string(),
            uuid: Some(uuid),
            id: None,
        }
    }

    pub fn standalone_collection_metadata(uuid: String) -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_COLLECTION_METADATA.to_string(),
            uuid: Some(uuid),
            id: None,
        }
    }

    pub fn standalone_document(uuid: String) -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_DOCUMENT.to_string(),
            uuid: Some(uuid),
            id: None,
        }
    }

    pub fn standalone_collection(uuid: String) -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_COLLECTION.to_string(),
            uuid: Some(uuid),
            id: None,
        }
    }

    pub fn standalone_metadata(id: i64) -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_METADATA.to_string(),
            uuid: None,
            id: Some(id),
        }
    }

    pub fn standalone_content(id: i64) -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_CONTENT.to_string(),
            uuid: None,
            id: Some(id),
        }
    }

    // UNKNOWN API

    pub fn unknown_api_document_metadata() -> Self {
        Self {
            r#type: ACCESSOR_API_DOCUMENT_METADATA.to_string(),
            uuid: None,
            id: None,
        }
    }

    pub fn unknown_api_document_content() -> Self {
        Self {
            r#type: ACCESSOR_API_DOCUMENT_CONTENT.to_string(),
            uuid: None,
            id: None,
        }
    }

    pub fn unknown_api_collection_metadata() -> Self {
        Self {
            r#type: ACCESSOR_API_COLLECTION_METADATA.to_string(),
            uuid: None,
            id: None,
        }
    }

    pub fn unknown_api_document() -> Self {
        Self {
            r#type: ACCESSOR_API_DOCUMENT.to_string(),
            uuid: None,
            id: None,
        }
    }

    pub fn unknown_api_collection() -> Self {
        Self {
            r#type: ACCESSOR_API_COLLECTION.to_string(),
            uuid: None,
            id: None,
        }
    }

    // UNKNOWN STANDALONE

    pub fn unknown_standalone_document_metadata() -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_DOCUMENT_METADATA.to_string(),
            uuid: None,
            id: None,
        }
    }

    pub fn unknown_standalone_document_content() -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_DOCUMENT_CONTENT.to_string(),
            uuid: None,
            id: None,
        }
    }

    pub fn unknown_standalone_collection_metadata() -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_COLLECTION_METADATA.to_string(),
            uuid: None,
            id: None,
        }
    }

    pub fn unknown_standalone_document() -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_DOCUMENT.to_string(),
            uuid: None,
            id: None,
        }
    }

    pub fn unknown_standalone_collection() -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_COLLECTION.to_string(),
            uuid: None,
            id: None,
        }
    }

    pub fn unknown_standalone_metadata() -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_METADATA.to_string(),
            uuid: None,
            id: None,
        }
    }

    pub fn unknown_standalone_content() -> Self {
        Self {
            r#type: ACCESSOR_STANDALONE_CONTENT.to_string(),
            uuid: None,
            id: None,
        }
    }

    // Extra functions

    pub fn document_metadata(uuid: String, item_type: AccessorType) -> Self {
        match item_type {
            AccessorType::ApiItem => Accessor::api_document_metadata(uuid),
            AccessorType::StandaloneItem => Accessor::standalone_document_metadata(uuid),
        }
    }

    pub fn document_content(uuid: String, item_type: AccessorType) -> Self {
        match item_type {
            AccessorType::ApiItem => Accessor::api_document_content(uuid),
            AccessorType::StandaloneItem => Accessor::standalone_document_content(uuid),
        }
    }

    pub fn collection_metadata(uuid: String, item_type: AccessorType) -> Self {
        match item_type {
            AccessorType::ApiItem => Accessor::api_collection_metadata(uuid),
            AccessorType::StandaloneItem => Accessor::standalone_collection_metadata(uuid),
        }
    }

    pub fn sub_metadata(uuid: String, item_type: AccessorType, sub_type: AccessorSubType) -> Self {
        match sub_type {
            AccessorSubType::Document => Accessor::document_metadata(uuid, item_type),
            AccessorSubType::Collection => Accessor::collection_metadata(uuid, item_type),
        }
    }

    pub fn api_sub_metadata(uuid: String, sub_type: AccessorSubType) -> Self {
        Accessor::sub_metadata(uuid, AccessorType::ApiItem, sub_type)
    }

    pub fn standalone_sub_metadata(uuid: String, sub_type: AccessorSubType) -> Self {
        Accessor::sub_metadata(uuid, AccessorType::StandaloneItem, sub_type)
    }

    pub fn document(uuid: String, item_type: AccessorType) -> Self {
        match item_type {
            AccessorType::ApiItem => Accessor::api_document(uuid),
            AccessorType::StandaloneItem => Accessor::standalone_document(uuid),
        }
    }

    pub fn collection(uuid: String, item_type: AccessorType) -> Self {
        match item_type {
            AccessorType::ApiItem => Accessor::api_collection(uuid),
            AccessorType::StandaloneItem => Accessor::standalone_collection(uuid),
        }
    }

    // Modify

    pub fn new_uuid(&self, uuid: String) -> Self {
        Self {
            r#type: self.r#type.clone(),
            uuid: Some(uuid),
            id: self.id,
        }
    }
}
