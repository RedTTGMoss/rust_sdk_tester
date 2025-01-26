mod moss_definitions;
pub use moss_definitions::functions::*;
pub use moss_definitions::types::*;

mod tests;
pub use tests::*;

use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[plugin_fn]
pub unsafe fn moss_extension_register(Json(state): Json<MossState>) -> FnResult<ExtensionInfo> {
    moss_em_config_set::<bool>("initialized", false);
    moss_em_config_set::<bool>("icon_loaded", false);
    moss_em_config_set::<bool>("icon_inverted", false);
    moss_em_config_set::<bool>("test_defaults_set<String>", false);
    moss_em_config_set::<bool>("test_defaults_set<TestTypes>", false);
    moss_em_config_set::<bool>("test_defaults_set_color_no_alpha", false);
    moss_em_config_set::<bool>("test_defaults_set_color_with_alpha", false);
    moss_em_config_set::<bool>("test_defaults_set_text_color_no_alpha_no_background", false);
    moss_em_config_set::<bool>(
        "test_defaults_set_text_color_no_alpha_with_background_no_alpha",
        false,
    );
    moss_em_config_set::<bool>(
        "test_defaults_set_text_color_no_alpha_with_background_with_alpha",
        false,
    );
    moss_em_config_set::<bool>(
        "test_defaults_set_text_color_with_alpha_no_background",
        false,
    );
    moss_em_config_set::<bool>(
        "test_defaults_set_text_color_with_alpha_with_background_no_alpha",
        false,
    );
    moss_em_config_set::<bool>(
        "test_defaults_set_text_color_with_alpha_with_background_with_alpha",
        false,
    );

    run_all_defaults_tests();

    warn!("Registering rust SDK tests");
    Ok(ExtensionInfo {
        files: [File {
            key: "test_icon".to_string(),
            path: "assets/test_icon.svg".to_string(),
        }]
        .to_vec(),
    })
}

#[plugin_fn]
pub unsafe fn moss_extension_loop(Json(state): Json<MossState>) -> FnResult<()> {
    let initialized = moss_em_config_get::<bool>("initialized")?.value;

    if !initialized {
        warn!("Initializing rust SDK tests");
        moss_em_config_set::<bool>("initialized", true);
        run_all_tests(&state);
        create_and_open_context_menu();
    }

    let mut opened = false;
    for context_menu in state.opened_context_menus.iter() {
        if context_menu == "test_context_menu" {
            opened = true;
            break;
        }
    }
    if !opened {
        open_context_menu();
    }

    Ok(())
}

#[plugin_fn]
pub fn moss_extension_unregister() -> FnResult<()> {
    Ok(())
}
