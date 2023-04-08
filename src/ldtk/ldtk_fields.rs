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

macro_rules! create_get_field_methods_copy {
    ($type_name:ident, $variant:ident, $type:ty) => {
        paste! {
            #[doc = " Get this item's nullable " $type_name " field value for the given identifier."]
            ///
            /// # Errors
            /// - returns [LdtkFieldsError::FieldNotFound] if no field with the given identifier exists.
            #[doc = " - returns [LdtkFieldsError::WrongFieldType] if the field is not " $variant "."]
            fn [< get_maybe_ $type_name _field >](
                &self,
                identifier: String,
            ) -> Result<Option<$type>, LdtkFieldsError> {
                match self.get_field(identifier.clone())? {
                    FieldValue::$variant($type_name) => Ok(*$type_name),
                    _ => Err(LdtkFieldsError::WrongFieldType {
                        identifier,
                    }),
                }
            }

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

macro_rules! create_get_field_methods_as_ref {
    ($type_name:ident, $variant:ident, $maybe_type:ty, $type: ty) => {
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

    /// Get this item's non-null bool field value for the given identifier.
    ///
    /// # Errors
    /// - returns [LdtkFieldsError::FieldNotFound] if no field with the given identifier exists.
    /// - returns [LdtkFieldsError::WrongFieldType] if the field is not Bool.
    fn get_bool_field(&self, identifier: String) -> Result<bool, LdtkFieldsError> {
        if let FieldValue::Bool(boolean) = self.get_field(identifier.clone())? {
            Ok(*boolean)
        } else {
            Err(LdtkFieldsError::WrongFieldType { identifier })
        }
    }

    create_get_field_methods_as_ref!(string, String, &Option<String>, &str);
    create_get_field_methods_as_ref!(file_path, FilePath, &Option<String>, &str);
    create_get_field_methods_as_ref!(tile, Tile, &Option<TilesetRectangle>, &TilesetRectangle);
    create_get_field_methods_as_ref!(
        entity_ref,
        EntityRef,
        &Option<FieldInstanceEntityReference>,
        &FieldInstanceEntityReference
    );

    create_get_field_methods_copy!(point, Point, IVec2);
    // implement similar methods for all `FieldValue` variants...
}
