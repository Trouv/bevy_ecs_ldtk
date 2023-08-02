use crate::ldtk::Level;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("loaded levels must have non-null layer instances")]
pub struct LevelNotLoaded;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LoadedLevel<'a> {
    level: &'a Level,
}

impl<'a> TryFrom<&'a Level> for LoadedLevel<'a> {
    type Error = LevelNotLoaded;

    fn try_from(level: &Level) -> Result<Self, Self::Error> {
        if level.layer_instances.is_some() {
            Ok(LoadedLevel { level })
        } else {
            Err(LevelNotLoaded)
        }
    }
}

impl<'a> LoadedLevel<'a> {
    pub fn level(&self) -> &Level {
        self.level
    }
}
