use tui::style::Color;

pub fn parse_color(s: &str) -> Result<Color, ()> {
    if let Ok(i) = s.parse::<u8>() {
        Ok(Color::Indexed(i.clamp(0, 15)))
    } else {
        match s {
            "reset" => Ok(Color::Reset),
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "gray" => Ok(Color::Gray),
            "darkgray" => Ok(Color::DarkGray),
            "lightred" => Ok(Color::LightRed),
            "lightgreen" => Ok(Color::LightGreen),
            "lightyellow" => Ok(Color::LightYellow),
            "lightblue" => Ok(Color::LightBlue),
            "lightmagenta" => Ok(Color::LightMagenta),
            "lightcyan" => Ok(Color::LightCyan),
            "white" => Ok(Color::White),
            _ => Err(()),
        }
    }
}
