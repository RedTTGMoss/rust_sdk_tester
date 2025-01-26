use extism_pdk::{info, FromBytes, ToBytes, Json};
use serde::{Deserialize, Serialize};
use crate::moss_definitions::functions::*;
use crate::tests::colors::*;

const FONT_KEYS: [&str; 19] = [
    "CUSTOM_FONT",
    "CUSTOM_FONT_BOLD",
    "MONO_FONT",
    "ROBOTO_REGULAR_FONT",
    "ROBOTO_MEDIUM_FONT",
    "TITLE_FONT",
    //
    "PATH_FONT",
    "FOLDER_TITLE_FONT",
    "DOCUMENT_TITLE_FONT",
    "DOCUMENT_ERROR_FONT",
    "INSTALLER_FONT",
    "BUTTON_FONT",
    //
    "LOGO_FONT",
    "MAIN_MENU_FONT",
    "MAIN_MENU_BAR_FONT",
    "MAIN_MENU_PROGRESS_FONT",
    "CODE_FONT",
    "DEBUG_FONT",
    "GUIDES_FONT",
];

#[derive(ToBytes, FromBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
struct TestTypes {
    test_bool: bool,
    test_i64: i64,
    test_f64: f64,
    test_string: String,
}

pub unsafe fn run_font_test() {
    let extensions_dir = moss_defaults_get::<String>("EXTENSIONS_DIR").unwrap().value;
    let font = format!("{}/rust_sdk_tester/assets/font.ttf", extensions_dir);
    info!("Font path: {}", font);
    for key in FONT_KEYS.iter() {
        let original_font = moss_defaults_get::<String>(key).unwrap().value;
        moss_defaults_set::<String>(key, font.clone());
        let new_font = moss_defaults_get::<String>(key).unwrap().value;
        moss_em_config_set("test_defaults_set<String>", new_font == font);
        moss_defaults_set::<String>(key, original_font);
    }
}

pub unsafe fn run_types_test() {
    let test_types = TestTypes {
        test_bool: true,
        test_i64: 42,
        test_f64: 3.14,
        test_string: "Hello, World!".to_string(),
    };
    moss_defaults_set::<TestTypes>("test_types", test_types.clone());
    let result_types = moss_defaults_get::<TestTypes>("test_types").unwrap().value;
    moss_em_config_set::<bool>("test_defaults_set<TestTypes>", test_types == result_types);
}

pub unsafe fn run_all_defaults_tests() {
    run_font_test();
    run_types_test();
    run_all_color_tests()
}
