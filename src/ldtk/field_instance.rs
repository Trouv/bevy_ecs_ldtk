use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[allow(unused_imports)]
use super::{EntityInstance, GridPoint, Level, ReferenceToAnEntityInstance, TilesetRectangle};
use bevy::prelude::*;
use regex::Regex;

use crate::ldtk::color;

#[derive(Debug, Clone, Serialize, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct FieldInstance {
    /// Field definition identifier
    #[serde(rename = "__identifier")]
    pub identifier: String,

    /// Optional TilesetRect used to display this field (this can be the field own Tile, or some
    /// other Tile guessed from the value, like an Enum).
    #[serde(rename = "__tile")]
    pub tile: Option<TilesetRectangle>,

    /// Type of the field, such as `Int`, `Float`, `String`, `Enum(my_enum_name)`, `Bool`,
    /// etc.<br/>  NOTE: if you enable the advanced option **Use Multilines type**, you will have
    /// "*Multilines*" instead of "*String*" when relevant.
    #[serde(rename = "__type")]
    pub field_instance_type: String,

    /// Actual value of the field instance. The value type varies, depending on `__type`:<br/>
    /// - For **classic types** (ie. Integer, Float, Boolean, String, Text and FilePath), you
    /// just get the actual value with the expected type.<br/>   - For **Color**, the value is an
    /// hexadecimal string using "#rrggbb" format.<br/>   - For **Enum**, the value is a String
    /// representing the selected enum value.<br/>   - For **Point**, the value is a
    /// [GridPoint](#ldtk-GridPoint) object.<br/>   - For **Tile**, the value is a
    /// [TilesetRect](#ldtk-TilesetRect) object.<br/>   - For **EntityRef**, the value is an
    /// [EntityReferenceInfos](#ldtk-EntityReferenceInfos) object.<br/><br/>  If the field is an
    /// array, then this `__value` will also be a JSON array.
    #[serde(rename = "__value")]
    pub value: FieldValue,

    /// Reference of the **Field definition** UID
    pub def_uid: i32,

    /// Editor internal raw values
    #[reflect(ignore)]
    pub real_editor_values: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
struct FieldInstanceHelper {
    #[serde(rename = "__identifier")]
    pub identifier: String,

    #[serde(rename = "__tile")]
    pub tile: Option<TilesetRectangle>,

    #[serde(rename = "__type")]
    pub field_instance_type: String,

    #[serde(rename = "__value")]
    pub value: serde_json::Value,

    #[serde(rename = "defUid")]
    pub def_uid: i32,

    #[serde(rename = "realEditorValues")]
    pub real_editor_values: Vec<Option<serde_json::Value>>,
}

#[derive(Deserialize)]
struct ColorHelper(#[serde(with = "color")] Color);

impl<'de> Deserialize<'de> for FieldInstance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = FieldInstanceHelper::deserialize(deserializer)?;

        let value = match helper.field_instance_type.as_str() {
            "Int" => FieldValue::Int(
                Option::<i32>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "Float" => FieldValue::Float(
                Option::<f32>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "Bool" => FieldValue::Bool(bool::deserialize(helper.value).map_err(de::Error::custom)?),
            "String" => FieldValue::String(
                Option::<String>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "Color" => {
                let value = color::deserialize(helper.value).map_err(de::Error::custom)?;

                FieldValue::Color(value)
            }
            "FilePath" => FieldValue::FilePath(
                Option::<String>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "Tile" => FieldValue::Tile(
                Option::<TilesetRectangle>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "EntityRef" => FieldValue::EntityRef(
                Option::<ReferenceToAnEntityInstance>::deserialize(helper.value)
                    .map_err(de::Error::custom)?,
            ),
            "Point" => {
                let point_helper =
                    Option::<GridPoint>::deserialize(helper.value).map_err(de::Error::custom)?;

                FieldValue::Point(point_helper.map(|p| IVec2::new(p.cx, p.cy)))
            }
            "Multilines" => FieldValue::String(
                Option::<String>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "Array<Multilines>" => FieldValue::Strings(
                Vec::<Option<String>>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "Array<Int>" => FieldValue::Ints(
                Vec::<Option<i32>>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "Array<Float>" => FieldValue::Floats(
                Vec::<Option<f32>>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "Array<Bool>" => FieldValue::Bools(
                Vec::<bool>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "Array<String>" => FieldValue::Strings(
                Vec::<Option<String>>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "Array<Color>" => {
                let helpers =
                    Vec::<ColorHelper>::deserialize(helper.value).map_err(de::Error::custom)?;

                FieldValue::Colors(helpers.iter().map(|h| h.0).collect())
            }
            "Array<FilePath>" => FieldValue::Strings(
                Vec::<Option<String>>::deserialize(helper.value).map_err(de::Error::custom)?,
            ),
            "Array<Tile>" => FieldValue::Tiles(
                Vec::<Option<TilesetRectangle>>::deserialize(helper.value)
                    .map_err(de::Error::custom)?,
            ),
            "Array<EntityRef>" => FieldValue::EntityRefs(
                Vec::<Option<ReferenceToAnEntityInstance>>::deserialize(helper.value)
                    .map_err(de::Error::custom)?,
            ),
            "Array<Point>" => {
                let point_helpers = Vec::<Option<GridPoint>>::deserialize(helper.value)
                    .map_err(de::Error::custom)?;

                let points = point_helpers
                    .into_iter()
                    .map(|ph| ph.map(|p| IVec2::new(p.cx, p.cy)))
                    .collect();

                FieldValue::Points(points)
            }
            t => {
                let enum_regex =
                    Regex::new(r"^(LocalEnum|ExternEnum)\.").expect("enum regex should be valid");
                let enums_regex = Regex::new(r"^Array<(LocalEnum|ExternEnum)\.")
                    .expect("enums regex should be valid");

                if enum_regex.is_match(t) {
                    FieldValue::Enum(
                        Option::<String>::deserialize(helper.value).map_err(de::Error::custom)?,
                    )
                } else if enums_regex.is_match(t) {
                    FieldValue::Enums(
                        Vec::<Option<String>>::deserialize(helper.value)
                            .map_err(de::Error::custom)?,
                    )
                } else {
                    return Err(de::Error::custom(format!(
                        "Encountered unknown field type: {t}"
                    )));
                }
            }
        };

        Ok(FieldInstance {
            identifier: helper.identifier,
            tile: helper.tile,
            field_instance_type: helper.field_instance_type,
            def_uid: helper.def_uid,
            real_editor_values: helper.real_editor_values,
            value,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Reflect)]
#[reflect(opaque)]
#[serde(untagged)]
/// The actual value of a field instance on a [Level] or [EntityInstance].
///
/// This has been re-typed for this plugin.
/// In LDtk's [QuickType loader](https://ldtk.io/files/quicktype/LdtkJson.rs),
/// this is just a [serde_json::Value].
pub enum FieldValue {
    Int(Option<i32>),
    Float(Option<f32>),
    Bool(bool),
    /// Represents either a String or a Multilines
    String(Option<String>),
    #[serde(with = "color")]
    Color(Color),
    FilePath(Option<String>),
    Enum(Option<String>),
    Tile(Option<TilesetRectangle>),
    EntityRef(Option<ReferenceToAnEntityInstance>),
    #[serde(serialize_with = "serialize_point")]
    Point(Option<IVec2>),
    Ints(Vec<Option<i32>>),
    Floats(Vec<Option<f32>>),
    Bools(Vec<bool>),
    /// Represents either Strings or Multilines
    Strings(Vec<Option<String>>),
    #[serde(serialize_with = "serialize_colors")]
    Colors(Vec<Color>),
    FilePaths(Vec<Option<String>>),
    Enums(Vec<Option<String>>),
    Tiles(Vec<Option<TilesetRectangle>>),
    EntityRefs(Vec<Option<ReferenceToAnEntityInstance>>),
    #[serde(serialize_with = "serialize_points")]
    Points(Vec<Option<IVec2>>),
}

fn serialize_colors<S: Serializer>(colors: &[Color], serializer: S) -> Result<S::Ok, S::Error> {
    let field_values: Vec<FieldValue> = colors.iter().map(|c| FieldValue::Color(*c)).collect();
    field_values.serialize(serializer)
}

fn serialize_point<S: Serializer>(point: &Option<IVec2>, serializer: S) -> Result<S::Ok, S::Error> {
    let point_helper = point.map(|p| GridPoint { cx: p.x, cy: p.y });
    point_helper.serialize(serializer)
}

fn serialize_points<S: Serializer>(
    points: &[Option<IVec2>],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let field_values: Vec<FieldValue> = points.iter().map(|p| FieldValue::Point(*p)).collect();
    field_values.serialize(serializer)
}
