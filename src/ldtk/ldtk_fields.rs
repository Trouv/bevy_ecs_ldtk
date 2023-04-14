//! Contains [`LdtkFields`] trait, providing convenience methods for accessing field instances.
use crate::ldtk::{
    EntityInstance, FieldInstance, FieldInstanceEntityReference, FieldValue, Level,
    TilesetRectangle,
};
use bevy::prelude::*;
use paste::paste;
use std::{iter::Flatten, slice::Iter};
use thiserror::Error;

pub struct NotAllSomeError;

pub struct AllSomeIter<'a, T> {
    flattened: Flatten<Iter<'a, Option<T>>>,
}

impl<'a, T> Iterator for AllSomeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.flattened.next()
    }
}

impl<'a, T> TryFrom<&'a [Option<T>]> for AllSomeIter<'a, T> {
    type Error = NotAllSomeError;

    fn try_from(value: &'a [Option<T>]) -> Result<Self, Self::Error> {
        if value.iter().all(|v| v.is_some()) {
            Ok(AllSomeIter {
                flattened: value.iter().flatten(),
            })
        } else {
            Err(NotAllSomeError)
        }
    }
}

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
macro_rules! create_get_plural_fields_method {
    ($variant:ident, $item:ty) => {
        paste! {
            #[doc = " Get this item's non-null " $variant " field value for the given identifier."]
            ///
            /// # Errors
            /// - returns [`LdtkFieldsError::FieldNotFound`] if no field with the given identifier exists.
            #[doc = " - returns [`LdtkFieldsError::WrongFieldType`] if the field is not [`FieldValue::" $variant "`]."]
            /// - returns [`LdtkFieldsError::UnexpectedNull`] if **any** element of the field is null.
            fn [< get_ $variant:snake _field >](&self, identifier: &str) -> Result<AllSomeIter<$item>, LdtkFieldsError> {
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
/// accessing a field instance and unwrapping it to the given variant or erroring,
/// and returning a copy to it instead of a reference.
///
/// Intended only for variants whose internal type is **not** optional and can be cheaply copied.
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
macro_rules! create_get_plural_fields_methods {
    ($variant:ident, $type:ty) => {
        create_get_maybe_field_method!($variant, &[Option<$type>]);
        create_get_plural_fields_method!($variant, $type);
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
    create_get_field_methods!(EntityRef, FieldInstanceEntityReference);

    create_get_field_methods!(Point, IVec2);

    create_get_plural_fields_methods!(Ints, i32);
    create_get_plural_fields_methods!(Floats, f32);

    create_just_get_plural_fields_method!(Bools, bool);

    create_get_plural_fields_methods!(Strings, String);

    create_just_get_plural_fields_method!(Colors, Color);

    create_get_plural_fields_methods!(FilePaths, String);
    create_get_plural_fields_methods!(Enums, String);
    create_get_plural_fields_methods!(Tiles, TilesetRectangle);
    create_get_plural_fields_methods!(EntityRefs, FieldInstanceEntityReference);
    create_get_plural_fields_methods!(Points, IVec2);
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
            &self
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
                EntityRef(Some(FieldInstanceEntityReference::default())),
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
                EntityRefs(vec![None, Some(FieldInstanceEntityReference::default())]),
            ),
            field_instance_from_value(
                "EntityRefs",
                EntityRefs(vec![
                    Some(FieldInstanceEntityReference::default()),
                    Some(FieldInstanceEntityReference::default()),
                ]),
            ),
            field_instance_from_value("PointsNullable", Points(vec![None, Some(IVec2::default())])),
            field_instance_from_value(
                "Points",
                Points(vec![Some(IVec2::default()), Some(IVec2::default())]),
            ),
        ]
    }

    macro_rules! test_get_field_methods {
        ($type_name:ident, $maybe_ident:literal, $just_ident:literal, $wrong_ident:literal, $expected_maybe:expr, $expected_just:expr) => {
            paste! {
                #[test]
                fn [< test_get_ $type_name _field_methods >]() {
                    let field_instances = sample_field_instances();

                    assert!(matches!(
                        field_instances.[< get_maybe_ $type_name _field >]("NonExistent"),
                        Err(LdtkFieldsError::FieldNotFound { .. })
                    ));
                    assert!(matches!(
                        field_instances.[< get_maybe_ $type_name _field >]($wrong_ident),
                        Err(LdtkFieldsError::WrongFieldType { .. })
                    ));
                    assert_eq!(
                        field_instances.[< get_maybe_ $type_name _field >]($maybe_ident).unwrap(),
                        None
                    );
                    assert_eq!(
                        field_instances.[< get_maybe_ $type_name _field >]($just_ident).unwrap(),
                        $expected_maybe
                    );

                    assert!(matches!(
                        field_instances.[< get_ $type_name _field >]("NonExistent"),
                        Err(LdtkFieldsError::FieldNotFound { .. })
                    ));
                    assert!(matches!(
                        field_instances.[< get_ $type_name _field >]($wrong_ident),
                        Err(LdtkFieldsError::WrongFieldType { .. })
                    ));
                    assert!(matches!(
                        field_instances.[< get_ $type_name _field >]($maybe_ident),
                        Err(LdtkFieldsError::UnexpectedNull { .. })
                    ));
                    assert_eq!(field_instances.[< get_ $type_name _field >]($just_ident).unwrap(), $expected_just);
                }
            }
        };
    }

    macro_rules! test_just_get_field_method {
        ($type_name:ident, $ident:literal, $wrong_ident:literal, $expected:expr)  => {
            paste! {
                #[test]
                fn [< test_get_ $type_name _field_methods >]() {
                    let field_instances = sample_field_instances();

                    assert!(matches!(
                        field_instances.[< get_ $type_name _field >]("NonExistent"),
                        Err(LdtkFieldsError::FieldNotFound { .. })
                    ));
                    assert!(matches!(
                        field_instances.[< get_ $type_name _field >]($wrong_ident),
                        Err(LdtkFieldsError::WrongFieldType { .. })
                    ));
                    assert_eq!(field_instances.[< get_ $type_name _field >]($ident).unwrap(), $expected);
                }
            }
        };
    }

    test_get_field_methods!(int, "IntNone", "IntSome", "Bool", Some(0), 0);
    test_get_field_methods!(float, "FloatNone", "FloatSome", "Bool", Some(1.0), 1.0);

    test_just_get_field_method!(bool, "Bool", "Color", true);
}
