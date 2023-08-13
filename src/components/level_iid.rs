use std::fmt::Display;

use bevy::prelude::*;

/// [`Component`] that stores a level's instance identifier.
///
/// [`Component`]: https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Component, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct LevelIid(String);

impl LevelIid {
    /// Creates a new [`LevelIid`] from any string-like type.
    pub fn new(iid: impl Into<String>) -> Self {
        let iid = iid.into();
        LevelIid(iid)
    }

    /// Immutable access to the IID as a `String`.
    pub fn get(&self) -> &String {
        &self.0
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

impl Display for LevelIid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_converts_to_and_from_level_iid() {
        let original_string = "level-iid".to_string();
        let level_iid = LevelIid::new(original_string.clone());

        assert_eq!(level_iid, LevelIid(original_string.clone()));
        assert_eq!(level_iid.get(), &original_string);
        assert_eq!(level_iid.as_str(), original_string.as_str());
        assert_eq!(LevelIid::from(original_string.clone()), level_iid);
        assert_eq!(String::from(level_iid.clone()), original_string);
        assert_eq!(level_iid.as_ref(), original_string.as_str());
        assert_eq!(
            format!("display: {level_iid}"),
            format!("display: {original_string}")
        );
    }
}
