use bevy::prelude::*;
use serde::{Deserialize, Serialize, Serializer};

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
