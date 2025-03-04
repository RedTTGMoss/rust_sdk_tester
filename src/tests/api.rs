use crate::{
    moss_em_config_get, moss_em_config_set, Base64VecU8, DocumentNewEPUBBuilder,
    DocumentNewNotebookBuilder, DocumentNewPDFBuilder, MetadataNewBuilder, RM_Content, RM_Document,
    RM_DocumentCollection, RM_Metadata,
};
use extism_pdk::{error, plugin_fn, Error, FnResult};
use std::fs::File;

const DOCUMENT_UUID_ERROR: &str = "Test document not found, please check config";
const DOCUMENT_UUID_KEY: &str = "test_document_uuid";
const DOCUMENT_COLLECTION_UUID_ERROR: &str =
    "Test document collection not found, please check config";
const DOCUMENT_COLLECTION_UUID_KEY: &str = "test_document_collection_uuid";

pub unsafe fn run_existing_document_test() -> Result<RM_Document, String> {
    let document_uuid;
    let _document_uuid = moss_em_config_get::<String>(DOCUMENT_UUID_KEY);
    if _document_uuid.is_err() {
        moss_em_config_set::<String>(DOCUMENT_UUID_KEY, "".to_string());
        return Err(DOCUMENT_UUID_ERROR.to_string());
    } else {
        document_uuid = _document_uuid.unwrap().value;
        if document_uuid.is_empty() {
            return Err(DOCUMENT_UUID_ERROR.to_string());
        }
    }

    let mut document = RM_Document::get_api(document_uuid.clone())
        .map_err(|e| format!("Error retrieving document collection: {:?}", e))?;
    document
        .metadata
        .set_visible_name("TEST SUCCEEDED 1!".to_string());
    document.content.set_usable(true);
    document.set_provision(true);
    assert_eq!(document.provision, document.get_provision().unwrap().value);

    let mut document_metadata = RM_Metadata::get_from_api_document(document_uuid.clone()).unwrap();
    let mut document_content = RM_Content::get_from_api_document(document_uuid.clone()).unwrap();
    document_metadata.set_visible_name("TEST SUCCEEDED 2!".to_string());
    document_content.set_usable(false);
    assert_eq!(
        document_metadata.visible_name,
        document_metadata.get_visible_name().unwrap().value
    );
    assert_eq!(
        document_content.usable,
        document_content.get_usable().unwrap().value
    );
    Ok(document)
}

pub unsafe fn run_existing_collection_test() -> Result<Option<String>, String> {
    let document_collection_uuid;
    let _document_collection_uuid = moss_em_config_get::<String>(DOCUMENT_COLLECTION_UUID_KEY);
    if _document_collection_uuid.is_err() {
        moss_em_config_set::<String>(DOCUMENT_COLLECTION_UUID_KEY, "".to_string());
        return Err(DOCUMENT_COLLECTION_UUID_ERROR.to_string());
    } else {
        document_collection_uuid = _document_collection_uuid.unwrap().value;
        if document_collection_uuid.is_empty() {
            return Err(DOCUMENT_COLLECTION_UUID_ERROR.to_string());
        }
    }

    let mut document_collection = RM_DocumentCollection::get(document_collection_uuid.clone())
        .map_err(|e| format!("Error retrieving document collection: {:?}", e))?;
    document_collection
        .metadata
        .set_visible_name("TEST SUCCEEDED 1!".to_string());
    document_collection.set_has_items(true);
    assert_eq!(
        document_collection.has_items,
        document_collection.get_has_items().unwrap().value
    );

    let mut collection_metadata =
        RM_Metadata::get_from_api_collection(document_collection_uuid.clone()).unwrap();
    collection_metadata.set_visible_name("TEST SUCCEEDED 2!".to_string());
    assert_eq!(
        collection_metadata.visible_name.clone(),
        collection_metadata.get_visible_name().unwrap().value
    );
    Ok(document_collection.metadata.parent)
}

pub unsafe fn run_fetch_test() -> (Option<RM_Document>, Option<String>) {
    let mut test_document = None;
    let mut api_test_folder = None;

    match run_existing_document_test() {
        Ok(document) => {
            test_document = Some(document);
        }
        Err(e) => {
            error!("Document test failed: {}", e);
        }
    }
    match run_existing_collection_test() {
        Ok(parent) => {
            api_test_folder = parent;
        }
        Err(e) => {
            error!("Collection test failed: {}", e);
        }
    }
    (test_document, api_test_folder)
}

pub unsafe fn run_new_notebook_test(api_test_folder: Option<String>) -> Result<(), Error> {
    match RM_Document::new_notebook(
        DocumentNewNotebookBuilder::default()
            .name("Test Notebook".to_string())
            .parent(api_test_folder),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
pub unsafe fn run_new_pdf_test(api_test_folder: Option<String>) -> Result<(), Error> {
    match RM_Document::new_pdf(
        DocumentNewPDFBuilder::default()
            .name("Test PDF".to_string())
            .pdf_file(Some("extension/assets/test_pdf.pdf".to_string()))
            .parent(api_test_folder),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
pub unsafe fn run_new_epub_test(api_test_folder: Option<String>) -> Result<(), Error> {
    let epub_file = File::open("extension/assets/test_epub.epub").unwrap();
    let epub_reader = std::io::BufReader::new(epub_file);

    match RM_Document::new_epub(
        DocumentNewEPUBBuilder::default()
            .name("Test EPUB".to_string())
            .epub_data(Some(Base64VecU8::from_reader(epub_reader)))
            .parent(api_test_folder),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub unsafe fn run_new_documents_test(api_test_folder: Option<String>) {
    match run_new_notebook_test(api_test_folder.clone()) {
        Err(e) => {
            error!("Document new notebook failed: {}", e);
        }
        _ => {}
    }
    match run_new_pdf_test(api_test_folder.clone()) {
        Err(e) => {
            error!("Document new PDF failed: {}", e);
        }
        _ => {}
    }
    match run_new_epub_test(api_test_folder.clone()) {
        Err(e) => {
            error!("Document new EPUB failed: {}", e);
        }
        _ => {}
    }
}

pub unsafe fn _test_document_functions(document_uuid: String) -> FnResult<()> {
    let document = RM_Document::get_api(document_uuid.clone())?;
    document.unload_files();
    document.ensure_download();
    document.unload_files();
    document.load_files_from_cache();
    document.unload_files();
    match document.duplicate() {
        Ok(mut duplicate) => {
            duplicate
                .metadata
                .set_visible_name("TEST SUCCEEDED DUPLICATE!".to_string());
        }
        Err(e) => {
            error!("Document duplicate failed: {}", e);
        }
    };

    match document.randomize_uuids() {
        Ok(mut modified_document) => {
            modified_document.export();
            modified_document
                .metadata
                .set_visible_name("TEST SUCCEEDED 3!".to_string());
        }
        Err(e) => {
            error!("Document randomize uuids failed: {}", e);
        }
    }

    Ok(())
}

#[plugin_fn]
pub unsafe fn test_download_callback(document_uuid: String) -> FnResult<()> {
    let result = _test_document_functions(document_uuid);
    moss_em_config_set::<bool>("download_callback_called", true);
    result
}

pub unsafe fn run_download_callback_test(document: RM_Document) {
    document.ensure_download_and_callback("test_download_callback");
}

pub unsafe fn run_new_metadata_test() {
    match RM_Metadata::new(MetadataNewBuilder::default().name("Test Metadata".to_string())) {
        Ok(mut metadata) => {
            metadata.set_visible_name("TEST SUCCEEDED 4!".to_string());
        }
        Err(e) => {
            error!("New metadata creation failed: {}", e);
        }
    }
}

pub unsafe fn run_all_api_tests() {
    let (test_document, api_test_folder) = run_fetch_test();
    run_new_documents_test(api_test_folder);
    if let Some(document) = test_document {
        run_download_callback_test(document);
    }
    run_new_metadata_test();
}
