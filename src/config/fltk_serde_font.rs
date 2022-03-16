use fltk::enums::Font;
use serde::{Deserialize, Serializer, Deserializer};

pub fn serialize<S: Serializer>(font: &Font, serializer: S) -> Result<S::Ok, S::Error> {
    let s = font.get_name();
    serializer.serialize_str(&s)
}

pub fn deserialize<'d, D: Deserializer<'d>>(deserializer: D) -> Result<Font, D::Error> {
    let s = String::deserialize(deserializer)?;
    Ok(Font::by_name(&s))
}

