use crate::moss_definitions::types::*;
use extism_pdk::json::to_vec;
use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

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
    pub fn moss_em_export_statistical_data();
    pub fn moss_em_register_extension_button(button: ContextButton);

    pub fn moss_em_loader_progress() -> ConfigGet<f64>;

    // PygameExtra
    #[link_name = "moss_pe_draw_rect"]
    fn _moss_pe_draw_rect(draw: PygameExtraRect);

    // Screens
    pub fn moss_pe_register_screen(screen: MossScreen);
    #[link_name = "moss_pe_open_screen"]
    fn _moss_pe_open_screen(key: String, initial_values: Vec<u8>) -> i64;
    pub fn moss_pe_close_screen();

    pub fn moss_pe_get_screen_value<T: for<'de> Deserialize<'de>>(key: &str) -> ConfigGet<T>;
    #[link_name = "moss_pe_set_screen_value"]
    fn _moss_pe_set_screen_value<T: Serialize>(value: ConfigSet<T>);

    // API
    pub fn moss_api_document_get<T: for<'de> Deserialize<'de>>(
        document_uuid: &str,
        key: &str,
    ) -> ConfigGet<T>;
    #[link_name = "moss_api_document_set"]
    pub fn _moss_api_document_set<T: Serialize>(document_uuid: &str, value: ConfigSet<T>);
    pub fn moss_api_document_get_all(document_uuid: &str) -> RM_Document;
    pub fn moss_api_collection_get<T: for<'de> Deserialize<'de>>(
        collection_uuid: &str,
        key: &str,
    ) -> ConfigGet<T>;
    #[link_name = "moss_api_collection_set"]
    pub fn _moss_api_collection_set<T: Serialize>(collection_uuid: &str, value: ConfigSet<T>);
    pub fn moss_api_collection_get_all(collection_uuid: &str) -> RM_DocumentCollection;
    pub fn moss_api_collection_metadata_get<T: for<'de> Deserialize<'de>>(
        collection_uuid: &str,
        key: &str,
    ) -> ConfigGet<T>;
    #[link_name = "moss_api_collection_metadata_set"]
    pub fn _moss_api_collection_metadata_set<T: Serialize>(
        collection_uuid: &str,
        value: ConfigSet<T>,
    );
    pub fn moss_api_collection_metadata_get_all(collection_uuid: &str) -> RM_Metadata;
    pub fn moss_api_document_metadata_get<T: for<'de> Deserialize<'de>>(
        document_uuid: &str,
        key: &str,
    ) -> ConfigGet<T>;
    #[link_name = "moss_api_document_metadata_set"]
    pub fn _moss_api_document_metadata_set<T: Serialize>(document_uuid: &str, value: ConfigSet<T>);
    pub fn moss_api_document_metadata_get_all(document_uuid: &str) -> RM_Metadata;
    pub fn moss_api_metadata_get<T: for<'de> Deserialize<'de>>(
        metadata_id: &i64,
        key: &str,
    ) -> ConfigGet<T>;
    #[link_name = "moss_api_metadata_set"]
    pub fn _moss_api_metadata_set<T: Serialize>(metadata_id: &i64, value: ConfigSet<T>);
    pub fn moss_api_metadata_get_all(metadata_id: &i64) -> RM_Metadata;
    pub fn moss_api_document_content_get<T: for<'de> Deserialize<'de>>(
        document_uuid: &str,
        key: &str,
    ) -> ConfigGet<T>;
    #[link_name = "moss_api_document_content_set"]
    pub fn _moss_api_document_content_set<T: Serialize>(document_uuid: &str, value: ConfigSet<T>);
    pub fn moss_api_document_content_get_all(document_uuid: &str) -> RM_Content;
    pub fn moss_api_content_get<T: for<'de> Deserialize<'de>>(
        content_id: &i64,
        key: &str,
    ) -> ConfigGet<T>;
    #[link_name = "moss_api_content_set"]
    pub fn _moss_api_content_set<T: Serialize>(content_id: &i64, value: ConfigSet<T>);
    pub fn moss_api_content_get_all(content_id: &str) -> RM_Content;

    // Document host functions
    pub fn moss_api_document_new_notebook(value: DocumentNewNotebook) -> String;
    pub fn moss_api_document_new_pdf(value: DocumentNewPDF) -> String;
    pub fn moss_api_document_new_epub(value: DocumentNewEPUB) -> String;

    pub fn moss_api_document_duplicate(document_uuid: &str) -> String;
    pub fn moss_api_document_randomize_uuids(document_uuid: &str) -> String;
    pub fn moss_api_document_unload_files(document_uuid: &str);
    pub fn moss_api_document_load_files_from_cache(document_uuid: &str);
    pub fn moss_api_document_ensure_download_and_callback(document_uuid: &str, callback: &str);
    pub fn moss_api_document_ensure_download(document_uuid: &str);
    pub fn moss_api_document_export(document_uuid: &str);
}

pub unsafe fn moss_em_config_set<T: Serialize>(key: &str, value: T) {
    _moss_em_config_set::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    })
    .unwrap();
}
pub unsafe fn moss_pe_set_screen_value<T: Serialize>(key: &str, value: T) {
    _moss_pe_set_screen_value::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    })
    .unwrap();
}

pub unsafe fn moss_pe_open_screen<T: Serialize>(
    key: &str,
    initial_values: T,
) -> Result<i64, extism_pdk::Error> {
    let serialized_values = to_vec(&initial_values)?;
    _moss_pe_open_screen(key.into(), serialized_values)
}

pub unsafe fn moss_defaults_set<T: Serialize>(key: &str, value: T) {
    _moss_defaults_set::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    })
    .unwrap();
}

pub unsafe fn moss_pe_draw_rect(
    color: &Color,
    rect: &Rect,
    width: i64,
    edge_rounding: Option<PygameExtraRectEdgeRounding>,
) {
    _moss_pe_draw_rect(PygameExtraRect {
        color: color.to_owned(),
        rect: rect.to_owned(),
        width,
        edge_rounding,
    })
    .unwrap();
}

pub unsafe fn moss_api_document_metadata_set<T: Serialize>(
    document_uuid: &str,
    key: &str,
    value: T,
) {
    _moss_api_document_metadata_set(
        document_uuid,
        ConfigSet::<T> {
            key: key.into(),
            value,
        },
    )
    .unwrap();
}
pub unsafe fn moss_api_document_content_set<T: Serialize>(
    document_uuid: &str,
    key: &str,
    value: T,
) {
    _moss_api_document_content_set(
        document_uuid,
        ConfigSet::<T> {
            key: key.into(),
            value,
        },
    )
    .unwrap();
}

pub unsafe fn moss_api_collection_metadata_set<T: Serialize>(
    collection_uuid: &str,
    key: &str,
    value: T,
) {
    _moss_api_collection_metadata_set(
        collection_uuid,
        ConfigSet::<T> {
            key: key.into(),
            value,
        },
    )
    .unwrap();
}

pub unsafe fn moss_api_document_set<T: Serialize>(document_uuid: &str, key: &str, value: T) {
    _moss_api_document_set(
        document_uuid,
        ConfigSet::<T> {
            key: key.into(),
            value,
        },
    )
    .unwrap();
}
pub unsafe fn moss_api_collection_set<T: Serialize>(collection_uuid: &str, key: &str, value: T) {
    _moss_api_collection_set(
        collection_uuid,
        ConfigSet::<T> {
            key: key.into(),
            value,
        },
    )
    .unwrap();
}

pub unsafe fn moss_api_metadata_set<T: Serialize>(metadata_id: &i64, key: &str, value: T) {
    _moss_api_metadata_set(
        metadata_id,
        ConfigSet::<T> {
            key: key.into(),
            value,
        },
    )
    .unwrap();
}
pub unsafe fn moss_api_content_set<T: Serialize>(content_id: &i64, key: &str, value: T) {
    _moss_api_content_set(
        content_id,
        ConfigSet::<T> {
            key: key.into(),
            value,
        },
    )
    .unwrap();
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

pub fn get_rm_time_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}
