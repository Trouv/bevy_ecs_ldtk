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

macro_rules! create_get_ambiguous_field_method_copy {
    ($adjective:literal, $doc_name:ident, $type_name:ident, $variant:ident, $type:ty) => {
        paste! {
            #[doc = " Get this item's " $adjective $doc_name " field value for the given identifier."]
            ///
            /// # Errors
            /// - returns [LdtkFieldsError::FieldNotFound] if no field with the given identifier exists.
            #[doc = " - returns [LdtkFieldsError::WrongFieldType] if the field is not " $variant "."]
            fn [< get_ $type_name _field >](
                &self,
                identifier: String,
            ) -> Result<$type, LdtkFieldsError> {
                match self.get_field(identifier.clone())? {
                    FieldValue::$variant($type_name) => Ok(*$type_name),
                    _ => Err(LdtkFieldsError::WrongFieldType {
                        identifier,
                    }),
                }
            }
        }
    }
}

macro_rules! create_get_maybe_field_method {
    ($type_name:ident, $variant:ident, $maybe_type:ty) => {
        paste! {
            #[doc = " Get this item's nullable " $type_name " field value for the given identifier."]
            ///
            /// # Errors
            /// - returns [LdtkFieldsError::FieldNotFound] if no field with the given identifier exists.
            #[doc = " - returns [LdtkFieldsError::WrongFieldType] if the field is not " $variant "."]
            fn [< get_maybe_ $type_name _field >](
                &self,
                identifier: String,
            ) -> Result<$maybe_type, LdtkFieldsError> {
                match self.get_field(identifier.clone())? {
                    FieldValue::$variant($type_name) => Ok($type_name),
                    _ => Err(LdtkFieldsError::WrongFieldType {
                        identifier,
                    }),
                }
            }
        }
    }
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
            fn [< get_ $type_name _field >](&self, identifier: String) -> Result<$type, LdtkFieldsError> {
                if let Some($type_name) = self.[< get_maybe_ $type_name _field >](identifier.clone())? {
                    Ok($type_name)
                } else {
                    Err(LdtkFieldsError::UnexpectedNull { identifier })
                }
            }
        }
    };
}

macro_rules! create_get_field_methods_copy {
    ($type_name:ident, $variant:ident, $type:ty) => {
        paste! {
            create_get_ambiguous_field_method_copy!("nullable ", $type_name, [< maybe_ $type_name >], $variant, Option<$type>);
        }
        create_get_field_method!($type_name, $variant, $type);
    };
}

macro_rules! create_get_field_methods {
    ($type_name:ident, $variant:ident, $maybe_type:ty, $type: ty) => {
        create_get_maybe_field_method!($type_name, $variant, $maybe_type);
        create_get_field_method!($type_name, $variant, $type);
    };
}

pub trait LdtkFields {
    /// Immutable accessor for this item's field instances, by reference.
    fn field_instances(&self) -> &[FieldInstance];

    /// Get this item's field instance (with metadata) for given identifier.
    ///
    /// # Errors
    /// - returns [LdtkFieldsError::FieldNotFound] if no field with the given identifier exists.
    fn get_field_instance(&self, identifier: String) -> Result<&FieldInstance, LdtkFieldsError> {
        self.field_instances()
            .iter()
            .find(|f| f.identifier == identifier)
            .ok_or(LdtkFieldsError::FieldNotFound { identifier })
    }

    /// Get this item's field value for the given identifier.
    ///
    /// # Errors
    /// - returns [LdtkFieldsError::FieldNotFound] if no field with the given identifier exists.
    fn get_field(&self, identifier: String) -> Result<&FieldValue, LdtkFieldsError> {
        Ok(&self.get_field_instance(identifier)?.value)
    }

    create_get_field_methods_copy!(int, Int, i32);
    create_get_field_methods_copy!(float, Float, f32);

    create_get_ambiguous_field_method_copy!("", bool, bool, Bool, bool);

    create_get_field_methods!(string, String, &Option<String>, &str);

    create_get_ambiguous_field_method_copy!("", color, color, Color, Color);

    create_get_field_methods!(file_path, FilePath, &Option<String>, &str);
    create_get_field_methods!(tile, Tile, &Option<TilesetRectangle>, &TilesetRectangle);
    create_get_field_methods!(
        entity_ref,
        EntityRef,
        &Option<FieldInstanceEntityReference>,
        &FieldInstanceEntityReference
    );

    create_get_field_methods_copy!(point, Point, IVec2);
    // implement similar methods for all `FieldValue` variants...
}
