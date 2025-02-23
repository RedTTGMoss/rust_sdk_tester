use crate::{
    moss_api_collection_metadata_get_all, moss_api_document_metadata_get_all, moss_em_config_get,
    moss_em_config_set, RM_Document, RM_DocumentCollection,
};
const DOCUMENT_UUID_ERROR: &str = "Test document not found, please check config";
const DOCUMENT_UUID_KEY: &str = "test_document_uuid";
const DOCUMENT_COLLECTION_UUID_ERROR: &str =
    "Test document collection not found, please check config";
const DOCUMENT_COLLECTION_UUID_KEY: &str = "test_document_collection_uuid";

pub unsafe fn run_fetch_test() {
    let document_uuid;
    let _document_uuid = moss_em_config_get::<String>(DOCUMENT_UUID_KEY);
    if _document_uuid.is_err() {
        moss_em_config_set::<String>(DOCUMENT_UUID_KEY, "".to_string());
        panic!("{}", DOCUMENT_UUID_ERROR);
    } else {
        document_uuid = _document_uuid.unwrap().value;
        if document_uuid.is_empty() {
            panic!("{}", DOCUMENT_UUID_ERROR);
        }
    }
    let document_collection_uuid;
    let _document_collection_uuid = moss_em_config_get::<String>(DOCUMENT_COLLECTION_UUID_KEY);
    if _document_collection_uuid.is_err() {
        moss_em_config_set::<String>(DOCUMENT_COLLECTION_UUID_KEY, "".to_string());
        panic!("{}", DOCUMENT_COLLECTION_UUID_ERROR);
    } else {
        document_collection_uuid = _document_collection_uuid.unwrap().value;
        if document_collection_uuid.is_empty() {
            panic!("{}", DOCUMENT_COLLECTION_UUID_ERROR);
        }
    }

    let mut document = RM_Document::get(document_uuid.as_str());
    document
        .metadata
        .set_visible_name("TEST SUCCEEDED 1!".to_string());

    let mut document_metadata = moss_api_document_metadata_get_all(document_uuid.as_str()).unwrap();
    document_metadata.set_visible_name("TEST SUCCEEDED 2!".to_string());

    let mut document_collection = RM_DocumentCollection::get(document_collection_uuid.as_str());
    document_collection
        .metadata
        .set_visible_name("TEST SUCCEEDED 1!".to_string());

    let mut collection_metadata =
        moss_api_collection_metadata_get_all(document_collection_uuid.as_str()).unwrap();
    collection_metadata.set_visible_name("TEST SUCCEEDED 2!".to_string());
}

pub unsafe fn run_all_api_tests() {
    run_fetch_test();
}
