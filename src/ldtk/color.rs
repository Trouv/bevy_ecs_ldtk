use bevy::prelude::*;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub(crate) fn serialize<S: Serializer>(color: &Color, serializer: S) -> Result<S::Ok, S::Error> {
    color.to_srgba().to_hex().serialize(serializer)
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    Srgba::hex(String::deserialize(deserializer)?)
        .map(|c| c.into())
        .map_err(|_| de::Error::custom("Encountered HexColorError"))
}

pub mod optional {
    use bevy::prelude::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub(crate) fn serialize<S: Serializer>(
        color: &Option<Color>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(color) = color {
            super::serialize(color, serializer)
        } else {
            serializer.serialize_none()
        }
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper(#[serde(with = "super")] Color);

        let c: Option<Wrapper> = Option::deserialize(deserializer)?;
        Ok(c.map(|c| c.0))
    }
}
