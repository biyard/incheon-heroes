use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub struct ColorTheme {
    pub background: String,
    pub primary: String,
    pub primary_icon: String,
    pub primary_text: String,
    pub secondary: String,
    pub secondary_icon: String,
    pub secondary_text: String,
}

impl Default for ColorTheme {
    fn default() -> Self {
        ColorTheme {
            background: "white".to_string(),
            primary: "#ADBCD7".to_string(),
            primary_icon: "#74789E".to_string(),
            primary_text: "black".to_string(),
            secondary: "#8588AB".to_string(),
            secondary_icon: "#74789E".to_string(),
            secondary_text: "#74789E".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FontTheme {
    pub exbold15: String,
    pub bold15: String,
}

impl Default for FontTheme {
    fn default() -> Self {
        FontTheme {
            exbold15: "font-extrabold text-[15px] leading-[22.5px]".to_string(),
            bold15: "font-bold text-[15px] leading[22.5px]".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeService;

impl ThemeService {
    pub fn init() {
        use_context_provider(|| ColorTheme::default());
        use_context_provider(|| FontTheme::default());
    }
}
