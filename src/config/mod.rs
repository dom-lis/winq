use fltk::enums::{Font, Color};
use serde::{Serialize, Deserialize};

mod fltk_serde_color;
mod fltk_serde_font;

#[derive(Serialize, Deserialize)]
pub struct ColorScheme {
    #[serde(with="fltk_serde_color")]
    pub background:     Color,
    #[serde(with="fltk_serde_color")]
    pub foreground:     Color,
    #[serde(with="fltk_serde_color")]
    pub normal_black:   Color,
    #[serde(with="fltk_serde_color")]
    pub normal_white:   Color,
    #[serde(with="fltk_serde_color")]
    pub normal_red:     Color,
    #[serde(with="fltk_serde_color")]
    pub normal_green:   Color,
    #[serde(with="fltk_serde_color")]
    pub normal_blue:    Color,
    #[serde(with="fltk_serde_color")]
    pub normal_magenta: Color,
    #[serde(with="fltk_serde_color")]
    pub normal_cyan:    Color,
    #[serde(with="fltk_serde_color")]
    pub normal_yellow:  Color,
    #[serde(with="fltk_serde_color")]
    pub bright_black:   Color,
    #[serde(with="fltk_serde_color")]
    pub bright_white:   Color,
    #[serde(with="fltk_serde_color")]
    pub bright_red:     Color,
    #[serde(with="fltk_serde_color")]
    pub bright_green:   Color,
    #[serde(with="fltk_serde_color")]
    pub bright_blue:    Color,
    #[serde(with="fltk_serde_color")]
    pub bright_magenta: Color,
    #[serde(with="fltk_serde_color")]
    pub bright_cyan:    Color,
    #[serde(with="fltk_serde_color")]
    pub bright_yellow:  Color,
}

impl ColorScheme {
    pub fn by_index(&self, index: usize) -> Option<&Color> {
        match index {
            0x0 => Some(&self.normal_black),
            0x1 => Some(&self.normal_white),
            0x2 => Some(&self.normal_red),
            0x3 => Some(&self.normal_green),
            0x4 => Some(&self.normal_blue),
            0x5 => Some(&self.normal_magenta),
            0x6 => Some(&self.normal_cyan),
            0x7 => Some(&self.normal_yellow),
            0x8 => Some(&self.bright_black),
            0x9 => Some(&self.bright_white),
            0xa => Some(&self.bright_red),
            0xb => Some(&self.bright_green),
            0xc => Some(&self.bright_blue),
            0xd => Some(&self.bright_magenta),
            0xe => Some(&self.bright_cyan),
            0xf => Some(&self.bright_yellow),
              _ => None
        }
    }
}

impl Default for ColorScheme {
    fn default() -> ColorScheme {
        ColorScheme {
            background:     Color::from_hex(0xf0f0f0),
            foreground:     Color::from_hex(0x424242),
            normal_black:   Color::from_hex(0x424242),
            normal_white:   Color::from_hex(0xbdbdbd),
            normal_red:     Color::from_hex(0xb71c1c),
            normal_green:   Color::from_hex(0x2e7d32),
            normal_blue:    Color::from_hex(0x0d47a1),
            normal_magenta: Color::from_hex(0x6a1b9a),
            normal_cyan:    Color::from_hex(0x0097a7),
            normal_yellow:  Color::from_hex(0xa08030),
            bright_black:   Color::from_hex(0x000000),
            bright_white:   Color::from_hex(0xffffff),
            bright_red:     Color::from_hex(0xff1744),
            bright_green:   Color::from_hex(0x00c853),
            bright_blue:    Color::from_hex(0x448aff),
            bright_magenta: Color::from_hex(0xe040fb),
            bright_cyan:    Color::from_hex(0x26c6da),
            bright_yellow:  Color::from_hex(0xfdd83d),
        }
    }
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

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub font_styles: FontConfig,
    pub font_size: Option<i32>,
    pub color_scheme: ColorScheme,
}

impl Config {
    pub fn load() -> Option<Config> {
        use std::fs::File;
        use std::io::BufReader;
        
        let dir = dirs::config_dir()?;
        
        let dir = dir.join("winq");
        let path = dir.join("config.json");
        let config_file = File::open(path).ok()?;
        let reader = BufReader::new(config_file);
        let config = serde_json::from_reader(reader).ok()?;
        config
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            font_styles: FontConfig::default(),
            font_size: None,
            color_scheme: ColorScheme::default()
        }
    }
}
