use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ColorScheme {
    background:     Option<String>,
    foreground:     Option<String>,
    normal_black:   Option<String>,
    normal_white:   Option<String>,
    normal_red:     Option<String>,
    normal_green:   Option<String>,
    normal_blue:    Option<String>,
    normal_magenta: Option<String>,
    normal_cyan:    Option<String>,
    normal_yellow:  Option<String>,
    bright_black:   Option<String>,
    bright_white:   Option<String>,
    bright_red:     Option<String>,
    bright_green:   Option<String>,
    bright_blue:    Option<String>,
    bright_magenta: Option<String>,
    bright_cyan:    Option<String>,
    bright_yellow:  Option<String>,
}

impl Default for ColorScheme {
    fn default() -> ColorScheme {
        ColorScheme {
            background:     Some("f0f0f0".to_string()),
            foreground:     Some("424242".to_string()),
            normal_black:   Some("424242".to_string()),
            normal_white:   Some("bdbdbd".to_string()),
            normal_red:     Some("b71c1c".to_string()),
            normal_green:   Some("2e7d32".to_string()),
            normal_blue:    Some("0d47a1".to_string()),
            normal_magenta: Some("6a1b9a".to_string()),
            normal_cyan:    Some("0097a7".to_string()),
            normal_yellow:  Some("a08030".to_string()),
            bright_black:   Some("000000".to_string()),
            bright_white:   Some("ffffff".to_string()),
            bright_red:     Some("ff1744".to_string()),
            bright_green:   Some("00c853".to_string()),
            bright_blue:    Some("448aff".to_string()),
            bright_magenta: Some("e040fb".to_string()),
            bright_cyan:    Some("26c6da".to_string()),
            bright_yellow:  Some("fdd83d".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FontConfig {
    regular: Option<String>,
    bold: Option<String>,
    italic: Option<String>,
    bold_italic: Option<String>,
}

impl Default for FontConfig {
    fn default() -> FontConfig {
        FontConfig {
            regular: None,
            bold: None,
            italic: None,
            bold_italic: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    font_styles: FontConfig,
    font_size: Option<i32>,
    color_scheme: ColorScheme,
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
