#[cfg(not(windows))]
use tui::style::Color;

#[cfg(not(windows))]
pub fn parse_color(s: &str) -> Color {
    if let Ok(i) = u8::from_str_radix(s, 16) {
        Color::Indexed(i.clamp(0, 15))
    } else {
        Color::Reset
    }
}

pub fn parse_key(b: u8) -> Option<char> {
    if (32..127).contains(&b) {
        Some(b as u8 as char)
    } else {
        None
    }
}
