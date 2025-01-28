use crate::moss_definitions::functions::*;
use crate::moss_definitions::types::*;
use extism_pdk::{plugin_fn, FnResult};

pub unsafe fn create_and_open_context_menu() {
    let key = "test_context_menu".to_string();
    moss_gui_register_context_menu(ContextMenu {
        key,
        invert: true,
        pre_loop: None,
        post_loop: None,
        buttons: vec![
            ContextButton {
                text: "Rust SDK tester".to_string(),
                action: None,
                icon: "test_icon".to_string(),
                context_menu: None,
                context_icon: None,
            },
            ContextButton {
                text: "The results of the test can\nbe viewed on the next screen".to_string(),
                action: None,
                icon: "test_icon_blue".to_string(),
                context_menu: None,
                context_icon: None,
            },
            ContextButton {
                text: "View Results".to_string(),
                action: Some("ResultScreen_open_action".to_string()),
                icon: "notebook".to_string(),
                context_menu: None,
                context_icon: Some("chevron_right".to_string()),
            },
        ],
    })
    .unwrap();
    open_context_menu();
}

pub unsafe fn open_context_menu() {
    moss_gui_open_context_menu("test_context_menu", 100, 100).unwrap();
}
