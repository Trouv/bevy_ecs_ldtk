use bevy::prelude::*;

/// `Component` that stores a level's instance identifier.
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Component, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct LevelIid(String);

impl LevelIid {
    /// Creates a new [`LevelIid`] from any string-like type.
    pub fn new(iid: impl Into<String>) -> Self {
        let iid = iid.into();
        LevelIid(iid)
    }

    /// Immutable access to the IID as a `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for LevelIid {
    fn from(value: String) -> Self {
        LevelIid::new(value)
    }
}

impl From<LevelIid> for String {
    fn from(value: LevelIid) -> String {
        value.0
    }
}

impl AsRef<str> for LevelIid {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
