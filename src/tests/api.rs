use crate::{moss_em_config_get, moss_em_config_set, RM_Document, RM_DocumentCollection};
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
    moss_em_config_set::<bool>("test_api_document_get_full", true);

    document
        .metadata
        .set_visible_name("TEST SUCCEEDED!".to_string());

    moss_em_config_set::<bool>("test_api_document_metadata_set", true);

    let mut document_collection = RM_DocumentCollection::get(document_collection_uuid.as_str());
    moss_em_config_set::<bool>("test_api_collection_get_full", true);

    document_collection
        .metadata
        .set_visible_name("TEST SUCCEEDED!".to_string());

    moss_em_config_set::<bool>("test_api_collection_metadata_set", true);
}

pub unsafe fn run_all_api_tests() {
    run_fetch_test();
}
