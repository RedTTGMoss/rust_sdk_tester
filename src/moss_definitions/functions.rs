use crate::moss_definitions::types::*;
use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[host_fn]
extern "ExtismHost" {
    // GUI
    pub fn moss_gui_register_context_menu(menu: ContextMenu);
    pub fn moss_gui_invert_icon(key: String, result_key: String);

    // Defaults
    pub fn moss_defaults_set_color(key: String, color: Color);
    pub fn moss_defaults_get_color(key: String) -> Color;
    pub fn moss_defaults_set_text_color(key: String, colors: TextColors);
    pub fn moss_defaults_get_text_color(key: String) -> TextColors;

    pub fn moss_defaults_get<T: for<'de> Deserialize<'de>>(key: &str) -> ConfigGet<T>;
    #[link_name = "moss_defaults_set"]
    fn _moss_defaults_set<T: Serialize>(value: ConfigSet<T>);

    // Extension manager
    pub fn moss_em_config_get<T: for<'de> Deserialize<'de>>(key: &str) -> ConfigGet<T>;
    #[link_name = "moss_em_config_set"]
    fn _moss_em_config_set<T: Serialize>(value: ConfigSet<T>);
}

pub unsafe fn moss_em_config_set<T: Serialize>(key: &str, value: T) {
    let _ = _moss_em_config_set::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    });
}

pub unsafe fn moss_defaults_set<T: Serialize>(key: &str, value: T) {
    let _ = _moss_defaults_set::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    });
}

#[link(wasm_import_module = "extism:host/user")]
extern "C" {
    #[link_name = "moss_gui_open_context_menu"]
    fn moss_gui_open_context_menu_impl(key: u64, x: i64, y: i64) -> ();
}

pub unsafe fn moss_gui_open_context_menu(
    key: &str,
    x: i64,
    y: i64,
) -> Result<(), extism_pdk::Error> {
    let res =
        moss_gui_open_context_menu_impl(extism_pdk::ToMemory::to_memory(&&key)?.offset(), x, y);
    Ok(res)
}
