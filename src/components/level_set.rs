use bevy::prelude::*;
use std::collections::HashSet;

use crate::LevelIid;

/// [`Component`] that determines the desired levels to be spawned in an [`LdtkWorldBundle`].
///
/// For more explanation and comparison of options for selecting levels to spawn, see the
/// [*Level Selection*](https://trouv.github.io/bevy_ecs_ldtk/v0.10.0/explanation/level-selection.html) <!-- x-release-please-version -->
/// chapter of the `bevy_ecs_ldtk` book.
///
/// [`Component`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.Component.html
/// [`LdtkWorldBundle`]: crate::prelude::LdtkWorldBundle
#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct LevelSet {
    pub iids: HashSet<LevelIid>,
}

impl LevelSet {
    /// Construct a new [`LevelSet`] from a collection of iids.
    ///
    /// These iids simply need to implement `Into<String>`.
    ///
    /// # Example
    /// ```
    /// use bevy_ecs_ldtk::prelude::*;
    ///
    /// let iids = [
    ///     "fa26aa50-fd0f-4dac-a076-3edfb0afd358",
    ///     "9ae9ecf0-ef64-4d96-bc68-cead527efe90",
    ///     "57b26336-8f4e-41ee-8a1b-7af708e4a338",
    /// ];
    ///
    /// let level_set = LevelSet::from_iids(iids);
    /// # let mut iids_as_set = std::collections::HashSet::new();
    /// # iids_as_set.insert(LevelIid::new("fa26aa50-fd0f-4dac-a076-3edfb0afd358"));
    /// # iids_as_set.insert(LevelIid::new("9ae9ecf0-ef64-4d96-bc68-cead527efe90"));
    /// # iids_as_set.insert(LevelIid::new("57b26336-8f4e-41ee-8a1b-7af708e4a338"));
    /// # assert_eq!(level_set, LevelSet { iids: iids_as_set });
    /// ```
    pub fn from_iids<I: Into<String>>(iids: impl IntoIterator<Item = I>) -> Self {
        iids.into_iter().map(LevelIid::new).collect()
    }
}

impl IntoIterator for LevelSet {
    type Item = LevelIid;
    type IntoIter = <HashSet<LevelIid> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iids.into_iter()
    }
}

impl FromIterator<LevelIid> for LevelSet {
    fn from_iter<T: IntoIterator<Item = LevelIid>>(iter: T) -> Self {
        let iids = iter.into_iter().collect();
        LevelSet { iids }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_set_converts_to_and_from_iterator() {
        let iids = [
            "fa26aa50-fd0f-4dac-a076-3edfb0afd358",
            "9ae9ecf0-ef64-4d96-bc68-cead527efe90",
            "57b26336-8f4e-41ee-8a1b-7af708e4a338",
        ]
        .into_iter()
        .map(LevelIid::new)
        .collect::<HashSet<_>>();

        let level_set = LevelSet::from_iter(iids.clone());

        assert_eq!(level_set.into_iter().collect::<HashSet<_>>(), iids);
    }
}
