use fltk::enums::Color;
use serde::{Deserialize, Serializer, Deserializer};

pub fn serialize<S: Serializer>(color: &Color, serializer: S) -> Result<S::Ok, S::Error> {
    let s = format!("{:x}", color.bits());
    serializer.serialize_str(&s)
}

pub fn deserialize<'d, D: Deserializer<'d>>(deserializer: D) -> Result<Color, D::Error> {
    let s = String::deserialize(deserializer)?;
    let z = u32::from_str_radix(&s, 16).map_err(serde::de::Error::custom)?;
    Ok(Color::from_hex(z))
}

