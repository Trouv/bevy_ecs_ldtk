use bevy::prelude::*;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub(crate) fn serialize_color<S: Serializer>(
    color: &Color,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let color = color.as_rgba_f32();
    let mut hex_string =
        hex::encode_upper::<Vec<u8>>(color[0..3].iter().map(|f| (f * 256.) as u8).collect());
    hex_string.insert(0, '#');
    hex_string.serialize(serializer)
}

pub(crate) fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let long_hex = String::deserialize(deserializer)?;

    let hex = match long_hex.strip_prefix('#') {
        Some(h) => h.to_string(),
        None => long_hex,
    };

    Color::hex(hex).map_err(|_| de::Error::custom("Encountered HexColorError"))
}
