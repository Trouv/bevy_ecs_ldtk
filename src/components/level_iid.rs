use bevy::prelude::*;

use std::borrow::Cow;

#[derive(Clone, Debug, Default, Deref, Hash, Eq, PartialEq, Component, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct LevelIid(Cow<'static, str>);

impl LevelIid {
    /// Creates a new [`LevelIid`] from any string-like type.
    pub fn new(iid: impl Into<Cow<'static, str>>) -> Self {
        let iid = iid.into();
        LevelIid(iid)
    }

    /// Gets the Iid of the entity as a &str.
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for LevelIid {
    #[inline(always)]
    fn from(value: &str) -> Self {
        LevelIid::new(value.to_owned())
    }
}

impl From<String> for LevelIid {
    #[inline(always)]
    fn from(value: String) -> Self {
        LevelIid::new(value)
    }
}

impl AsRef<str> for LevelIid {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&LevelIid> for String {
    #[inline(always)]
    fn from(value: &LevelIid) -> String {
        value.as_str().to_owned()
    }
}

impl From<LevelIid> for String {
    #[inline(always)]
    fn from(value: LevelIid) -> String {
        value.0.into_owned()
    }
}
