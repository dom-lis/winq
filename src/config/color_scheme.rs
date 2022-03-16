use serde::{Serialize, Deserialize};
use fltk::enums::Color;

use super::fltk_serde_color;

#[derive(Serialize, Deserialize)]
pub struct ColorScheme {
    #[serde(with="fltk_serde_color", default="ColorScheme::default_background")]
    pub background:     Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_foreground")]
    pub foreground:     Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_normal_black")]
    pub normal_black:   Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_normal_white")]
    pub normal_white:   Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_normal_red")]
    pub normal_red:     Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_normal_green")]
    pub normal_green:   Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_normal_blue")]
    pub normal_blue:    Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_normal_magenta")]
    pub normal_magenta: Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_normal_cyan")]
    pub normal_cyan:    Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_normal_yellow")]
    pub normal_yellow:  Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_bright_black")]
    pub bright_black:   Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_bright_white")]
    pub bright_white:   Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_bright_red")]
    pub bright_red:     Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_bright_green")]
    pub bright_green:   Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_bright_blue")]
    pub bright_blue:    Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_bright_magenta")]
    pub bright_magenta: Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_bright_cyan")]
    pub bright_cyan:    Color,
    #[serde(with="fltk_serde_color", default="ColorScheme::default_bright_yellow")]
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

    pub fn default_background() -> Color {
        Color::from_hex(0xf0f0f0)
    }

    pub fn default_foreground() -> Color {
        Color::from_hex(0x424242)
    }

    pub fn default_normal_black() -> Color {
        Color::from_hex(0x424242)
    }

    pub fn default_normal_white() -> Color {
        Color::from_hex(0xbdbdbd)
    }

    pub fn default_normal_red() -> Color {
        Color::from_hex(0xb71c1c)
    }

    pub fn default_normal_green() -> Color {
        Color::from_hex(0x2e7d32)
    }

    pub fn default_normal_blue() -> Color {
        Color::from_hex(0x0d47a1)
    }

    pub fn default_normal_magenta() -> Color {
        Color::from_hex(0x6a1b9a)
    }

    pub fn default_normal_cyan() -> Color {
        Color::from_hex(0x0097a7)
    }

    pub fn default_normal_yellow() -> Color {
        Color::from_hex(0xa08030)
    }

    pub fn default_bright_black() -> Color {
        Color::from_hex(0x000000)
    }

    pub fn default_bright_white() -> Color {
        Color::from_hex(0xffffff)
    }

    pub fn default_bright_red() -> Color {
        Color::from_hex(0xff1744)
    }

    pub fn default_bright_green() -> Color {
        Color::from_hex(0x00c853)
    }

    pub fn default_bright_blue() -> Color {
        Color::from_hex(0x448aff)
    }

    pub fn default_bright_magenta() -> Color {
        Color::from_hex(0xe040fb)
    }

    pub fn default_bright_cyan() -> Color {
        Color::from_hex(0x26c6da)
    }

    pub fn default_bright_yellow() -> Color {
        Color::from_hex(0xfdd83d)
    }

}

impl Default for ColorScheme {
    fn default() -> ColorScheme {
        ColorScheme {
            background:     ColorScheme::default_background(),
            foreground:     ColorScheme::default_foreground(),
            normal_black:   ColorScheme::default_bright_yellow(),
            normal_white:   ColorScheme::default_normal_white(),
            normal_red:     ColorScheme::default_bright_yellow(),
            normal_green:   ColorScheme::default_bright_yellow(),
            normal_blue:    ColorScheme::default_normal_blue(),
            normal_magenta: ColorScheme::default_normal_magenta(),
            normal_cyan:    ColorScheme::default_normal_cyan(),
            normal_yellow:  ColorScheme::default_normal_yellow(),
            bright_black:   ColorScheme::default_bright_black(),
            bright_white:   ColorScheme::default_bright_white(),
            bright_red:     ColorScheme::default_bright_red(),
            bright_green:   ColorScheme::default_bright_green(),
            bright_blue:    ColorScheme::default_bright_blue(),
            bright_magenta: ColorScheme::default_bright_magenta(),
            bright_cyan:    ColorScheme::default_bright_cyan(),
            bright_yellow:  ColorScheme::default_bright_yellow(),
        }
    }
}

