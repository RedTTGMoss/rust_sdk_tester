use extism_pdk::info;
use crate::{moss_em_config_set, RM_Document};

pub unsafe fn run_fetch_test() {
    let document = RM_Document::get("d1b68662-b2ad-4fc8-8565-c3c5ed7a16f4");
    moss_em_config_set::<bool>("test_api_document_get_full", true);
}

pub unsafe fn run_all_api_tests() {
    run_fetch_test();
}