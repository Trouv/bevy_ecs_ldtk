use crate::ldtk::{FieldInstance, FieldValue};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LdtkFieldsError {
    #[error("could not find {identifier} field")]
    FieldNotFound { identifier: String },
    #[error("found {identifier} field, but its type is not {requested_type}")]
    WrongFieldType {
        identifier: String,
        requested_type: String,
    },
    #[error("found {identifier} field of the correct type, but it is null")]
    UnexpectedNull { identifier: String },
}

pub trait LdtkFields {
    fn field_instances(&self) -> &[FieldInstance];

    fn get_field_instance(&self, identifier: String) -> Result<&FieldInstance, LdtkFieldsError> {
        self.field_instances()
            .iter()
            .find(|f| f.identifier == identifier)
            .ok_or(LdtkFieldsError::FieldNotFound { identifier })
    }

    fn get_field(&self, identifier: String) -> Result<&FieldValue, LdtkFieldsError> {
        Ok(&self.get_field_instance(identifier)?.value)
    }

    fn get_maybe_int_field(&self, identifier: String) -> Result<Option<i32>, LdtkFieldsError> {
        match self.get_field(identifier.clone())? {
            FieldValue::Int(maybe_int) => Ok(*maybe_int),
            _ => Err(LdtkFieldsError::WrongFieldType {
                identifier,
                requested_type: "Int".to_string(),
            }),
        }
    }

    fn get_int_field(&self, identifier: String) -> Result<i32, LdtkFieldsError> {
        if let Some(int) = self.get_maybe_int_field(identifier.clone())? {
            Ok(int)
        } else {
            Err(LdtkFieldsError::UnexpectedNull { identifier })
        }
    }

    // implement similar methods for all `FieldValue` variants...
}
