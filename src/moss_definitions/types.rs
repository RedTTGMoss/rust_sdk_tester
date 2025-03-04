#![allow(non_camel_case_types)]

use crate::moss_definitions::accessors::{
    Accessor, AccessorSubType, AccessorType, ACCESSOR_API_DOCUMENT, ACCESSOR_STANDALONE_DOCUMENT,
};
use crate::DocumentNewPDFBuilderError::ValidationError;
use crate::{
    get_rm_time_now, moss_api_document_duplicate, moss_api_document_ensure_download,
    moss_api_document_ensure_download_and_callback, moss_api_document_export,
    moss_api_document_load_files_from_cache, moss_api_document_new_epub,
    moss_api_document_new_notebook, moss_api_document_new_pdf, moss_api_document_randomize_uuids,
    moss_api_document_unload_files, moss_api_get_all, moss_api_metadata_new, moss_text_display,
    moss_text_get_rect, moss_text_make, moss_text_set_font, moss_text_set_rect, moss_text_set_text,
};
use base64::engine::general_purpose;
use base64::Engine;
use chrono::{DateTime, SecondsFormat, TimeZone, Utc};
use derive_builder::Builder;
use extism_pdk::{error, info, Error, FromBytes, Json, ToBytes};
use moss_macros::{moss_color, Accessors, DocumentAccessorBuilder};
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Read};

// Metadata types
#[allow(dead_code)]
pub static DOCUMENT_TYPE: &str = "DocumentType";

#[allow(dead_code)]
pub static COLLECTION_TYPE: &str = "CollectionType";

// Serialization for bytes to base64
#[derive(Debug, Clone, PartialEq)]
pub struct Base64VecU8(pub Vec<u8>);

impl Base64VecU8 {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Self {
        let mut buffer = Vec::new();
        reader.into_inner().read_to_end(&mut buffer).unwrap();
        Self(buffer)
    }
}

impl Serialize for Base64VecU8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let encoded = general_purpose::STANDARD.encode(&self.0);
        serializer.serialize_str(&encoded)
    }
}

impl<'de> Deserialize<'de> for Base64VecU8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let decoded = general_purpose::STANDARD
            .decode(&s)
            .map_err(serde::de::Error::custom)?;
        Ok(Base64VecU8(decoded))
    }
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone, Builder, DocumentAccessorBuilder)]
#[builder(pattern = "owned")]
#[encoding(Json)]
pub struct DocumentNewNotebook {
    name: String,
    #[builder(default)]
    parent: Option<String>,
    #[builder(default = "1")]
    page_count: i64,
    #[builder(default = "Some(Vec::new())")]
    notebook_data: Option<Vec<Base64VecU8>>,
    #[builder(default = "Some(Vec::new())")]
    notebook_files: Option<Vec<String>>,
    #[builder(default)]
    metadata_id: Option<String>,
    #[builder(default)]
    content_id: Option<String>,
    #[builder(default = "Accessor::unknown_api_document()")]
    accessor: Accessor,
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone, Builder, DocumentAccessorBuilder)]
#[builder(pattern = "owned")]
#[encoding(Json)]
pub struct DocumentNewPDF {
    name: String,
    #[builder(default)]
    pdf_data: Option<Base64VecU8>,
    #[builder(default)]
    pdf_file: Option<String>,
    #[builder(default)]
    parent: Option<String>,
    #[builder(default = "Accessor::unknown_api_document()")]
    accessor: Accessor,
}

impl DocumentNewPDFBuilder {
    fn validate(&self) -> Result<(), Error> {
        if self.pdf_data.is_none() && self.pdf_file.is_none() {
            return Err(Error::from(ValidationError(
                "Either pdf_data or pdf_file must be provided".to_string(),
            )));
        }
        Ok(())
    }
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone, Builder, DocumentAccessorBuilder)]
#[builder(pattern = "owned")]
#[encoding(Json)]
pub struct DocumentNewEPUB {
    name: String,
    #[builder(default)]
    epub_data: Option<Base64VecU8>,
    #[builder(default)]
    epub_file: Option<String>,
    #[builder(default)]
    parent: Option<String>,
    #[builder(default = "Accessor::unknown_api_document()")]
    accessor: Accessor,
}

impl DocumentNewEPUBBuilder {
    fn validate(&self) -> Result<(), Error> {
        if self.epub_data.is_none() && self.epub_file.is_none() {
            return Err(Error::from(ValidationError(
                "Either epub_data or epub_file must be provided".to_string(),
            )));
        }
        Ok(())
    }
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone, Builder)]
#[builder(pattern = "owned")]
#[encoding(Json)]
pub struct MetadataNew {
    name: String,
    #[builder(default)]
    parent: Option<String>,
    #[builder(default = "Some(DOCUMENT_TYPE)")]
    document_type: Option<&'static str>,
}

#[derive(ToBytes, FromBytes, Deserialize, Serialize, PartialEq, Debug, Clone, Copy)]
#[encoding(Json)]
pub struct Color {
    pub r: i64,
    pub g: i64,
    pub b: i64,
    pub a: Option<i64>,
}

impl Color {
    pub fn new(r: i64, g: i64, b: i64, a: Option<i64>) -> Self {
        Color { r, g, b, a }
    }
    pub fn new_monochrome(color: i64, a: Option<i64>) -> Self {
        Color {
            r: color,
            g: color,
            b: color,
            a,
        }
    }
    pub fn from_existing(color: Self, a: Option<i64>) -> Self {
        Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a,
        }
    }
}

#[derive(ToBytes, FromBytes, Deserialize, Serialize, PartialEq, Debug, Clone, Copy)]
#[encoding(Json)]
pub struct TextColors {
    pub foreground: Color,
    pub background: Option<Color>,
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct File {
    pub key: String,
    pub path: String,
}

#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct ExtensionInfo {
    pub files: Vec<File>,
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct ContextButton {
    pub text: String,
    pub icon: String,
    pub context_icon: Option<String>,
    pub action: Option<String>,
    pub context_menu: Option<String>,
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct ContextMenu {
    pub key: String,
    pub buttons: Vec<ContextButton>,
    pub pre_loop: Option<String>,
    pub post_loop: Option<String>,
    pub invert: bool,
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct PygameExtraRectEdgeRounding {
    pub edge_rounding: Option<i64>,
    pub edge_rounding_topright: Option<i64>,
    pub edge_rounding_topleft: Option<i64>,
    pub edge_rounding_bottomright: Option<i64>,
    pub edge_rounding_bottomleft: Option<i64>,
}

impl PygameExtraRectEdgeRounding {
    pub fn new(
        edge_rounding: Option<i64>,
        edge_rounding_topright: Option<i64>,
        edge_rounding_topleft: Option<i64>,
        edge_rounding_bottomright: Option<i64>,
        edge_rounding_bottomleft: Option<i64>,
    ) -> Self {
        Self {
            edge_rounding,
            edge_rounding_topright,
            edge_rounding_topleft,
            edge_rounding_bottomright,
            edge_rounding_bottomleft,
        }
    }
    pub fn all(edge_rounding: i64) -> Self {
        Self {
            edge_rounding: Some(edge_rounding),
            edge_rounding_topright: None,
            edge_rounding_topleft: None,
            edge_rounding_bottomright: None,
            edge_rounding_bottomleft: None,
        }
    }
}

#[derive(ToBytes, FromBytes, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct Rect {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}

impl Rect {
    pub fn new(x: i64, y: i64, width: i64, height: i64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn move_to(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y;
    }
    pub fn set_center(&mut self, x: i64, y: i64) {
        self.x = x - self.width / 2;
        self.y = y - self.height / 2;
    }
    pub fn set_size(&mut self, width: i64, height: i64) {
        self.width = width;
        self.height = height;
    }
    pub fn set_topleft(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y;
    }

    pub fn set_topright(&mut self, x: i64, y: i64) {
        self.x = x - self.width;
        self.y = y;
    }

    pub fn set_bottomleft(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y - self.height;
    }

    pub fn set_bottomright(&mut self, x: i64, y: i64) {
        self.x = x - self.width;
        self.y = y - self.height;
    }
}

#[derive(FromBytes, Deserialize, ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct PygameExtraRect {
    pub color: Color,
    pub rect: Rect,
    pub width: i64,
    pub edge_rounding: Option<PygameExtraRectEdgeRounding>,
}

#[derive(FromBytes, Deserialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct MossState {
    pub width: i32,
    pub height: i32,
    pub current_screen: String,
    pub opened_context_menus: Vec<String>,
    pub icons: Vec<String>,
}

#[derive(FromBytes, Deserialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct ConfigGet<T> {
    pub value: T,
}

#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct ConfigSet<T> {
    pub key: String,
    pub value: T,
}

#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct MossScreen {
    pub key: String,
    pub screen_pre_loop: Option<String>,
    pub screen_loop: String,
    pub screen_post_loop: Option<String>,
    pub event_hook: Option<String>,
}

impl MossScreen {
    pub fn basic(key: String, screen_loop: String) -> Self {
        Self {
            key,
            screen_pre_loop: None,
            screen_loop,
            screen_post_loop: None,
            event_hook: None,
        }
    }
    pub fn basic_with_event_hook(key: String, screen_loop: String, event_hook: String) -> Self {
        Self {
            key,
            screen_pre_loop: None,
            screen_loop,
            screen_post_loop: None,
            event_hook: Some(event_hook),
        }
    }
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct TextRef {
    text_id: i64,
    pub text: String,
    pub font: String,
    pub font_size: i64,
    pub colors: TextColors,
    pub rect: Rect,
}

impl TextRef {
    pub unsafe fn new(
        text_id: i64,
        text: &str,
        font: &str,
        font_size: i64,
        colors: TextColors,
    ) -> Self {
        Self {
            text_id,
            text: text.to_string(),
            font: font.to_string(),
            font_size,
            colors,
            rect: moss_text_get_rect(text_id).unwrap(),
        }
    }

    pub unsafe fn create(text: &str, font: &str, font_size: i64, colors: TextColors) -> Self {
        let text_id = moss_text_make(text, font, font_size, colors).unwrap();
        Self::new(text_id, text, font, font_size, colors)
    }

    pub unsafe fn create_white(text: &str, font: &str, font_size: i64) -> Self {
        Self::create(
            text,
            font,
            font_size,
            TextColors {
                foreground: moss_color!(0xFFFFFF),
                background: None,
            },
        )
    }

    pub unsafe fn update_rect(&mut self) {
        moss_text_set_rect(self.text_id, &self.rect).unwrap();
    }

    pub unsafe fn update_font(&mut self, font: &str, font_size: i64) {
        self.font = font.to_string();
        self.font_size = font_size;
        self.rect = moss_text_set_font(self.text_id, font, font_size).unwrap();
    }

    pub unsafe fn update_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.rect = moss_text_set_text(self.text_id, text).unwrap();
    }

    pub unsafe fn create_black(text: &str, font: &str, font_size: i64) -> Self {
        Self::create(
            text,
            font,
            font_size,
            TextColors {
                foreground: moss_color!(0x000000),
                background: None,
            },
        )
    }

    pub unsafe fn display(&self) {
        moss_text_display(self.text_id).unwrap();
    }
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct RM_File {
    pub content_count: i64,
    pub hash: String,
    pub rm_filename: String,
    pub size: i64,
    pub uuid: String,
}

impl RM_File {
    pub fn new(content_count: i64, hash: &str, rm_filename: &str, size: i64, uuid: &str) -> Self {
        Self {
            content_count,
            hash: hash.to_string(),
            rm_filename: rm_filename.to_string(),
            size,
            uuid: uuid.to_string(),
        }
    }
    // TODO: get_contents
    // TODO: get_files
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct RM_TimestampedValue<T> {
    pub timestamp: String,
    pub value: Option<T>,
}

impl<T> RM_TimestampedValue<T> {
    pub fn new(timestamp: &str, value: Option<T>) -> Self {
        Self {
            timestamp: timestamp.to_string(),
            value,
        }
    }
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct RM_TimestampedDate {
    timestamp: String,
    pub value: Option<String>,
}

impl RM_TimestampedDate {
    pub fn new(timestamp: &str, value: &str) -> Self {
        Self {
            timestamp: timestamp.to_string(),
            value: Some(value.to_string()),
        }
    }

    pub fn get_timestamp(&self) -> i64 {
        let datetime: DateTime<Utc> = self.timestamp.parse().expect("Invalid timestamp");
        datetime.timestamp()
    }

    pub fn set_timestamp(&mut self, timestamp: i64) {
        let datetime_back = Utc
            .timestamp_opt(timestamp, 0)
            .single()
            .expect("Invalid timestamp");
        let iso_string = datetime_back.to_rfc3339_opts(SecondsFormat::Secs, true);
        self.value = Some(iso_string);
    }
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct RM_Tag {
    pub name: String,
    pub timestamp: i64,
}

impl RM_Tag {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            timestamp: get_rm_time_now(),
        }
    }
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct RM_Page {
    pub id: String,
    pub index: RM_TimestampedValue<String>,
    pub template: RM_TimestampedValue<String>,
    pub redirect: Option<RM_TimestampedValue<i64>>,
    pub scroll_time: Option<RM_TimestampedDate>,
    pub vertical_scroll: Option<RM_TimestampedValue<i64>>,
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct RM_CPagesUUID {
    pub first: String,
    pub second: i64,
}

impl RM_CPagesUUID {
    pub fn new(first: &str, second: i64) -> Self {
        Self {
            first: first.to_string(),
            second,
        }
    }
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct RM_CPages {
    pub pages: Vec<RM_Page>,
    pub original: RM_TimestampedValue<i64>,
    pub last_opened: RM_TimestampedValue<String>,
    pub uuids: Vec<RM_CPagesUUID>,
}

#[allow(non_snake_case)]
#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct RM_Zoom {
    // RAW
    pub zoomMode: String,
    pub customZoomCenterX: i64,
    pub customZoomCenterY: i64,
    pub customZoomPageHeight: i64,
    pub customZoomPageWidth: i64,
    pub customZoomScale: f64,
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone, Accessors)]
#[accessor_type("content")]
#[encoding(Json)]
pub struct RM_Content {
    pub hash: String,
    #[accessor(exclude)]
    pub c_pages: RM_CPages,
    pub cover_page_number: i64,
    pub file_type: String,
    pub version: i64,
    pub usable: bool,
    pub zoom: RM_Zoom,
    pub orientation: String,
    pub tags: Vec<RM_Tag>,
    pub size_in_bytes: i64,
    pub dummy_document: bool,
    // reference data
    pub accessor: Accessor,
}

impl RM_Content {
    pub unsafe fn get(id: i64) -> Result<Self, Error> {
        match moss_api_get_all::<Self>(&Accessor::standalone_content(id)) {
            Ok(get) => Ok(get.value),
            Err(e) => {
                error!("Error retrieving content: {:?}", e);
                Err(e)
            }
        }
    }

    pub unsafe fn _get_from_document(uuid: String, item_type: AccessorType) -> Result<Self, Error> {
        match moss_api_get_all::<Self>(&Accessor::document_content(uuid, item_type)) {
            Ok(get) => Ok(get.value),
            Err(e) => {
                error!("Error retrieving content: {:?}", e);
                Err(e)
            }
        }
    }

    pub unsafe fn get_from_api_document(uuid: String) -> Result<Self, Error> {
        Self::_get_from_document(uuid, AccessorType::ApiItem)
    }

    pub unsafe fn get_from_standalone_document(uuid: String) -> Result<Self, Error> {
        Self::_get_from_document(uuid, AccessorType::StandaloneItem)
    }
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone, Accessors)]
#[accessor_type("metadata")]
#[encoding(Json)]
pub struct RM_Metadata {
    pub hash: String,
    pub r#type: String,
    pub parent: Option<String>,
    pub created_time: i64,
    pub last_modified: i64,
    pub visible_name: String,
    pub metadata_modified: bool,
    pub modified: bool,
    pub synced: bool,
    pub version: Option<i64>,
    pub last_opened: Option<i64>,
    pub last_opened_page: Option<i64>,
    // reference data
    pub accessor: Accessor,
}

impl RM_Metadata {
    pub unsafe fn get(id: i64) -> Result<Self, Error> {
        match moss_api_get_all::<Self>(&Accessor::standalone_metadata(id)) {
            Ok(get) => Ok(get.value),
            Err(e) => {
                error!("Error retrieving metadata: {:?}", e);
                Err(e)
            }
        }
    }
    pub unsafe fn _get_from(
        uuid: String,
        item_type: AccessorType,
        sub_type: AccessorSubType,
    ) -> Result<Self, Error> {
        match moss_api_get_all::<Self>(&Accessor::sub_metadata(uuid, item_type, sub_type)) {
            Ok(get) => Ok(get.value),
            Err(e) => {
                error!("Error retrieving metadata: {:?}", e);
                Err(e)
            }
        }
    }

    pub unsafe fn get_from_api_document(uuid: String) -> Result<Self, Error> {
        Self::_get_from(uuid, AccessorType::ApiItem, AccessorSubType::Document)
    }

    pub unsafe fn get_from_standalone_document(uuid: String) -> Result<Self, Error> {
        Self::_get_from(
            uuid,
            AccessorType::StandaloneItem,
            AccessorSubType::Document,
        )
    }
    pub unsafe fn get_from_api_collection(uuid: String) -> Result<Self, Error> {
        Self::_get_from(uuid, AccessorType::ApiItem, AccessorSubType::Collection)
    }

    pub unsafe fn get_from_standalone_collection(uuid: String) -> Result<Self, Error> {
        Self::_get_from(
            uuid,
            AccessorType::StandaloneItem,
            AccessorSubType::Collection,
        )
    }

    pub unsafe fn _new(builder: MetadataNewBuilder) -> Result<i64, Error> {
        let metadata = builder.build()?;

        match moss_api_metadata_new(metadata) {
            Ok(metadata_id) => Ok(metadata_id),
            Err(e) => {
                error!("Error creating new metadata: {:?}", e);
                Err(e)
            }
        }
    }
    pub unsafe fn new(builder: MetadataNewBuilder) -> Result<Self, Error> {
        match Self::_new(builder) {
            Ok(metadata_id) => Self::get(metadata_id),
            Err(e) => Err(e),
        }
    }
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone, Accessors)]
#[accessor_type("collection")]
#[accessor_uuid(true)]
#[encoding(Json)]
pub struct RM_DocumentCollection {
    pub tags: Vec<RM_Tag>,
    #[accessor(exclude)]
    pub metadata: RM_Metadata,
    pub uuid: String,
    pub has_items: bool,
    pub accessor: Accessor,
}

impl RM_DocumentCollection {
    pub unsafe fn get(uuid: String) -> Result<Self, Error> {
        match moss_api_get_all::<Self>(&Accessor::api_collection(uuid)) {
            Ok(get) => Ok(get.value),
            Err(e) => {
                error!("Error retrieving document collection: {:?}", e);
                Err(e)
            }
        }
    }
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone, Accessors)]
#[accessor_type("document")]
#[accessor_uuid(true)]
#[encoding(Json)]
pub struct RM_Document {
    #[accessor(exclude)]
    pub files: Vec<RM_File>,
    #[accessor(exclude)]
    pub content: RM_Content,
    #[accessor(exclude)]
    pub metadata: RM_Metadata,
    pub uuid: String,
    pub server_hash: Option<String>,
    #[accessor(exclude)]
    pub files_available: Vec<String>,
    pub downloading: bool,
    pub provision: bool,
    #[accessor(exclude)]
    pub available: bool,
    pub accessor: Accessor,
}

impl RM_Document {
    pub unsafe fn _get_with_accessor(accessor: &Accessor) -> Result<Self, Error> {
        match moss_api_get_all::<Self>(accessor) {
            Ok(get) => Ok(get.value),
            Err(e) => {
                error!("Error retrieving document: {:?}", e);
                Err(e)
            }
        }
    }

    pub unsafe fn _get(uuid: String, item_type: AccessorType) -> Result<Self, Error> {
        Self::_get_with_accessor(&Accessor::document(uuid, item_type))
    }

    pub unsafe fn get_api(uuid: String) -> Result<Self, Error> {
        Self::_get(uuid, AccessorType::ApiItem)
    }

    pub unsafe fn get_standalone(uuid: String) -> Result<Self, Error> {
        Self::_get(uuid, AccessorType::StandaloneItem)
    }

    pub unsafe fn _new_notebook(builder: DocumentNewNotebookBuilder) -> Result<Accessor, Error> {
        let notebook = builder.build()?;

        match moss_api_document_new_notebook(&notebook) {
            Ok(document_uuid) => {
                info!(
                    "The accessor is: {}",
                    notebook.accessor.new_uuid(document_uuid.clone())
                );
                Ok(notebook.accessor.new_uuid(document_uuid))
            }
            Err(e) => {
                error!("Error creating new notebook: {:?}", e);
                Err(e)
            }
        }
    }
    pub unsafe fn _new_pdf(builder: DocumentNewPDFBuilder) -> Result<Accessor, Error> {
        builder.validate()?;
        let pdf = builder.build()?;

        match moss_api_document_new_pdf(&pdf) {
            Ok(document_uuid) => Ok(pdf.accessor.new_uuid(document_uuid)),
            Err(e) => {
                error!("Error creating new pdf: {:?}", e);
                Err(e)
            }
        }
    }
    pub unsafe fn _new_epub(builder: DocumentNewEPUBBuilder) -> Result<Accessor, Error> {
        builder.validate()?;
        let epub = builder.build()?;

        match moss_api_document_new_epub(&epub) {
            Ok(document_uuid) => Ok(epub.accessor.new_uuid(document_uuid)),
            Err(e) => {
                error!("Error creating new pdf: {:?}", e);
                Err(e)
            }
        }
    }
    pub unsafe fn new_notebook(builder: DocumentNewNotebookBuilder) -> Result<Self, Error> {
        match Self::_new_notebook(builder) {
            Ok(document_accessor) => Self::_get_with_accessor(&document_accessor),
            Err(e) => Err(e),
        }
    }
    pub unsafe fn new_pdf(builder: DocumentNewPDFBuilder) -> Result<Self, Error> {
        match Self::_new_pdf(builder) {
            Ok(document_accessor) => Self::_get_with_accessor(&document_accessor),
            Err(e) => Err(e),
        }
    }
    pub unsafe fn new_epub(builder: DocumentNewEPUBBuilder) -> Result<Self, Error> {
        match Self::_new_epub(builder) {
            Ok(document_accessor) => Self::_get_with_accessor(&document_accessor),
            Err(e) => Err(e),
        }
    }

    pub unsafe fn _duplicate(&self) -> Result<String, Error> {
        match moss_api_document_duplicate(&self.accessor) {
            Ok(document_uuid) => Ok(document_uuid),
            Err(e) => {
                error!("Error duplicating document: {:?}", e);
                Err(e)
            }
        }
    }

    pub unsafe fn duplicate(&self) -> Result<Self, Error> {
        match self._duplicate() {
            Ok(document_uuid) => Self::_get_with_accessor(&self.accessor.new_uuid(document_uuid)),
            Err(e) => Err(e),
        }
    }

    pub unsafe fn _randomize_uuids(&self) -> Result<String, Error> {
        match moss_api_document_randomize_uuids(&self.accessor) {
            Ok(document_uuid) => Ok(document_uuid),
            Err(e) => {
                error!("Error randomizing document UUIDs: {:?}", e);
                Err(e)
            }
        }
    }

    pub unsafe fn randomize_uuids(&self) -> Result<Self, Error> {
        match self._randomize_uuids() {
            Ok(document_uuid) => Self::_get_with_accessor(&self.accessor.new_uuid(document_uuid)),
            Err(e) => Err(e),
        }
    }

    pub unsafe fn unload_files(&self) {
        moss_api_document_unload_files(&self.accessor).unwrap()
    }
    pub unsafe fn load_files_from_cache(&self) {
        moss_api_document_load_files_from_cache(&self.accessor).unwrap()
    }

    pub unsafe fn ensure_download_and_callback(&self, callback: &str) {
        moss_api_document_ensure_download_and_callback(&self.accessor, callback).unwrap()
    }
    pub unsafe fn ensure_download(&self) {
        moss_api_document_ensure_download(&self.accessor).unwrap()
    }
    pub unsafe fn export(&self) {
        moss_api_document_export(&self.accessor).unwrap()
    }
}

#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct RM_RootInfo {
    pub generation: i64,
    pub hash: String,
}
