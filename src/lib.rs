mod moss_definitions;
pub use moss_definitions::functions::*;
pub use moss_definitions::types::*;

mod result_screen;
mod tests;
pub use tests::*;

use crate::result_screen::ResultScreen;
use extism_pdk::*;

#[plugin_fn]
pub unsafe fn moss_extension_register(Json(_state): Json<MossState>) -> FnResult<ExtensionInfo> {
    moss_em_config_set::<bool>("initialized", false);
    moss_em_config_set::<bool>("completed", false);

    run_all_defaults_tests();

    ResultScreen::register();

    moss_em_register_extension_button(ContextButton {
        text: "Rust SDK Tester".to_string(),
        icon: "test_icon".to_string(),
        context_icon: None,
        action: Some("ResultScreen_open_action".to_string()),
        context_menu: Some("open_context_menu_action".to_string()),
    })?;

    warn!("Registering rust SDK tests");
    Ok(ExtensionInfo {
        files: [File {
            key: "test_icon".to_string(),
            path: "extension/assets/test_icon.svg".to_string(),
        }]
        .to_vec(),
    })
}

#[plugin_fn]
pub unsafe fn moss_extension_loop(Json(state): Json<MossState>) -> FnResult<()> {
    let initialized = moss_em_config_get::<bool>("initialized")?.value;
    let loader_progress = moss_em_loader_progress()?.value;

    if !initialized && loader_progress == 1.0 {
        warn!("Initializing rust SDK tests");
        moss_em_config_set::<bool>("initialized", true);
        run_all_tests(&state);
        create_context_menu();
    }

    Ok(())
}

#[plugin_fn]
pub fn moss_extension_unregister() -> FnResult<()> {
    Ok(())
}
