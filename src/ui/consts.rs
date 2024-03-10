use mvutils::once::Lazy;
use std::string::ToString;
use std::sync::RwLock;

const STATIC_ELEMENTS: [(&str, &str); 4] = [
    ("element", "mvcore::ui::elements::UiElementImpl"),
    ("", "mvcore::ui::elements::Wrapper"),
    ("label", "ui::Label"),
    ("button", "ui::Button"),
];

pub const META_FILE: &str = "ui.meta";
pub const META_PATH: &str = "assets/ui/ui.meta";
pub const UI_ANNOTATION: &str = "ui_element";
pub const UI_PATH: &str = "assets/ui";
pub const UI_COMPILED_PATH: &str = "src/ui/generated";

pub const UI_MOD_PATH: &str = "src/ui/mod.rs";
pub const LIB_PATH: &str = "src/lib.rs";
pub const MAIN_PATH: &str = "src/main.rs";

pub const ATTRIBUTE_PATH: &str = "mvcore::ui::attributes::Attributes";
pub const STYLE_PATH: &str = "mvcore::ui::style::Style";

pub const UI_EXTENSIONS: [&str; 3] = [".xml", ".ui", ".mui"];
pub const STYLE_EXTENSIONS: [&str; 3] = [".style", ".st", ".mst"];

pub static ELEMENTS: Lazy<RwLock<Vec<(String, String)>>> = Lazy::new(|| {
    STATIC_ELEMENTS
        .map(|(tag, path)| (tag.to_string(), path.to_string()))
        .to_vec()
        .into()
});
