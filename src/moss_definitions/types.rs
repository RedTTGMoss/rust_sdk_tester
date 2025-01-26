use extism_pdk::{FromBytes, ToBytes, Json};
use serde::{Deserialize, Serialize};

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
        Color { r: color, g: color, b: color, a }
    }
    pub fn from_existing(color: Self, a: Option<i64>) -> Self {
        Color { r: color.r, g: color.g, b: color.b, a }
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

#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct ContextMenu {
    pub key: String,
    pub buttons: Vec<ContextButton>,
}

#[derive(ToBytes, Deserialize, PartialEq, Debug)]
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