use serde::{Serialize, Deserialize};

mod color_scheme;
mod fltk_serde_color;
mod fltk_serde_font;
mod fonts;

pub use color_scheme::*;
pub use fonts::*;

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub font_styles: FontConfig,
    #[serde(default="default_font_size")]
    pub font_size: i32,
    #[serde(default)]
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
            font_size: default_font_size(),
            color_scheme: ColorScheme::default()
        }
    }
}
