use mvutils::once::Lazy;
use std::string::ToString;
use std::sync::Mutex;

const STATIC_ELEMENTS: [(&str, &str); 2] = [
    ("element", "mvcore::ui::elements::UiElementImpl"),
    ("label", "mvcore::ui::elements::text::UiLabel"),
];

pub const META_FILE: &str = "ui.meta";
pub const META_PATH: &str = "assets/ui/ui.meta";
pub const UI_ANNOTATION: &str = "ui_element";
pub const UI_PATH: &str = "assets/ui";
pub const UI_COMPILED_PATH: &str = "src/ui";

pub const UI_EXTENSIONS: [&str; 3] = [".xml", ".ui", ".mui"];

pub static ELEMENTS: Lazy<Mutex<Vec<(String, String)>>> = Lazy::new(|| {
    STATIC_ELEMENTS
        .map(|(tag, path)| (tag.to_string(), path.to_string()))
        .to_vec()
        .into()
});
