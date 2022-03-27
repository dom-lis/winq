use fltk::enums::Color;
use serde::{Deserialize, Serializer, Deserializer};

pub fn serialize<S: Serializer>(color: &Color, serializer: S) -> Result<S::Ok, S::Error> {
    let s = color.to_hex_str();
    serializer.serialize_str(&s)
}

pub fn deserialize<'d, D: Deserializer<'d>>(deserializer: D) -> Result<Color, D::Error> {
    let s = String::deserialize(deserializer)?;
    let c = Color::from_hex_str(&s).map_err(serde::de::Error::custom)?;
    Ok(c)
}

