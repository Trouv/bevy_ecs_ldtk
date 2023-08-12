use bevy::prelude::*;
use std::collections::HashSet;

/// [Component] that determines the desired levels to be loaded for an [LdtkWorldBundle].
///
/// There is an abstraction for this in the form of the [LevelSelection] resource.
/// This component does not respond to the
/// [LevelSpawnBehavior::UseWorldTranslation::load_level_neighbors] option at all, while the
/// [LevelSelection] does.
/// If a [LevelSelection] is inserted, the plugin will update this component based off its value.
/// If not, [LevelSet] allows you to have more direct control over the levels you spawn.
///
/// Changes to this component are idempotent, so levels won't be respawned greedily.
#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct LevelSet {
    pub iids: HashSet<String>,
}

impl LevelSet {
    pub fn from_iid<T: Into<String>>(iid: T) -> Self {
        let mut iids = HashSet::default();
        iids.insert(iid.into());
        Self { iids }
    }
}
