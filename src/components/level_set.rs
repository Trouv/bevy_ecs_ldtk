use bevy::prelude::*;
use std::collections::HashSet;

use crate::LevelIid;

/// [`Component`] that determines the desired levels to be loaded for an [`LdtkWorldBundle`].
///
/// There is an abstraction for this in the form of the [`LevelSelection`] resource.
/// This component does not respond to the [`load_level_neighbors`] option at all, while the
/// [`LevelSelection`] does.
/// If a [`LevelSelection`] is inserted, the plugin will update this component based off its value.
/// If not, [`LevelSet`] allows you to have more direct control over the levels you spawn.
///
/// Changes to this component are idempotent, so levels won't be respawned greedily.
///
/// [`LevelSelection`]: crate::prelude::LevelSelection
/// [`load_level_neighbors`]:
/// crate::prelude::LevelSpawnBehavior::UseWorldTranslation::load_level_neighbors
/// [`LdtkWorldBundle`]: crate::prelude::LdtkWorldBundle
/// [`Component`]: https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html
#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct LevelSet {
    pub iids: HashSet<LevelIid>,
}

impl LevelSet {
    pub fn from_iid<T: Into<String>>(iid: T) -> Self {
        let mut iids = HashSet::default();
        iids.insert(LevelIid::new(iid));
        Self { iids }
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
