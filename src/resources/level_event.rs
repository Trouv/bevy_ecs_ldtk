use bevy::prelude::*;

use crate::LevelIid;

/// Events fired by the plugin related to level spawning/despawning.
///
/// Each variant stores the level's `iid` in LDtk.
#[derive(Clone, Eq, PartialEq, Debug, Hash, Message)]
pub enum LevelEvent {
    /// Indicates that a level has been triggered to spawn, but hasn't been spawned yet.
    SpawnTriggered(LevelIid),
    /// The level, with all of its layers, entities, etc., has spawned.
    ///
    /// Note: due to the frame-delay of [`GlobalTransform`] being updated, this may not be the
    /// event you want to listen for.
    /// If your systems are [`GlobalTransform`]-dependent, see [`LevelEvent::Transformed`].
    ///
    /// [`GlobalTransform`]: https://docs.rs/bevy/latest/bevy/prelude/struct.GlobalTransform.html
    Spawned(LevelIid),
    /// Occurs during the [`PostUpdate`] after the level has spawned, so all [`GlobalTransform`]s
    /// of the level should be updated.
    ///
    /// [`PostUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PostUpdate.html
    /// [`GlobalTransform`]: https://docs.rs/bevy/latest/bevy/prelude/struct.GlobalTransform.html
    Transformed(LevelIid),
    /// Indicates that a level has despawned.
    Despawned(LevelIid),
}
