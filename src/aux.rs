use tui::style::Color;

pub fn parse_color(s: &str) -> Color {
    if let Ok(i) = u8::from_str_radix(s, 16) {
        Color::Indexed(i.clamp(0, 15))
    } else {
        Color::Reset
    }
}
