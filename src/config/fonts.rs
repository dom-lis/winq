use fltk::enums::Font;
use serde::{Serialize, Deserialize};

use super::fltk_serde_font;

pub fn default_font_size() -> i32 {
    16
}

#[derive(Serialize, Deserialize)]
pub struct FontConfig {
    #[serde(with="fltk_serde_font")]
    pub regular: Font,
    #[serde(with="fltk_serde_font")]
    pub bold: Font,              
    #[serde(with="fltk_serde_font")]
    pub italic: Font,              
    #[serde(with="fltk_serde_font")]
    pub bold_italic: Font,              
}

impl FontConfig {
    pub fn by_index(&self, index: usize) -> Option<&Font> {
        match index {
            0x0 => Some(&self.regular),
            0x1 => Some(&self.bold),
            0x2 => Some(&self.italic),
            0x3 => Some(&self.bold_italic),
              _ => None,
        }
    }
}

impl Default for FontConfig {
    fn default() -> FontConfig {
        FontConfig {
            regular: Font::Courier,
            bold: Font::CourierBold,
            italic: Font::CourierItalic,
            bold_italic: Font::CourierBoldItalic,
        }
    }
}
