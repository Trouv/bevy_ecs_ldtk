use crate::ldtk::{FieldInstance, FieldInstanceEntityReference, FieldValue, TilesetRectangle};
use bevy::prelude::*;
use paste::paste;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LdtkFieldsError {
    #[error("could not find {identifier} field")]
    FieldNotFound { identifier: String },
    #[error("found {identifier} field, but its type is not correct")]
    WrongFieldType { identifier: String },
    #[error("found {identifier} field of the correct type, but it is null")]
    UnexpectedNull { identifier: String },
}

macro_rules! create_get_ambiguous_field_method {
    ($adjective:literal, $doc_name:ident, $var_name:ident, $variant:ident, $return_type:ty, $return_expr:expr) => {
        paste! {
            #[doc = " Get this item's " $adjective $doc_name " field value for the given identifier."]
            ///
            /// # Errors
            /// - returns [LdtkFieldsError::FieldNotFound] if no field with the given identifier exists.
            #[doc = " - returns [LdtkFieldsError::WrongFieldType] if the field is not " $variant "."]
            fn [< get_ $var_name _field >](
                &self,
                identifier: &str,
            ) -> Result<$return_type, LdtkFieldsError> {
                match self.get_field(identifier)? {
                    FieldValue::$variant($var_name) => Ok($return_expr),
                    _ => Err(LdtkFieldsError::WrongFieldType {
                        identifier: identifier.to_string(),
                    }),
                }
            }
        }
    }
}

macro_rules! create_get_maybe_field_method {
    ($type_name:ident, $variant:ident, $maybe_type:ty) => {
        paste! {
            create_get_ambiguous_field_method!("nullable ", $type_name, [< maybe_ $type_name >], $variant, $maybe_type, [< maybe_ $type_name >]);
        }
    }
}

macro_rules! create_get_maybe_field_method_copy {
    ($type_name:ident, $variant:ident, $maybe_type:ty) => {
        paste! {
            create_get_ambiguous_field_method!("nullable ", $type_name, [< maybe_ $type_name >], $variant, $maybe_type, *[< maybe_ $type_name >]);
        }
    }
}

macro_rules! create_just_get_field_method_copy {
    ($type_name:ident, $variant:ident, $type:ty) => {
        paste! {
            create_get_ambiguous_field_method!("", $type_name, $type_name, $variant, $type, *$type_name);
        }
    };
}

macro_rules! create_get_field_method {
    ($type_name:ident, $variant:ident, $type:ty) => {
        paste! {
            #[doc = " Get this item's non-null " $type_name " field value for the given identifier."]
            ///
            /// # Errors
            /// - returns [LdtkFieldsError::FieldNotFound] if no field with the given identifier exists.
            #[doc = " - returns [LdtkFieldsError::WrongFieldType] if the field is not " $variant "."]
            /// - returns [LdtkFieldsError::UnexpectedNull] if the field is null.
            fn [< get_ $type_name _field >](&self, identifier: &str) -> Result<$type, LdtkFieldsError> {
                if let Some($type_name) = self.[< get_maybe_ $type_name _field >](identifier.clone())? {
                    Ok($type_name)
                } else {
                    Err(LdtkFieldsError::UnexpectedNull { identifier: identifier.to_string() })
                }
            }
        }
    };
}

macro_rules! create_get_field_methods_copy {
    ($type_name:ident, $variant:ident, $type:ty) => {
        create_get_maybe_field_method_copy!($type_name, $variant, Option<$type>);
        create_get_field_method!($type_name, $variant, $type);
    };
}

macro_rules! create_get_field_methods {
    ($type_name:ident, $variant:ident, $maybe_type:ty, $as_ref_type: ty) => {
        create_get_maybe_field_method!($type_name, $variant, $maybe_type);
        create_get_field_method!($type_name, $variant, $as_ref_type);
    };
}

macro_rules! create_get_plural_fields_method {
    ($type_name:ident, $variant:ident, $collected_type:ty) => {
        paste! {
            #[doc = " Get this item's non-null " $type_name " field value for the given identifier."]
            ///
            /// # Errors
            /// - returns [LdtkFieldsError::FieldNotFound] if no field with the given identifier exists.
            #[doc = " - returns [LdtkFieldsError::WrongFieldType] if the field is not " $variant "."]
            /// - returns [LdtkFieldsError::UnexpectedNull] if **any** element of the field is null.
            fn [< get_ $type_name _field >](&self, identifier: &str) -> Result<$collected_type, LdtkFieldsError> {
                let $type_name = self.[< get_maybe_ $type_name _field >](identifier)?;

                if $type_name.iter().all(|e| e.is_some()) {
                    Ok($type_name.iter().flatten().collect())
                } else {
                    Err(LdtkFieldsError::UnexpectedNull {
                        identifier: identifier.to_string(),
                    })
                }
            }
        }
    };
}

macro_rules! create_get_plural_fields_methods {
    ($type_name:ident, $variant:ident, $maybe_type:ty, $as_ref_type: ty) => {
        create_get_maybe_field_method!($type_name, $variant, &[$maybe_type]);
        create_get_plural_fields_method!($type_name, $variant, Vec<$as_ref_type>);
    };
}

pub trait LdtkFields {
    /// Immutable accessor for this item's field instances, by reference.
    fn field_instances(&self) -> &[FieldInstance];

    /// Get this item's field instance (with metadata) for given identifier.
    ///
    /// # Errors
    /// - returns [LdtkFieldsError::FieldNotFound] if no field with the given identifier exists.
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
    /// - returns [LdtkFieldsError::FieldNotFound] if no field with the given identifier exists.
    fn get_field(&self, identifier: &str) -> Result<&FieldValue, LdtkFieldsError> {
        Ok(&self.get_field_instance(identifier)?.value)
    }

    create_get_field_methods_copy!(int, Int, i32);
    create_get_field_methods_copy!(float, Float, f32);

    create_just_get_field_method_copy!(bool, Bool, bool);

    create_get_field_methods!(string, String, &Option<String>, &str);

    create_just_get_field_method_copy!(color, Color, Color);

    create_get_field_methods!(file_path, FilePath, &Option<String>, &str);
    create_get_field_methods!(tile, Tile, &Option<TilesetRectangle>, &TilesetRectangle);
    create_get_field_methods!(
        entity_ref,
        EntityRef,
        &Option<FieldInstanceEntityReference>,
        &FieldInstanceEntityReference
    );

    create_get_field_methods_copy!(point, Point, IVec2);

    create_get_plural_fields_methods!(ints, Ints, Option<i32>, &i32);

    // implement similar methods for all `FieldValue` variants...
}
