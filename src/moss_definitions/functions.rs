use crate::moss_definitions::types::*;
use extism_pdk::json::to_vec;
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
    pub fn moss_em_get_state() -> MossState;
    pub fn moss_em_register_extension_button(button: ContextButton);

    // PygameExtra
    #[link_name = "moss_pe_draw_rect"]
    fn _moss_pe_draw_rect(draw: PygameExtraRect);

    // Screens
    pub fn moss_pe_register_screen(screen: MossScreen);
    #[link_name = "moss_pe_open_screen"]
    fn _moss_pe_open_screen(key: String, initial_values: Vec<u8>);

    pub fn moss_pe_get_screen_value<T: for<'de> Deserialize<'de>>(key: &str) -> ConfigGet<T>;
    #[link_name = "moss_pe_set_screen_value"]
    fn _moss_pe_set_screen_value<T: Serialize>(value: ConfigSet<T>);
}

pub unsafe fn moss_em_config_set<T: Serialize>(key: &str, value: T) {
    let _ = _moss_em_config_set::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    });
}
pub unsafe fn moss_pe_set_screen_value<T: Serialize>(key: &str, value: T) {
    let _ = _moss_pe_set_screen_value::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    });
}

pub unsafe fn moss_pe_open_screen<T: Serialize>(
    key: &str,
    initial_values: T,
) -> Result<(), extism_pdk::Error> {
    let serialized_values = to_vec(&initial_values)?;
    _moss_pe_open_screen(key.into(), serialized_values)
}

pub unsafe fn moss_defaults_set<T: Serialize>(key: &str, value: T) {
    let _ = _moss_defaults_set::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    });
}

pub unsafe fn moss_pe_draw_rect(
    color: Color,
    rect: Rect,
    width: i64,
    edge_rounding: Option<PygameExtraRectEdgeRounding>,
) {
    let _ = _moss_pe_draw_rect(PygameExtraRect {
        color,
        rect,
        width,
        edge_rounding,
    });
}

#[link(wasm_import_module = "extism:host/user")]
extern "C" {
    #[link_name = "moss_gui_open_context_menu"]
    fn moss_gui_open_context_menu_impl(key: u64, x: i64, y: i64) -> ();

    #[link_name = "moss_text_make"]
    pub fn moss_text_make_impl(text: u64, font: u64, font_size: i64, colors: u64) -> u64;
    #[link_name = "moss_text_get_rect"]
    pub fn moss_text_get_rect_impl(text_id: i64) -> u64;

    #[link_name = "moss_text_set_rect"]
    pub fn moss_text_set_rect_impl(text_id: i64, rect: u64) -> ();

    #[link_name = "moss_text_set_text"]
    pub fn moss_text_set_text_impl(text_id: i64, text: u64) -> u64;

    #[link_name = "moss_text_set_font"]
    pub fn moss_text_set_font_impl(text_id: i64, font: u64, font_size: i64) -> u64;

    #[link_name = "moss_text_display"]
    pub fn moss_text_display_impl(text_id: i64) -> ();
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

pub unsafe fn moss_text_make(
    text: &str,
    font: &str,
    font_size: i64,
    colors: TextColors,
) -> Result<i64, extism_pdk::Error> {
    let res = extism_pdk::Memory::from(moss_text_make_impl(
        extism_pdk::ToMemory::to_memory(&&text)?.offset(),
        extism_pdk::ToMemory::to_memory(&&font)?.offset(),
        font_size,
        extism_pdk::ToMemory::to_memory(&&colors)?.offset(),
    ));
    <ConfigGet<i64> as extism_pdk::FromBytes>::from_bytes(&res.to_vec()).map(|x| x.value)
}

pub unsafe fn moss_text_get_rect(text_id: i64) -> Result<Rect, Error> {
    let res = extism_pdk::Memory::from(moss_text_get_rect_impl(text_id));

    <Rect as extism_pdk::FromBytes>::from_bytes(&res.to_vec())
}

pub unsafe fn moss_text_set_rect(text_id: i64, rect: &Rect) -> Result<(), Error> {
    let res = moss_text_set_rect_impl(text_id, extism_pdk::ToMemory::to_memory(&&rect)?.offset());

    Ok(res)
}
pub unsafe fn moss_text_set_text(text_id: i64, text: &str) -> Result<Rect, Error> {
    let res = extism_pdk::Memory::from(moss_text_set_text_impl(
        text_id,
        extism_pdk::ToMemory::to_memory(&&text)?.offset(),
    ));

    <Rect as extism_pdk::FromBytes>::from_bytes(&res.to_vec())
}
pub unsafe fn moss_text_set_font(text_id: i64, font: &str, font_size: i64) -> Result<Rect, Error> {
    let res = extism_pdk::Memory::from(moss_text_set_font_impl(
        text_id,
        extism_pdk::ToMemory::to_memory(&&font)?.offset(),
        font_size,
    ));

    <Rect as extism_pdk::FromBytes>::from_bytes(&res.to_vec())
}

pub unsafe fn moss_text_display(text_id: i64) -> Result<(), Error> {
    let res = moss_text_display_impl(text_id);
    Ok(res)
}
