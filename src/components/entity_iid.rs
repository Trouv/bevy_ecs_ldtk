use bevy::prelude::*;

use std::borrow::Cow;

/// [Component] added to all [LdtkEntity]s by default.
///
/// The `iid` stored in this component can be used to uniquely identify LDtk entities within an [LdtkAsset].
#[derive(Clone, Debug, Default, Deref, Hash, Eq, PartialEq, Component, FromReflect, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct EntityIid(Cow<'static, str>);

impl EntityIid {
    /// Construct an [`EntityIid`].
    pub fn new(iid: impl Into<Cow<'static, str>>) -> Self {
        let iid = iid.into();
        EntityIid(iid)
    }

    /// Returns the internal entity iid as a `&str`.
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for EntityIid {
    #[inline(always)]
    fn from(value: &str) -> Self {
        EntityIid::new(value.to_owned())
    }
}

impl From<String> for EntityIid {
    #[inline(always)]
    fn from(value: String) -> Self {
        EntityIid::new(value)
    }
}

impl AsRef<str> for EntityIid {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&EntityIid> for String {
    #[inline(always)]
    fn from(value: &EntityIid) -> String {
        value.as_str().to_owned()
    }
}

impl From<EntityIid> for String {
    #[inline(always)]
    fn from(value: EntityIid) -> String {
        value.0.into_owned()
    }
}
