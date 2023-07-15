//! Contains [`LdtkFields`] trait, providing convenience methods for accessing field instances.
use crate::ldtk::{
    all_some_iter::AllSomeIter, EntityInstance, FieldInstance, FieldValue, Level,
    ReferenceToAnEntityInstance, TilesetRectangle,
};
use bevy::prelude::*;
use paste::paste;
use thiserror::Error;

/// Errors related to the [`LdtkFields`] trait.
#[derive(Debug, PartialEq, Eq, Error)]
pub enum LdtkFieldsError {
    /// Could not find a field instance with the given identifier.
    #[error("could not find {identifier} field")]
    FieldNotFound { identifier: String },
    /// The field instance exists, but is the wrong [`FieldValue`] variant.
    #[error("found {identifier} field, but its type is not correct")]
    WrongFieldType { identifier: String },
    /// The field instance exists and is the correct variant, but the value is null.
    #[error("found {identifier} field of the correct type, but the value is null")]
    UnexpectedNull { identifier: String },
}

/// Base macro for generating a method that accesses a field instance and unwraps its [FieldValue]
/// variant into the assigned type, or errors if it isn't the correct variant.
///
/// This macro is not intended for doing any further unwrapping, such as unwrapping an option.
macro_rules! create_base_get_field_method {
    ($adjective:literal, $var_name:ident, $variant:ident, $return_type:ty) => {
        paste! {
            #[doc = " Get this item's " $adjective $variant " field value for the given identifier."]
            ///
            /// # Errors
            /// - returns [`LdtkFieldsError::FieldNotFound`] if no field with the given identifier exists.
            #[doc = " - returns [`LdtkFieldsError::WrongFieldType`] if the field is not [`FieldValue::" $variant "`]."]
            fn [< get_ $var_name _field >](
                &self,
                identifier: &str,
            ) -> Result<$return_type, LdtkFieldsError> {
                match self.get_field(identifier)? {
                    FieldValue::$variant($var_name) => Ok($var_name),
                    _ => Err(LdtkFieldsError::WrongFieldType {
                        identifier: identifier.to_string(),
                    }),
                }
            }
        }
    }
}

/// Generates a `get_type_field` method corresponding to a `get_maybe_type_field` method,
/// unwrapping the optional or erroring.
macro_rules! create_get_field_method {
    ($variant:ident, $type:ty) => {
        paste! {
            #[doc = " Get this item's non-null " $variant " field value for the given identifier."]
            ///
            /// # Errors
            /// - returns [`LdtkFieldsError::FieldNotFound`] if no field with the given identifier exists.
            #[doc = " - returns [`LdtkFieldsError::WrongFieldType`] if the field is not [`FieldValue::" $variant "`]."]
            /// - returns [`LdtkFieldsError::UnexpectedNull`] if the field is null.
            fn [< get_ $variant:snake _field >](&self, identifier: &str) -> Result<&$type, LdtkFieldsError> {
                if let Some([< $variant:snake _ >]) = self.[< get_maybe_ $variant:snake _field >](identifier.clone())? {
                    Ok([< $variant:snake _ >])
                } else {
                    Err(LdtkFieldsError::UnexpectedNull { identifier: identifier.to_string() })
                }
            }
        }
    };
}

/// Generates a `get_types_field` method corresponding to a `get_maybe_types_field` method,
/// unwrapping the optionals if they are all `Some` or erroring.
macro_rules! create_iter_plural_fields_method {
    ($variant:ident, $item:ty) => {
        paste! {
            #[doc = " Iterate over this item's non-null " $variant " field value for the given identifier."]
            ///
            /// # Errors
            /// - returns [`LdtkFieldsError::FieldNotFound`] if no field with the given identifier exists.
            #[doc = " - returns [`LdtkFieldsError::WrongFieldType`] if the field is not [`FieldValue::" $variant "`]."]
            /// - returns [`LdtkFieldsError::UnexpectedNull`] if **any** element of the field is null.
            fn [< iter_ $variant:snake _field >](&self, identifier: &str) -> Result<AllSomeIter<$item>, LdtkFieldsError> {
                let [< $variant:snake >]= self.[< get_maybe_ $variant:snake _field >](identifier)?;

                [< $variant:snake >].try_into().map_err(|_| LdtkFieldsError::UnexpectedNull { identifier: identifier.to_string() })
            }
        }
    };
}

/// Generates a `get_maybe_type_field` method for the given [FieldValue] variant,
/// accessing a field instance and unwrapping it to the given variant or erroring.
///
/// Intended only for variants whose internal type is optional.
macro_rules! create_get_maybe_field_method {
    ($variant:ident, $maybe_type:ty) => {
        paste! {
            create_base_get_field_method!("nullable ", [< maybe_ $variant:snake >], $variant, $maybe_type);
        }
    }
}

/// Generates a `get_type_field` method for the given [FieldValue] variant,
/// accessing a field instance and unwrapping it to the given variant or erroring.
///
/// Intended only for variants whose internal type is **not** optional.
macro_rules! create_just_get_field_method {
    ($variant:ident, $type:ty) => {
        paste! {
            create_base_get_field_method!("", [< $variant:snake >], $variant, &$type);
        }
    };
}

/// Generates both `get_maybe_type_field` and `get_type_field` methods for the given [FieldValue]
/// variant.
///
/// Intended only for variants whose internal type is optional.
macro_rules! create_get_field_methods {
    ($variant:ident, $type:ty) => {
        create_get_maybe_field_method!($variant, &Option<$type>);
        create_get_field_method!($variant, $type);
    };
}

/// Generates a `get_types_field` method for the given [FieldValue] variant,
/// accessing a field instance and unwrapping it to the given variant or erroring.
///
/// Intended only for variants whose internal type is a collection of a **non-optional** type.
macro_rules! create_just_get_plural_fields_method {
    ($variant:ident, $type:ty) => {
        paste! {
            create_base_get_field_method!("", [< $variant:snake >], $variant, &[$type]);
        }
    };
}

/// Generates both `get_maybe_types_field` and `get_types_field` methods for the given [FieldValue]
/// variant.
///
/// Intended only for variants whose internal type is a collection of an optional type.
macro_rules! create_plural_fields_methods {
    ($variant:ident, $type:ty) => {
        create_get_maybe_field_method!($variant, &[Option<$type>]);
        create_iter_plural_fields_method!($variant, $type);
    };
}

/// Convenience methods for accessing field instances.
pub trait LdtkFields {
    /// Immutable accessor for this item's field instances, by reference.
    fn field_instances(&self) -> &[FieldInstance];

    /// Get this item's field instance (with metadata) for the given identifier.
    ///
    /// # Errors
    /// - returns [`LdtkFieldsError::FieldNotFound`] if no field with the given identifier exists.
    fn get_field_instance(&self, identifier: &str) -> Result<&FieldInstance, LdtkFieldsError> {
        self.field_instances()
            .iter()
            .find(|f| f.identifier == identifier)
            .ok_or(LdtkFieldsError::FieldNotFound {
                identifier: identifier.to_string(),
            })
    }

    /// Get this item's field value for the given identifier.
    ///
    /// # Errors
    /// - returns [`LdtkFieldsError::FieldNotFound`] if no field with the given identifier exists.
    fn get_field(&self, identifier: &str) -> Result<&FieldValue, LdtkFieldsError> {
        Ok(&self.get_field_instance(identifier)?.value)
    }

    create_get_field_methods!(Int, i32);
    create_get_field_methods!(Float, f32);

    create_just_get_field_method!(Bool, bool);

    create_get_field_methods!(String, String);

    create_just_get_field_method!(Color, Color);

    create_get_field_methods!(FilePath, String);
    create_get_field_methods!(Enum, String);
    create_get_field_methods!(Tile, TilesetRectangle);
    create_get_field_methods!(EntityRef, ReferenceToAnEntityInstance);

    create_get_field_methods!(Point, IVec2);

    create_plural_fields_methods!(Ints, i32);
    create_plural_fields_methods!(Floats, f32);

    create_just_get_plural_fields_method!(Bools, bool);

    create_plural_fields_methods!(Strings, String);

    create_just_get_plural_fields_method!(Colors, Color);

    create_plural_fields_methods!(FilePaths, String);
    create_plural_fields_methods!(Enums, String);
    create_plural_fields_methods!(Tiles, TilesetRectangle);
    create_plural_fields_methods!(EntityRefs, ReferenceToAnEntityInstance);
    create_plural_fields_methods!(Points, IVec2);
}

impl LdtkFields for EntityInstance {
    fn field_instances(&self) -> &[FieldInstance] {
        &self.field_instances
    }
}

impl LdtkFields for Level {
    fn field_instances(&self) -> &[FieldInstance] {
        &self.field_instances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl LdtkFields for Vec<FieldInstance> {
        fn field_instances(&self) -> &[FieldInstance] {
            self
        }
    }

    fn field_instance_from_value(identifier: &str, value: FieldValue) -> FieldInstance {
        FieldInstance {
            identifier: identifier.to_string(),
            value,
            field_instance_type: "".to_string(),
            tile: None,
            def_uid: 0,
            real_editor_values: Vec::new(),
        }
    }

    fn sample_field_instances() -> Vec<FieldInstance> {
        use FieldValue::*;
        vec![
            field_instance_from_value("IntNone", Int(None)),
            field_instance_from_value("IntSome", Int(Some(0))),
            field_instance_from_value("FloatNone", Float(None)),
            field_instance_from_value("FloatSome", Float(Some(1.0))),
            field_instance_from_value("Bool", Bool(true)),
            field_instance_from_value("StringNone", String(None)),
            field_instance_from_value("StringSome", String(Some("two".to_string()))),
            field_instance_from_value("Color", Color(bevy::prelude::Color::BLACK)),
            field_instance_from_value("FilePathNone", FilePath(None)),
            field_instance_from_value("FilePathSome", FilePath(Some("three".to_string()))),
            field_instance_from_value("EnumNone", Enum(None)),
            field_instance_from_value("EnumSome", Enum(Some("Four".to_string()))),
            field_instance_from_value("TileNone", Tile(None)),
            field_instance_from_value("TileSome", Tile(Some(TilesetRectangle::default()))),
            field_instance_from_value("EntityRefNone", EntityRef(None)),
            field_instance_from_value(
                "EntityRefSome",
                EntityRef(Some(ReferenceToAnEntityInstance::default())),
            ),
            field_instance_from_value("PointNone", Point(None)),
            field_instance_from_value("PointSome", Point(Some(IVec2::default()))),
            field_instance_from_value("IntsNullable", Ints(vec![None, Some(5)])),
            field_instance_from_value("Ints", Ints(vec![Some(6), Some(7)])),
            field_instance_from_value("FloatsNullable", Floats(vec![None, Some(8.)])),
            field_instance_from_value("Floats", Floats(vec![Some(9.), Some(10.)])),
            field_instance_from_value("Bools", Bools(vec![false, true])),
            field_instance_from_value(
                "StringsNullable",
                Strings(vec![None, Some("eleven".to_string())]),
            ),
            field_instance_from_value(
                "Strings",
                Strings(vec![
                    Some("twelve".to_string()),
                    Some("thirteen".to_string()),
                ]),
            ),
            field_instance_from_value(
                "Colors",
                Colors(vec![
                    bevy::prelude::Color::BLACK,
                    bevy::prelude::Color::WHITE,
                ]),
            ),
            field_instance_from_value(
                "FilePathsNullable",
                FilePaths(vec![None, Some("fourteen".to_string())]),
            ),
            field_instance_from_value(
                "FilePaths",
                FilePaths(vec![
                    Some("fifteen".to_string()),
                    Some("sixteen".to_string()),
                ]),
            ),
            field_instance_from_value(
                "EnumsNullable",
                Enums(vec![None, Some("Seventeen".to_string())]),
            ),
            field_instance_from_value(
                "Enums",
                Enums(vec![
                    Some("Eighteen".to_string()),
                    Some("Nineteen".to_string()),
                ]),
            ),
            field_instance_from_value(
                "TilesNullable",
                Tiles(vec![None, Some(TilesetRectangle::default())]),
            ),
            field_instance_from_value(
                "Tiles",
                Tiles(vec![
                    Some(TilesetRectangle::default()),
                    Some(TilesetRectangle::default()),
                ]),
            ),
            field_instance_from_value(
                "EntityRefsNullable",
                EntityRefs(vec![None, Some(ReferenceToAnEntityInstance::default())]),
            ),
            field_instance_from_value(
                "EntityRefs",
                EntityRefs(vec![
                    Some(ReferenceToAnEntityInstance::default()),
                    Some(ReferenceToAnEntityInstance::default()),
                ]),
            ),
            field_instance_from_value("PointsNullable", Points(vec![None, Some(IVec2::default())])),
            field_instance_from_value(
                "Points",
                Points(vec![Some(IVec2::default()), Some(IVec2::default())]),
            ),
        ]
    }

    macro_rules! test_ambiguous_get_field_method {
        ($method_name:ident, $wrong_ident:literal, $( $ident:literal, $value:expr ),*) => {
            paste! {
                #[test]
                fn [< test_ $method_name >]() {
                    let field_instances = sample_field_instances();

                    assert!(matches!(
                        field_instances.$method_name("NonExistent"),
                        Err(LdtkFieldsError::FieldNotFound { .. })
                    ));
                    assert!(matches!(
                        field_instances.$method_name($wrong_ident),
                        Err(LdtkFieldsError::WrongFieldType { .. })
                    ));
                    $(
                        assert_eq!(
                            *field_instances.$method_name($ident).unwrap(),
                            $value
                        );
                    )*
                }
            }
        };
    }

    macro_rules! test_just_get_field_method {
        ($method_name:ident, $wrong_ident:literal, $nullable_ident:literal, $ident:literal, $value:expr) => {
            paste! {
                #[test]
                fn [< test_ $method_name >]() {
                    let field_instances = sample_field_instances();

                    assert!(matches!(
                        field_instances.$method_name("NonExistent"),
                        Err(LdtkFieldsError::FieldNotFound { .. })
                    ));
                    assert!(matches!(
                        field_instances.$method_name($wrong_ident),
                        Err(LdtkFieldsError::WrongFieldType { .. })
                    ));
                    assert!(matches!(
                        field_instances.$method_name($nullable_ident),
                        Err(LdtkFieldsError::UnexpectedNull { .. })
                    ));
                    assert_eq!(
                        *field_instances.$method_name($ident).unwrap(),
                        $value
                    );
                }
            }
        };
    }

    macro_rules! test_iter_fields_method {
        ($method_name:ident, $wrong_ident:literal, $nullable_ident:literal, $ident:literal, $value:expr) => {
            paste! {
                #[test]
                fn [< test_ $method_name >]() {
                    let field_instances = sample_field_instances();

                    assert!(matches!(
                        field_instances.$method_name("NonExistent"),
                        Err(LdtkFieldsError::FieldNotFound { .. })
                    ));
                    assert!(matches!(
                        field_instances.$method_name($wrong_ident),
                        Err(LdtkFieldsError::WrongFieldType { .. })
                    ));
                    assert!(matches!(
                        field_instances.$method_name($nullable_ident),
                        Err(LdtkFieldsError::UnexpectedNull { .. })
                    ));
                    assert_eq!(
                        field_instances
                            .$method_name($ident)
                            .unwrap()
                            .cloned()
                            .collect::<Vec<_>>(),
                        $value
                    );
                }
            }
        };
    }

    test_ambiguous_get_field_method!(
        get_maybe_int_field,
        "Bool",
        "IntNone",
        None,
        "IntSome",
        Some(0)
    );
    test_just_get_field_method!(get_int_field, "Bool", "IntNone", "IntSome", 0);

    test_ambiguous_get_field_method!(
        get_maybe_float_field,
        "Bool",
        "FloatNone",
        None,
        "FloatSome",
        Some(1.)
    );
    test_just_get_field_method!(get_float_field, "Bool", "FloatNone", "FloatSome", 1.);

    test_ambiguous_get_field_method!(get_bool_field, "Color", "Bool", true);

    test_ambiguous_get_field_method!(
        get_maybe_string_field,
        "Bool",
        "StringNone",
        None,
        "StringSome",
        Some("two".to_string())
    );
    test_just_get_field_method!(
        get_string_field,
        "Bool",
        "StringNone",
        "StringSome",
        "two".to_string()
    );

    test_ambiguous_get_field_method!(get_color_field, "Bool", "Color", Color::BLACK);

    test_ambiguous_get_field_method!(
        get_maybe_file_path_field,
        "Bool",
        "FilePathNone",
        None,
        "FilePathSome",
        Some("three".to_string())
    );
    test_just_get_field_method!(
        get_file_path_field,
        "Bool",
        "FilePathNone",
        "FilePathSome",
        "three".to_string()
    );

    test_ambiguous_get_field_method!(
        get_maybe_enum_field,
        "Bool",
        "EnumNone",
        None,
        "EnumSome",
        Some("Four".to_string())
    );
    test_just_get_field_method!(
        get_enum_field,
        "Bool",
        "EnumNone",
        "EnumSome",
        "Four".to_string()
    );

    test_ambiguous_get_field_method!(
        get_maybe_tile_field,
        "Bool",
        "TileNone",
        None,
        "TileSome",
        Some(TilesetRectangle::default())
    );
    test_just_get_field_method!(
        get_tile_field,
        "Bool",
        "TileNone",
        "TileSome",
        TilesetRectangle::default()
    );

    test_ambiguous_get_field_method!(
        get_maybe_entity_ref_field,
        "Bool",
        "EntityRefNone",
        None,
        "EntityRefSome",
        Some(ReferenceToAnEntityInstance::default())
    );
    test_just_get_field_method!(
        get_entity_ref_field,
        "Bool",
        "EntityRefNone",
        "EntityRefSome",
        ReferenceToAnEntityInstance::default()
    );

    test_ambiguous_get_field_method!(
        get_maybe_point_field,
        "Bool",
        "PointNone",
        None,
        "PointSome",
        Some(IVec2::default())
    );
    test_just_get_field_method!(
        get_point_field,
        "Bool",
        "PointNone",
        "PointSome",
        IVec2::default()
    );

    test_ambiguous_get_field_method!(
        get_maybe_ints_field,
        "Bools",
        "IntsNullable",
        [None, Some(5)],
        "Ints",
        [Some(6), Some(7)]
    );
    test_iter_fields_method!(iter_ints_field, "Bools", "IntsNullable", "Ints", [6, 7]);

    test_ambiguous_get_field_method!(
        get_maybe_floats_field,
        "Bools",
        "FloatsNullable",
        [None, Some(8.)],
        "Floats",
        [Some(9.), Some(10.)]
    );
    test_iter_fields_method!(
        iter_floats_field,
        "Bools",
        "FloatsNullable",
        "Floats",
        [9., 10.]
    );

    test_ambiguous_get_field_method!(get_bools_field, "Colors", "Bools", [false, true]);

    test_ambiguous_get_field_method!(
        get_maybe_strings_field,
        "Bools",
        "StringsNullable",
        [None, Some("eleven".to_string())],
        "Strings",
        [Some("twelve".to_string()), Some("thirteen".to_string())]
    );
    test_iter_fields_method!(
        iter_strings_field,
        "Bools",
        "StringsNullable",
        "Strings",
        ["twelve".to_string(), "thirteen".to_string()]
    );

    test_ambiguous_get_field_method!(
        get_colors_field,
        "Bools",
        "Colors",
        [Color::BLACK, Color::WHITE]
    );

    test_ambiguous_get_field_method!(
        get_maybe_file_paths_field,
        "Bools",
        "FilePathsNullable",
        [None, Some("fourteen".to_string())],
        "FilePaths",
        [Some("fifteen".to_string()), Some("sixteen".to_string())]
    );
    test_iter_fields_method!(
        iter_file_paths_field,
        "Bools",
        "FilePathsNullable",
        "FilePaths",
        ["fifteen".to_string(), "sixteen".to_string()]
    );

    test_ambiguous_get_field_method!(
        get_maybe_enums_field,
        "Bools",
        "EnumsNullable",
        [None, Some("Seventeen".to_string())],
        "Enums",
        [Some("Eighteen".to_string()), Some("Nineteen".to_string())]
    );
    test_iter_fields_method!(
        iter_enums_field,
        "Bools",
        "EnumsNullable",
        "Enums",
        ["Eighteen".to_string(), "Nineteen".to_string()]
    );

    test_ambiguous_get_field_method!(
        get_maybe_tiles_field,
        "Bools",
        "TilesNullable",
        [None, Some(TilesetRectangle::default())],
        "Tiles",
        [
            Some(TilesetRectangle::default()),
            Some(TilesetRectangle::default())
        ]
    );
    test_iter_fields_method!(
        iter_tiles_field,
        "Bools",
        "TilesNullable",
        "Tiles",
        [TilesetRectangle::default(), TilesetRectangle::default()]
    );

    test_ambiguous_get_field_method!(
        get_maybe_entity_refs_field,
        "Bools",
        "EntityRefsNullable",
        [None, Some(ReferenceToAnEntityInstance::default())],
        "EntityRefs",
        [
            Some(ReferenceToAnEntityInstance::default()),
            Some(ReferenceToAnEntityInstance::default())
        ]
    );
    test_iter_fields_method!(
        iter_entity_refs_field,
        "Bools",
        "EntityRefsNullable",
        "EntityRefs",
        [
            ReferenceToAnEntityInstance::default(),
            ReferenceToAnEntityInstance::default()
        ]
    );

    test_ambiguous_get_field_method!(
        get_maybe_points_field,
        "Bools",
        "PointsNullable",
        [None, Some(IVec2::default())],
        "Points",
        [Some(IVec2::default()), Some(IVec2::default())]
    );
    test_iter_fields_method!(
        iter_points_field,
        "Bools",
        "PointsNullable",
        "Points",
        [IVec2::default(), IVec2::default()]
    );
}
